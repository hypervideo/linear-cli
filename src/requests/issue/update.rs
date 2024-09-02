use crate::client::Client;
use eyre::Result;
use graphql_client::GraphQLQuery;

// input: issue_update::IssueUpdateInput {
//     title,
//     state_id: None,
//     description: None,
//     description_data: None,
//     assignee_id: None,
//     parent_id: None,
//     priority: None,
//     estimate: None,
//     subscriber_ids: None,
//     label_ids: None,
//     team_id: None,
//     cycle_id: None,
//     project_id: None,
//     project_milestone_id: None,
//     last_applied_template_id: None,
//     board_order: None,
//     sort_order: None,
//     priority_sort_order: None,
//     sub_issue_sort_order: None,
//     due_date: None,
//     trashed: None,
//     sla_breaches_at: None,
//     snoozed_until_at: None,
//     snoozed_by_id: None,
// },

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "graphql/issue-update.graphql",
    schema_path = "graphql/linear-api.graphql",
    input_derives = "Default",
    response_derives = "Debug"
)]
struct IssueUpdate;

#[builder]
pub async fn request(
    client: &Client,
    id: String,
    title: Option<String>,
    _state: Option<String>,
) -> Result<issue_update::IssueUpdateIssueUpdate> {
    let query = IssueUpdate::build_query(issue_update::Variables { id, title });
    client.req::<_, issue_update::IssueUpdateIssueUpdate>(query).await
}
