use fantasyf1::puller;

#[tokio::main]  
async fn main() {
    let races = puller::get_races(None).await.expect("Failed to get races");
    assert!(!races.is_empty(), "Races should not be empty");
    for race in races {
        println!("Race: {}, Date: {}, QualiData: {} Location: {}, {}",
            race.race_name, race.date, race.get_quali_date(), race.circuit.location.locality, race.circuit.location.country);
    }
}
