use reqwest::StatusCode;
use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;

use super::types::{GraphqlError, GraphqlRequest, GraphqlResponse};

const GITHUB_GRAPHQL_URL: &str = "https://api.github.com/graphql";

#[derive(Debug, Error)]
pub enum GithubError {
    #[error("missing environment variable: {0}")]
    MissingEnv(String),
    #[error("github http error {status}: {body}")]
    HttpStatus { status: StatusCode, body: String },
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("github graphql error(s): {0}")]
    Graphql(String),
    #[error("github graphql response did not contain data")]
    MissingData,
}

pub async fn graphql<T, V>(
    http: &reqwest::Client,
    token: &str,
    query: &'static str,
    variables: V,
) -> Result<T, GithubError>
where
    T: DeserializeOwned,
    V: Serialize,
{
    let request = GraphqlRequest { query, variables };

    let response = http
        .post(GITHUB_GRAPHQL_URL)
        .bearer_auth(token)
        .header("User-Agent", "discordwakatimewidget")
        .json(&request)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        return Err(GithubError::HttpStatus { status, body });
    }

    let response = serde_json::from_str::<GraphqlResponse<T>>(&body)?;

    if !response.errors.is_empty() {
        return Err(GithubError::Graphql(format_graphql_errors(response.errors)));
    }

    response.data.ok_or(GithubError::MissingData)
}

fn format_graphql_errors(errors: Vec<GraphqlError>) -> String {
    errors
        .into_iter()
        .map(|error| error.message)
        .collect::<Vec<_>>()
        .join("; ")
}
