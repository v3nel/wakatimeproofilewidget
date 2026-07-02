use crate::error::{AppError, AppResult};

pub use crate::types::Config;

impl Config {
    pub fn from_env() -> AppResult<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            wakatime_api_key: required_env("WAKATIME_API_KEY")?,
            github_token: required_env("GITHUB_TOKEN")?,
            discord_app_id: required_env("DISCORD_APP_ID")?,
            discord_bot_token: required_env("BOT_TOKEN")?,
            discord_user_id: required_env("DISCORD_USER_ID")?,
        })
    }
}

fn required_env(name: &str) -> AppResult<String> {
    std::env::var(name).map_err(|_| AppError::MissingEnv(name.to_string()))
}
