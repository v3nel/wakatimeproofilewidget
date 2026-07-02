#[derive(serde::Deserialize)]
pub struct WakatimeLast7DaysCodingTimeResponse {
    pub cumulative_total: WakatimeLast7DaysCodingTimeData,
}

#[derive(serde::Deserialize)]
pub struct WakatimeLast7DaysCodingTimeData {
    pub digital: String,
}