use serde_json::json;

use super::{
    client::{GithubError, graphql},
    queries,
    types::{CommitAdditionsData, Repo, ReposData, ViewerIdData},
};

const GITHUB_USERNAME: &str = "v3nel";
const EXCLUDED_README_REPO: &str = "v3nel";

pub async fn get_total_lines_written() -> Result<i64, GithubError> {
    let token = std::env::var("GITHUB_TOKEN")
        .map_err(|_| GithubError::MissingEnv("GITHUB_TOKEN".to_string()))?;
    let http = reqwest::Client::new();

    let viewer = get_viewer(&http, &token).await?;
    let repos = get_repos(&http, &token).await?;
    let mut total = 0_i64;

    for repo in repos.iter().filter(|repo| !should_skip_repo(repo)) {
        let branch = match &repo.default_branch_ref {
            Some(branch) => &branch.name,
            None => continue,
        };

        total += get_repo_commit_additions(
            &http,
            &token,
            &repo.owner.login,
            &repo.name,
            branch,
            &viewer.id,
        )
        .await?;
    }

    Ok(total)
}

async fn get_viewer(
    http: &reqwest::Client,
    token: &str,
) -> Result<super::types::Viewer, GithubError> {
    let data = graphql::<ViewerIdData, _>(http, token, queries::VIEWER_ID, json!({})).await?;
    Ok(data.viewer)
}

async fn get_repos(http: &reqwest::Client, token: &str) -> Result<Vec<Repo>, GithubError> {
    let mut repos = Vec::new();
    let mut cursor: Option<String> = None;

    loop {
        let data =
            graphql::<ReposData, _>(http, token, queries::GET_REPOS, json!({ "cursor": cursor }))
                .await?;

        repos.extend(data.viewer.repositories.nodes.into_iter().flatten());

        if !data.viewer.repositories.page_info.has_next_page {
            break;
        }

        cursor = data.viewer.repositories.page_info.end_cursor;
    }

    Ok(repos)
}

async fn get_repo_commit_additions(
    http: &reqwest::Client,
    token: &str,
    owner: &str,
    repo: &str,
    branch: &str,
    author_id: &str,
) -> Result<i64, GithubError> {
    let mut total = 0_i64;
    let mut cursor: Option<String> = None;

    loop {
        let data = graphql::<CommitAdditionsData, _>(
            http,
            token,
            queries::GET_COMMIT_ADDITIONS,
            json!({
                "owner": owner,
                "repo": repo,
                "branch": branch,
                "authorId": author_id,
                "cursor": cursor,
            }),
        )
        .await?;

        let Some(history) = data
            .repository
            .and_then(|repository| repository.ref_)
            .and_then(|ref_| ref_.target)
            .and_then(|target| target.history)
        else {
            break;
        };

        total += history
            .nodes
            .into_iter()
            .flatten()
            .map(|commit| commit.additions)
            .sum::<i64>();

        if !history.page_info.has_next_page {
            break;
        }

        cursor = history.page_info.end_cursor;
    }

    Ok(total)
}

fn should_skip_repo(repo: &Repo) -> bool {
    repo.is_fork
        || (repo.owner.login.eq_ignore_ascii_case(GITHUB_USERNAME)
            && repo.name.eq_ignore_ascii_case(EXCLUDED_README_REPO))
        || repo.default_branch_ref.is_none()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::github::types::{BranchRef, RepoOwner};

    #[test]
    fn skips_forks_readme_repo_and_repos_without_default_branch() {
        let fork = Repo {
            name: "project".to_string(),
            is_fork: true,
            owner: RepoOwner {
                login: "v3nel".to_string(),
            },
            default_branch_ref: Some(BranchRef {
                name: "main".to_string(),
            }),
        };
        let readme_repo = Repo {
            name: "V3NEL".to_string(),
            is_fork: false,
            owner: RepoOwner {
                login: "V3NEL".to_string(),
            },
            default_branch_ref: Some(BranchRef {
                name: "main".to_string(),
            }),
        };
        let empty_repo = Repo {
            name: "empty".to_string(),
            is_fork: false,
            owner: RepoOwner {
                login: "v3nel".to_string(),
            },
            default_branch_ref: None,
        };
        let valid_repo = Repo {
            name: "project".to_string(),
            is_fork: false,
            owner: RepoOwner {
                login: "org".to_string(),
            },
            default_branch_ref: Some(BranchRef {
                name: "main".to_string(),
            }),
        };

        assert!(should_skip_repo(&fork));
        assert!(should_skip_repo(&readme_repo));
        assert!(should_skip_repo(&empty_repo));
        assert!(!should_skip_repo(&valid_repo));
    }
}
