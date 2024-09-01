use eyre::{OptionExt, Result};
use graphql_client::Response;
use serde::Serialize;

pub struct Client {
    api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn req<Q, D>(&self, query: Q) -> Result<D>
    where
        Q: Serialize,
        D: serde::de::DeserializeOwned,
    {
        let response = reqwest::Client::new()
            .post("https://api.linear.app/graphql")
            .header("Authorization", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&query)
            .send()
            .await?;

        let response_body: Response<D> = response.json().await?;

        response_body.data.ok_or_eyre("no data")
    }
}
