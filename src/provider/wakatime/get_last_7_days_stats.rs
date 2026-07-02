use crate::{error::AppResult, state::AppState, types::WakatimeLast7DaysStatsResponse};

pub async fn get_last_7_days_stats(
    state: &AppState,
) -> AppResult<(String, String, String, String)> {
    let response = request_last_7_days_stats(&state).await?;
    
    let ai_additions = response.data.ai_additions.to_string();
    let ai_deletions = response.data.ai_deletions.to_string();
    let human_additions = response.data.human_additions.to_string();
    let human_deletions = response.data.human_deletions.to_string();

    let formated_ai = format!("+{ai_additions} -{ai_deletions}");
    let formated_human = format!("+{human_additions} -{human_deletions}");

    let fav_editor = response.data.editors
        .get(0)
        .map(|editor| editor.name.clone())
        .unwrap_or_else(|| "N/A".to_string());

    let fav_language = response.data.languages
        .get(0)
        .map(|language| language.name.clone())
        .unwrap_or_else(|| "N/A".to_string());
    
    Ok((formated_ai, formated_human, fav_editor, fav_language))
}

async fn request_last_7_days_stats(
    state: &AppState,
) -> AppResult<WakatimeLast7DaysStatsResponse> {
    let wakatime_api_key = &state.config.wakatime_api_key;
    
    let response = state.http
        .get(format!("https://wakatime.com/api/v1/users/current/stats/last_7_days"))
        .header("Authorization", format!("Basic {wakatime_api_key}"))
        .send()
        .await?
        .error_for_status()?
        .json::<WakatimeLast7DaysStatsResponse>()
        .await?;
    
    Ok(response)
}