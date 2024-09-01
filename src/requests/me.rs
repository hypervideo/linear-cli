use eyre::Result;
use graphql_client::GraphQLQuery;

use crate::client::Client;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "graphql/me.graphql",
    schema_path = "graphql/linear-api.graphql",
    response_derives = "Debug,Clone,Serialize,Deserialize"
)]
struct Me;

pub async fn me(client: &Client) -> Result<me::MeViewer> {
    let query = Me::build_query(me::Variables {});
    client.req::<_, me::ResponseData>(query).await.map(|r| r.viewer)
}
