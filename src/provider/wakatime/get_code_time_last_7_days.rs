use crate::{AppResult, AppState, scripts::{format_digital_coding_time, get_last_7_days_dates}, types::WakatimeLast7DaysCodingTimeResponse};

pub async fn get_code_time_last_7_days(
    state: &AppState,
) -> AppResult<String> {
    let response = request_code_time_last_7_days(&state).await?;
    let formatedtime = format_digital_coding_time(response.cumulative_total.digital);

    Ok(formatedtime)
}

async fn request_code_time_last_7_days(
    state: &AppState
) -> AppResult<WakatimeLast7DaysCodingTimeResponse> {
    let (today, today_minus_7) = get_last_7_days_dates();
    let wakatime_api_key = &state.config.wakatime_api_key;

    let response = state.http
        .get(format!("https://wakatime.com/api/v1/users/current/summaries?start={today_minus_7}&end={today}"))
        .header("Authorization", format!("Basic {wakatime_api_key}"))
        .send()
        .await?
        .error_for_status()?
        .json::<WakatimeLast7DaysCodingTimeResponse>()
        .await?;
    
    Ok(response)
}