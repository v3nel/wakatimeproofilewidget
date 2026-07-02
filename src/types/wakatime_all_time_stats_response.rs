#[derive(serde::Deserialize)]
pub struct WakatimeAllTimeStatsResponse {
    pub data: WakatimeAllTimeStatsData
}

#[derive(serde::Deserialize)]
pub struct WakatimeAllTimeStatsData {
    pub digital: String
}