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

pub async fn list_issues(client: &Client) -> Result<Vec<Issue>> {
    let n = Some(20);
    let mut i = 0;
    let mut after = None;
    let mut result = Vec::new();

    loop {
        debug!(page = %i, "list_issues");

        let query = ListIssues::build_query(list_issues::Variables {
            first: n,
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

        i += 1;
        after = response.page_info.end_cursor;
    }
}
