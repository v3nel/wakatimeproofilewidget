mod config;
mod error;
mod provider;
mod state;
mod types;
mod scripts;

use crate::config::Config;
use crate::error::AppResult;
use crate::provider::discord::push_to_widget;
use crate::provider::wakatime::{get_last_7_days_stats, get_total_code_time, get_code_time_last_7_days};
use crate::state::AppState;

#[tokio::main]
async fn main() -> AppResult<()> {
    let config = Config::from_env()?;
    let http = reqwest::Client::new();
    let state = AppState { config, http };

    let code_time_total = get_total_code_time(&state).await?;
    let code_time_7_days = get_code_time_last_7_days(&state).await?;
    let (ai, human, fav_editor, fav_language) = get_last_7_days_stats(&state).await?;
//    let total_lines = get_total_lines(&state).await?;
    println!("{}", code_time_total);
    println!("{}", code_time_7_days);
    println!("{}", ai);
    println!("{}", human);
    println!("{}", fav_editor);
    println!("{}", fav_language);

    let response = push_to_widget(&state, code_time_total, code_time_7_days, "13k".to_string(), ai, human, fav_editor, fav_language).await?;
    Ok(())
}
