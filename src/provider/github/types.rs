use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct GraphqlRequest<V> {
    pub query: &'static str,
    pub variables: V,
}

#[derive(Debug, Deserialize)]
pub struct GraphqlResponse<T> {
    pub data: Option<T>,
    #[serde(default)]
    pub errors: Vec<GraphqlError>,
}

#[derive(Debug, Deserialize)]
pub struct GraphqlError {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ViewerIdData {
    pub viewer: Viewer,
}

#[derive(Debug, Deserialize)]
pub struct Viewer {
    pub id: String,
    #[serde(rename = "login")]
    pub _login: String,
}

#[derive(Debug, Deserialize)]
pub struct ReposData {
    pub viewer: ViewerRepositories,
}

#[derive(Debug, Deserialize)]
pub struct ViewerRepositories {
    pub repositories: RepositoryConnection,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryConnection {
    pub page_info: PageInfo,
    #[serde(default)]
    pub nodes: Vec<Option<Repo>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    pub name: String,
    pub is_fork: bool,
    pub owner: RepoOwner,
    pub default_branch_ref: Option<BranchRef>,
}

#[derive(Debug, Deserialize)]
pub struct RepoOwner {
    pub login: String,
}

#[derive(Debug, Deserialize)]
pub struct BranchRef {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CommitAdditionsData {
    pub repository: Option<CommitRepository>,
}

#[derive(Debug, Deserialize)]
pub struct CommitRepository {
    #[serde(rename = "ref")]
    pub ref_: Option<RepositoryRef>,
}

#[derive(Debug, Deserialize)]
pub struct RepositoryRef {
    pub target: Option<CommitTarget>,
}

#[derive(Debug, Deserialize)]
pub struct CommitTarget {
    pub history: Option<CommitHistory>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitHistory {
    pub page_info: PageInfo,
    #[serde(default)]
    pub nodes: Vec<Option<CommitNode>>,
}

#[derive(Debug, Deserialize)]
pub struct CommitNode {
    pub additions: i64,
}
