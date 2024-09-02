use crate::client::Client;
use eyre::Result;
use graphql_client::GraphQLQuery;

type DateTime = chrono::DateTime<chrono::Utc>;
type TimelessDate = chrono::NaiveDate;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "graphql/issue-show.graphql",
    schema_path = "graphql/linear-api.graphql",
    response_derives = "Debug, Clone, Serialize, Deserialize"
)]
struct ShowIssue;

pub type Issue = show_issue::ShowIssueIssue;

#[builder]
pub async fn request(client: &Client, issue_id: String) -> Result<Issue> {
    let query = ShowIssue::build_query(show_issue::Variables { issue_id });
    client
        .req::<_, show_issue::ResponseData>(query)
        .await
        .map(|res| res.issue)
}

fn fmt_date(date: DateTime) -> String {
    let date = date.with_timezone(&chrono::Local);
    date.format("%Y-%m-%d %H:%M").to_string()
}

pub fn print(res: Result<Issue>, json: bool, full_width: bool) {
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
    table.set_content_arrangement(comfy_table::ContentArrangement::Disabled);

    let Issue {
        url,
        updated_at,
        title,
        team,
        assignee,
        creator,
        branch_name: _,
        canceled_at,
        completed_at,
        created_at,
        description,
        due_date,
        estimate,
        id: _,
        identifier,
        labels,
        parent,
        priority,
        priority_label,
        project,
        started_at,
        state,
        trashed,
    } = res;

    let creator = creator.map(|u| u.display_name).unwrap_or_default();
    let assignee = assignee.map(|u| u.display_name).unwrap_or_default();
    let team = team.key;
    let labels = labels.nodes.into_iter().map(|l| l.name).collect::<Vec<_>>().join(", ");
    let priority = format!("{} ({})", priority_label, priority);
    let state = if trashed.unwrap_or(false) {
        format!("{} ({}) -- TRASHED", state.name, state.type_)
    } else {
        format!("{} ({})", state.name, state.type_)
    };

    let created_at = fmt_date(created_at);
    let updated_at = fmt_date(updated_at);
    let started_at = started_at.map(fmt_date);
    let completed_at = completed_at.map(fmt_date);
    let canceled_at = canceled_at.map(fmt_date);
    let due_date = due_date.map(|d| d.format("%Y-%m-%d").to_string());

    table.add_row([Cell::new("id"), Cell::new(&identifier)]);
    table.add_row([Cell::new("url"), Cell::new(&url)]);

    table.add_row([Cell::new("title"), Cell::new(&title)]);
    table.add_row([Cell::new("state"), Cell::new(&state)]);
    table.add_row([Cell::new("creator"), Cell::new(&creator)]);
    table.add_row([Cell::new("assignee"), Cell::new(&assignee)]);
    table.add_row([Cell::new("team"), Cell::new(&team)]);
    if !labels.is_empty() {
        table.add_row([Cell::new("labels"), Cell::new(&labels)]);
    }
    table.add_row([Cell::new("priority"), Cell::new(&priority)]);
    if let Some(parent) = parent.and_then(|p| p.parent) {
        table.add_row([Cell::new("parent"), Cell::new(&parent.identifier)]);
    }
    if let Some(project) = project {
        table.add_row([Cell::new("project"), Cell::new(&project.name)]);
    }

    table.add_row([Cell::new("created at"), Cell::new(&created_at)]);
    table.add_row([Cell::new("updated at"), Cell::new(&updated_at)]);
    if let Some(started_at) = started_at {
        table.add_row([Cell::new("started at"), Cell::new(&started_at)]);
    }
    if let Some(completed_at) = completed_at {
        table.add_row([Cell::new("completed at"), Cell::new(&completed_at)]);
    }
    if let Some(canceled_at) = canceled_at {
        table.add_row([Cell::new("canceled at"), Cell::new(&canceled_at)]);
    }
    if let Some(due_date) = due_date {
        table.add_row([Cell::new("due date"), Cell::new(&due_date)]);
    }
    if let Some(estimate) = estimate {
        table.add_row([Cell::new("estimate"), Cell::new(estimate)]);
    }

    if let Some(description) = description {
        table.add_row([Cell::new("description"), Cell::new(&description)]);
    }

    println!("{table}")
}
