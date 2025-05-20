use crate::puller::{Driver, Race, RaceResult};
use crate::score;

#[derive(Debug, Clone)]
enum DriverId {
    Piastri = 1,
    Norris = 2,
    Leclerc = 3,
    Hamilton = 4,
    Verstappen = 5,
    Lawson = 6,
    Russell = 7,
    Antonelli = 8,
    Stroll = 9,
    Alonso = 10,
    Gasly = 11,
    Doohan = 12,
    Hadjar = 13,
    Tsunoda = 14,
    Ocon = 15,
    Bearman = 16,
    Albon = 17,
    Sainz = 18,
    Hulkenberg = 19,
    Bortoleto = 20,
    Colapinto = 21,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseDriverIdError;

impl std::convert::TryFrom<&Driver> for DriverId {
    type Error = ParseDriverIdError;

    fn try_from(driver: &Driver) -> Result<Self, Self::Error> {
        driver.get_name().parse()
    }
}

impl std::str::FromStr for DriverId {
    type Err = ParseDriverIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Piastri" => Ok(DriverId::Piastri),
            "Norris" => Ok(DriverId::Norris),
            "Leclerc" => Ok(DriverId::Leclerc),
            "Hamilton" => Ok(DriverId::Hamilton),
            "Verstappen" => Ok(DriverId::Verstappen),
            "Lawson" => Ok(DriverId::Lawson),
            "Russell" => Ok(DriverId::Russell),
            "Antonelli" => Ok(DriverId::Antonelli),
            "Stroll" => Ok(DriverId::Stroll),
            "Alonso" => Ok(DriverId::Alonso),
            "Gasly" => Ok(DriverId::Gasly),
            "Doohan" => Ok(DriverId::Doohan),
            "Hadjar" => Ok(DriverId::Hadjar),
            "Tsunoda" => Ok(DriverId::Tsunoda),
            "Ocon" => Ok(DriverId::Ocon),
            "Bearman" => Ok(DriverId::Bearman),
            "Albon" => Ok(DriverId::Albon),
            "Sainz" => Ok(DriverId::Sainz),
            "Hulkenberg" => Ok(DriverId::Hulkenberg),
            "Bortoleto" => Ok(DriverId::Bortoleto),
            "Colapinto" => Ok(DriverId::Colapinto),
            _ => Err(ParseDriverIdError),
        }
    }
}

pub struct RaceResults {
    race: Race,
}

impl RaceResults {
    pub fn new(race: &Race) -> Self {
        let gone = vec!["Doohan"];
        let mut race = race.clone();

        race.results
            .extend(gone.iter().map(|&driver| RaceResult::retired(driver)));
        RaceResults::sort_results(&mut race.results);

        if !race.sprint_results.is_empty() {
            race.sprint_results
                .extend(gone.iter().map(|&driver| RaceResult::retired(driver)));
            RaceResults::sort_results(&mut race.sprint_results);
        }
        Self { race }
    }

    fn sort_results(results: &mut [RaceResult]) {
        results.sort_by(|a, b| {
            (DriverId::try_from(&a.driver).unwrap() as isize)
                .cmp(&(DriverId::try_from(&b.driver).unwrap() as isize))
        });
    }

    fn print_results(results: &[RaceResult]) {
        println!("Driver     \tPosition\tScore\tP10");
        println!("-------------------------------------------");
        for result in results {
            println!(
                "{driver:12}\t{position:8}\t{score}\t{p10}",
                driver = result.driver.get_name(),
                position = result
                    .get_position()
                    .map(|p| p.to_string())
                    .unwrap_or(format!("\t{}({})", result.position, result.position_text)),
                score = score::to_score(result),
                p10 = score::tenth_score(result),
            );
        }
    }

    pub fn print_race_results(&self) {
        if !self.race.sprint_results.is_empty() {
            println!("Sprint Results");
            RaceResults::print_results(&self.race.sprint_results);
            println!();
        }

        println!("Race Results");
        RaceResults::print_results(&self.race.results);
    }
}

pub fn print_results(race: &Race) {
    let result = RaceResults::new(race);
    result.print_race_results();
}
