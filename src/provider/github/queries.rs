pub const VIEWER_ID: &str = r#"
query ViewerId {
  viewer {
    id
    login
  }
}
"#;

pub const GET_REPOS: &str = r#"
query GetRepos($cursor: String) {
  viewer {
    repositories(
      first: 100
      after: $cursor
      affiliations: [OWNER, COLLABORATOR, ORGANIZATION_MEMBER]
      ownerAffiliations: [OWNER, COLLABORATOR, ORGANIZATION_MEMBER]
      isFork: false
    ) {
      pageInfo {
        hasNextPage
        endCursor
      }
      nodes {
        name
        isFork
        owner {
          login
        }
        defaultBranchRef {
          name
        }
      }
    }
  }
}
"#;

pub const GET_COMMIT_ADDITIONS: &str = r#"
query GetCommitAdditions(
  $owner: String!
  $repo: String!
  $branch: String!
  $authorId: ID!
  $cursor: String
) {
  repository(owner: $owner, name: $repo) {
    ref(qualifiedName: $branch) {
      target {
        ... on Commit {
          history(
            first: 100
            after: $cursor
            author: { id: $authorId }
          ) {
            pageInfo {
              hasNextPage
              endCursor
            }
            nodes {
              additions
            }
          }
        }
      }
    }
  }
}
"#;
