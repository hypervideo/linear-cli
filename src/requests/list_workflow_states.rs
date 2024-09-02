use crate::client::Client;
use eyre::Result;
use graphql_client::GraphQLQuery;

pub type DateTime = chrono::DateTime<chrono::Utc>;

pub type WorkflowState = list_workflow_states::WorkflowStateConnectionNodes;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "graphql/workflow_state_list.graphql",
    schema_path = "graphql/linear-api.graphql",
    response_derives = "Debug, Serialize"
)]
struct ListWorkflowStates;

#[builder]
pub async fn request(client: &Client) -> Result<Vec<WorkflowState>> {
    let query = ListWorkflowStates::build_query(list_workflow_states::Variables);
    let response = client.req::<_, list_workflow_states::ResponseData>(query).await?;
    Ok(response.workflow_states.nodes)
}
