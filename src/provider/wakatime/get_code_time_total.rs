use crate::{AppResult, AppState, scripts::format_digital_coding_time, types::WakatimeAllTimeStatsResponse};

pub async fn get_total_code_time(
    state: &AppState,
) -> AppResult<String> {
    let wakatime_total_code_time = request_total_code_time(state).await?.data.digital;
    let formated = format_digital_coding_time(wakatime_total_code_time);
    
    Ok(formated)
}

async fn request_total_code_time(
    state: &AppState,
) -> AppResult<WakatimeAllTimeStatsResponse> {
    let base64apikey = &state.config.wakatime_api_key;
    let response = state
        .http
        .get("https://api.wakatime.com/api/v1/users/current/all_time_since_today")
        .header("Authorization", format!("Basic {}", base64apikey))
        .send()
        .await?
        .error_for_status()?
        .json::<WakatimeAllTimeStatsResponse>()
        .await?;

    Ok(response)
}