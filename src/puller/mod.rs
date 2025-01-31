use anyhow::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_alias::serde_alias;

const BASE_URL: &str = "https://api.jolpi.ca/ergast/f1";

// TODO some of these fields can be flatted
// https://crates.io/crates/serde_flat_path

#[serde_alias(CamelCase, PascalCase)]
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Race {
    pub race_name: String,
    pub date: NaiveDate,
    pub circuit: Circuit,
    pub qualifying: Option<Session>,
    pub sprint_qualifying: Option<Session>,
    #[serde(default)]
    pub results: Vec<RaceResult>,
}

impl Race {
    /// Get the earliest of the qualifying dates
    pub fn get_quali_date(&self) -> NaiveDate {
        self.sprint_qualifying
            .as_ref()
            .unwrap_or(&self.qualifying.as_ref().unwrap())
            .date
            .clone()
    }
}

#[serde_alias(CamelCase, PascalCase)]
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct RaceResult {
    pub position: String,
    pub driver: Driver,
    pub constructor: Constructor,
    pub time: Option<Time>,
}

#[serde_alias(CamelCase, PascalCase)]
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Driver {
    pub given_name: String,
    pub family_name: String,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Constructor {
    pub name: String,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Time {
    pub time: String,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Session {
    pub date: NaiveDate,
}

#[serde_alias(CamelCase, PascalCase)]
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Circuit {
    pub circuit_name: String,
    pub location: Location,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Location {
    pub locality: String,
    pub country: String,
}

#[serde_alias(CamelCase, PascalCase)]
#[derive(Deserialize, Debug)]
struct RaceTable {
    races: Vec<Race>,
}

#[serde_alias(CamelCase, PascalCase)]
#[derive(Deserialize, Debug)]
struct MRData {
    race_table: RaceTable,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    #[serde(alias = "MRData")]
    mr_data: MRData,
}

pub async fn get_races(year: Option<u32>) -> Result<Vec<Race>> {
    let url = if let Some(year) = year {
        format!("{}/{}.json", BASE_URL, year)
    } else {
        format!("{}/current.json", BASE_URL)
    };

    let response = reqwest::get(&url).await?;
    let json: serde_json::Value = response.json().await?;
    eprintln!("{}", serde_json::to_string_pretty(&json)?);

    let api_response: ApiResponse = serde_json::from_value(json)?;
    Ok(api_response.mr_data.race_table.races)
}

pub async fn get_race_results(year: Option<u32>, round: Option<u32>) -> Result<Race> {
    let year = year.map_or("current".to_string(), |y| y.to_string());
    let round = round.map_or("last".to_string(), |r| r.to_string());

    let url = format!("{}/{}/{}/results.json", BASE_URL, year, round);

    let response = reqwest::get(&url).await?;
    let json: serde_json::Value = response.json().await?;
    eprintln!("{}", serde_json::to_string_pretty(&json)?);

    let api_response: ApiResponse = serde_json::from_value(json)?;
    Ok(api_response.mr_data.race_table.races[0].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_races() {
        match get_races(None).await {
            Ok(races) => {
                assert!(!races.is_empty(), "Races should not be empty");
                for race in races {
                    println!(
                        "Race: {}, Date: {}, Location: {}, {}",
                        race.race_name,
                        race.date,
                        race.circuit.location.locality,
                        race.circuit.location.country
                    );
                }
            }
            Err(e) => panic!("Error fetching races: {}", e),
        }
    }
}
