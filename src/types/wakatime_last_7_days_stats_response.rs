#[derive(serde::Deserialize)]
pub struct WakatimeLast7DaysStatsResponse {
    pub data: WakatimeLast7DaysStatsData,
}

#[derive(serde::Deserialize)]
pub struct WakatimeLast7DaysStatsData {
    pub ai_additions: u64,
    pub ai_deletions: u64,
    pub human_additions: u64,
    pub human_deletions: u64,
    pub editors: Vec<Editors>,
    pub languages: Vec<Languages>,
    
}

#[derive(serde::Deserialize)]
pub struct Editors {
    pub name: String,
}

#[derive(serde::Deserialize)]
pub struct Languages {
    pub name: String,
}