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
        D: std::fmt::Debug,
    {
        trace!(query = %serde_json::to_string_pretty(&query).unwrap(), "sending query");

        let response = reqwest::Client::new()
            .post("https://api.linear.app/graphql")
            .header("Authorization", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&query)
            .send()
            .await?;

        trace!(status = ?response.status(), headers = ?response.headers(), "response");

        let response_body: Response<D> = response.json().await?;

        match response_body.errors {
            Some(mut errors) if !errors.is_empty() => {
                for error in &errors {
                    error!("graphql error {}", serde_json::to_string_pretty(&error).unwrap());
                }
                let error = errors.pop().unwrap();
                return Err(eyre::eyre!("graphql error: {}", error.message));
            }
            _ => (),
        }

        response_body.data.ok_or_eyre("no data")
    }
}
