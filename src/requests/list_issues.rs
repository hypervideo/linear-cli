use eyre::Result;
use graphql_client::GraphQLQuery;

use crate::client::Client;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "graphql/list-issues.graphql",
    schema_path = "graphql/linear-api.graphql",
    response_derives = "Debug,Clone,Serialize,Deserialize"
)]
struct ListIssues;

pub type DateTime = chrono::DateTime<chrono::Utc>;
pub type Issue = list_issues::Issue;

#[builder]
pub async fn request(client: &Client, n: Option<usize>) -> Result<Vec<Issue>> {
    let mut per_page = 20;
    let mut i = 0;
    let mut after = None;
    let mut result = Vec::new();

    loop {
        debug!(page = %i, %per_page, "list_issues");

        let query = ListIssues::build_query(list_issues::Variables {
            first: Some(per_page as _),
            order_by: Some(list_issues::PaginationOrderBy::createdAt),
            after,
            before: None,
            include_archived: None,
            last: None,
        });

        let response = client
            .req::<_, list_issues::ResponseData>(query)
            .await
            .map(|r| r.issues)?;

        result.extend(response.edges.into_iter().map(|e| e.node));

        if !response.page_info.has_next_page {
            break Ok(result);
        }

        if let Some(n) = n {
            if result.len() >= n {
                break Ok(result);
            }
            let remaining = n - result.len();
            per_page = remaining.min(per_page);
        }

        i += 1;
        after = response.page_info.end_cursor;
    }
}

pub fn print(res: Result<Vec<Issue>>) {
    use comfy_table::*;

    let res = match res {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{:?}", err);
            return;
        }
    };

    let mut table = Table::new();
    table.load_preset(comfy_table::presets::ASCII_BORDERS_ONLY_CONDENSED);
    table.set_content_arrangement(comfy_table::ContentArrangement::DynamicFullWidth);
    table.add_row([
        Cell::new("url"),
        Cell::new("id"),
        Cell::new("title"),
        Cell::new("created_at"),
        Cell::new("updated_at"),
        Cell::new("parent"),
        Cell::new("priority_label"),
        Cell::new("project"),
        Cell::new("assignee"),
        Cell::new("state"),
        Cell::new("labels"),
    ]);
    table
        .column_mut(0)
        .unwrap()
        .set_constraint(ColumnConstraint::ContentWidth);

    for issue in res {
        let Issue {
            url,
            identifier,
            title,
            created_at,
            updated_at,
            parent,
            priority_label,
            project,
            assignee,
            state,
            labels,
            ..
        } = issue;

        let state_t = state.type_;
        let state = state.name;

        table.add_row([
            Cell::new(url),
            Cell::new(identifier),
            Cell::new(title),
            Cell::new(created_at.to_string()),
            Cell::new(updated_at.to_string()),
            Cell::new(parent.map(|p| p.id).unwrap_or_default()),
            Cell::new(priority_label),
            Cell::new(project.map(|p| p.id).unwrap_or_default()),
            Cell::new(assignee.map(|a| a.display_name).unwrap_or_default()),
            Cell::new(format!("{} - {}", state, state_t)),
            Cell::new(labels.nodes.into_iter().map(|n| n.id).collect::<Vec<_>>().join(", ")),
        ]);
    }

    println!("{table}")
}
