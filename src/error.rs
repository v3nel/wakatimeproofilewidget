#[derive(Debug)]
pub enum AppError {
    MissingEnv(String),
    Http(reqwest::Error),
    Json(serde_json::Error),
    Github(crate::provider::github::GithubError),
}

pub type AppResult<T> = Result<T, AppError>;

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingEnv(name) => write!(f, "missing environment variable: {name}"),
            Self::Http(error) => write!(f, "http error: {error}"),
            Self::Json(error) => write!(f, "json error: {error}"),
            Self::Github(error) => write!(f, "github error: {error}"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::MissingEnv(_) => None,
            Self::Http(error) => Some(error),
            Self::Json(error) => Some(error),
            Self::Github(error) => Some(error),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<crate::provider::github::GithubError> for AppError {
    fn from(error: crate::provider::github::GithubError) -> Self {
        Self::Github(error)
    }
}
