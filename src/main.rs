#![windows_subsystem = "windows"]

mod config;
mod error;
mod provider;
mod scripts;
mod state;
mod types;

use crate::config::Config;
use crate::error::AppResult;
use crate::provider::discord::{WidgetData, push_to_widget};
use crate::provider::github::get_total_lines_written;
use crate::provider::wakatime::{
    get_code_time_last_7_days, get_last_7_days_stats, get_total_code_time,
};
use crate::scripts::format_total_lines;
use crate::state::AppState;

#[tokio::main]
async fn main() -> AppResult<()> {
    let config = Config::from_env()?;
    let http = reqwest::Client::new();
    let state = AppState { config, http };

    let code_time_total = get_total_code_time(&state).await?;
    let code_time_7_days = get_code_time_last_7_days(&state).await?;
    let (ai, human, fav_editor, fav_language) = get_last_7_days_stats(&state).await?;
    let total_lines = format_total_lines(get_total_lines_written().await?);
    println!("{}", code_time_total);
    println!("{}", code_time_7_days);
    println!("{}", ai);
    println!("{}", human);
    println!("{}", fav_editor);
    println!("{}", fav_language);

    push_to_widget(
        &state,
        WidgetData {
            code_time_total,
            code_time_7_days,
            total_lines,
            ai,
            human,
            fav_editor,
            fav_language,
        },
    )
    .await?;
    Ok(())
}
