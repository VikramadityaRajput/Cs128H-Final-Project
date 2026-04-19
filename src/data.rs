use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct MatchRecord {
    #[serde(rename = "HomeTeam")]
    pub home_team: String,
    #[serde(rename = "AwayTeam")]
    pub away_team: String,
    #[serde(rename = "FTHG")]
    pub home_goals: i32,
    #[serde(rename = "FTAG")]
    pub away_goals: i32,
    #[serde(rename = "FTR")]
    pub result: String, // "H", "A", or "D"
}

pub fn load_data(file_path: &str) -> Result<Vec<MatchRecord>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(file_path)?;
    let mut records = Vec::new();
    for result in reader.deserialize() {
        let record: MatchRecord = result?;
        records.push(record);
    }
    Ok(records)
}