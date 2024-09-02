use crate::client::Client;
use eyre::Result;
use graphql_client::GraphQLQuery;

pub type Team = list_teams::ListTeamsTeamsNodes;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "graphql/team_list.graphql",
    schema_path = "graphql/linear-api.graphql",
    response_derives = "Debug, Serialize"
)]
struct ListTeams;

#[builder]
pub async fn request(client: &Client) -> Result<Vec<Team>> {
    let query = ListTeams::build_query(list_teams::Variables);
    let response = client.req::<_, list_teams::ResponseData>(query).await?;
    Ok(response.teams.nodes)
}

pub fn print(res: Result<Vec<Team>>, json: bool, full_width: bool) {
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
        Cell::new("key"),
        Cell::new("name"),
        Cell::new("description"),
    ]);

    table
        .column_iter_mut()
        .last()
        .unwrap()
        .set_constraint(ColumnConstraint::ContentWidth);

    for issue in res {
        let Team {
            description,
            name,
            key,
            id,
        } = issue;

        table.add_row([
            Cell::new(id),
            Cell::new(key),
            Cell::new(name),
            Cell::new(description.unwrap_or_default()),
        ]);
    }

    println!("{table}")
}
