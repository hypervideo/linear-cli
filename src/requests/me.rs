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

pub async fn request(client: &Client) -> Result<me::MeViewer> {
    let query = Me::build_query(me::Variables {});
    client.req::<_, me::ResponseData>(query).await.map(|r| r.viewer)
}

pub fn print(res: Result<me::MeViewer>) {
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
    table.add_row([Cell::new("id"), Cell::new("name"), Cell::new("email")]);
    table.add_row([Cell::new(res.id), Cell::new(res.name), Cell::new(res.email)]);
    println!("{table}")
}
