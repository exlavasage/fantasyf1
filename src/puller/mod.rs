use anyhow::Result;
use serde::Deserialize;
use chrono::NaiveDate;

const BASE_URL: &str = "https://api.jolpi.ca/ergast/f1";

#[derive(Deserialize, Debug)]
pub struct Race {
    #[serde(alias = "raceName")]
    pub race_name: String,
    pub date: NaiveDate,
    #[serde(alias = "Circuit")]  
    pub circuit: Circuit,
    #[serde(alias = "Qualifying")]
    pub qualifying: Session,
    #[serde(alias = "SprintQualifying")]
    pub sprint_qualifying: Option<Session>,
}

impl Race {
    /// Get the earliest of the qualifying dates
    pub fn get_quali_date(&self) -> NaiveDate {
        self.sprint_qualifying.as_ref().unwrap_or(&self.qualifying).date.clone()
    }
}

#[derive(Deserialize, Debug)]
pub struct Session {
    pub date: NaiveDate,
}

#[derive(Deserialize, Debug)]
pub struct Circuit {
    #[serde(alias = "circuitName")]
    pub circuit_name: String,
    #[serde(alias = "Location")]  
    pub location: Location,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    pub locality: String,
    pub country: String,
}

#[derive(Deserialize, Debug)]
struct RaceTable {
    #[serde(alias = "Races")]
    races: Vec<Race>,
}

#[derive(Deserialize, Debug)]
struct MRData {
    #[serde(alias = "RaceTable")]
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
    let json : serde_json::Value = response.json().await?;
    eprintln!("{}", serde_json::to_string_pretty(&json)?);

    let api_response : ApiResponse = serde_json::from_value(json)?;  
    Ok(api_response.mr_data.race_table.races)
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
                    println!("Race: {}, Date: {}, Location: {}, {}",
                        race.race_name, race.date, race.circuit.location.locality, race.circuit.location.country);
                }
            }
            Err(e) => panic!("Error fetching races: {}", e),
        }
    }
}