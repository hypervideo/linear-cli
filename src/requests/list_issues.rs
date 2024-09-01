use eyre::Result;
use graphql_client::GraphQLQuery;

use crate::client::Client;

#[derive(Clone, clap::ValueEnum)]
pub enum SortBy {
    #[clap(name = "created")]
    CreatedAt,
    #[clap(name = "updated")]
    UpdatedAt,
}

#[derive(Clone, clap::ValueEnum, strum::EnumIter, PartialEq, Eq, Hash)]
pub enum IssueState {
    Started,
    Unstarted,
    Backlog,
    Completed,
    Canceled,
}

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "graphql/list-issues.graphql",
    schema_path = "graphql/linear-api.graphql",
    response_derives = "Debug, Clone, Serialize, Deserialize"
)]
struct ListIssues;

pub type DateTime = chrono::DateTime<chrono::Utc>;
pub type Issue = list_issues::Issue;

#[builder]
pub async fn request(
    client: &Client,
    n: Option<usize>,
    sort_by: SortBy,
    assignee: Option<String>,
    state: Option<Vec<IssueState>>,
) -> Result<Vec<Issue>> {
    const PER_PAGE: usize = 100;
    let mut i = 0;
    let mut after = None;
    let mut result = Vec::new();

    let order_by = match sort_by {
        SortBy::CreatedAt => list_issues::PaginationOrderBy::createdAt,
        SortBy::UpdatedAt => list_issues::PaginationOrderBy::updatedAt,
    };

    loop {
        debug!(page = %i, %PER_PAGE, "list_issues");

        let query = ListIssues::build_query(list_issues::Variables {
            first: Some(PER_PAGE as _),
            order_by: Some(order_by.clone()),
            after,
            before: None,
            include_archived: None,
            last: None,
        });

        let response = client
            .req::<_, list_issues::ResponseData>(query)
            .await
            .map(|r| r.issues)?;

        result.extend(
            response
                .edges
                .into_iter()
                .map(|e| e.node)
                .filter(|i| {
                    if let Some(assignee) = &assignee {
                        i.assignee.as_ref().map(|a| a.display_name.as_str()) == Some(assignee)
                    } else {
                        true
                    }
                })
                .filter(|i| {
                    let state_t = i.state.type_.as_str();
                    if let Some(state) = &state {
                        state.iter().any(|s| match s {
                            IssueState::Started => state_t == "started",
                            IssueState::Unstarted => state_t == "unstarted",
                            IssueState::Backlog => state_t == "backlog",
                            IssueState::Completed => state_t == "completed",
                            IssueState::Canceled => state_t == "canceled",
                        })
                    } else {
                        true
                    }
                }),
        );

        if !response.page_info.has_next_page {
            break Ok(result);
        }

        if let Some(n) = n {
            if result.len() >= n {
                break Ok(result);
            }
        }

        i += 1;
        after = response.page_info.end_cursor;
    }
}

fn fmt_date(date: DateTime) -> String {
    let date = date.with_timezone(&chrono::Local);
    date.format("%Y-%m-%d %H:%M").to_string()
}

pub fn print(res: Result<Vec<Issue>>, json: bool, full_width: bool) {
    use comfy_table::*;

    let res = match res {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{:?}", err);
            return;
        }
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&res).unwrap());
        return;
    }

    let mut table = Table::new();
    table.load_preset(comfy_table::presets::NOTHING);
    if full_width {
        table.set_content_arrangement(comfy_table::ContentArrangement::Disabled);
    } else {
        table.set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth);
    }

    table.add_row([
        Cell::new("id"),
        Cell::new("title"),
        Cell::new("state"),
        Cell::new("assignee"),
        Cell::new("created_at"),
        Cell::new("updated_at"),
        // Cell::new("parent"),
        Cell::new("priority"),
        // Cell::new("project"),
        // Cell::new("labels"),
        Cell::new("url"),
    ]);

    table
        .column_iter_mut()
        .last()
        .unwrap()
        .set_constraint(ColumnConstraint::ContentWidth);

    for issue in res {
        let Issue {
            url,
            identifier,
            title,
            created_at,
            updated_at,
            parent: _,
            priority,
            priority_label,
            project: _,
            assignee,
            state,
            labels: _,
            ..
        } = issue;

        let state = format!("{} ({})", state.name, state.type_);
        let priority = if priority == 0.0 { String::new() } else { priority_label };

        table.add_row([
            Cell::new(identifier),
            Cell::new(title),
            Cell::new(state),
            Cell::new(assignee.map(|a| a.display_name).unwrap_or_default()),
            Cell::new(fmt_date(created_at)),
            Cell::new(fmt_date(updated_at)),
            // Cell::new(parent.map(|p| p.id).unwrap_or_default()),
            Cell::new(priority),
            // Cell::new(project.map(|p| p.id).unwrap_or_default()),
            // Cell::new(labels.nodes.into_iter().map(|n| n.id).collect::<Vec<_>>().join(", ")),
            Cell::new(url),
        ]);
    }

    println!("{table}")
}
