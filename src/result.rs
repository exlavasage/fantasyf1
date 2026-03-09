use crate::puller::{Driver, Race, RaceResult};
use crate::score;

#[derive(Debug, Clone)]
enum DriverId {
    Norris = 1,
    Piastri = 2,
    Russell = 3,
    Antonelli = 4,
    Verstappen = 5,
    Hadjar = 6,
    Leclerc = 7,
    Hamilton = 8,
    Albon = 9,
    Sainz = 10,
    Lawson = 11,
    Lindblad = 12,
    Alonso = 13,
    Stroll = 14,
    Bearman = 15,
    Ocon = 16,
    Hulkenberg = 17,
    Bortoleto = 18,
    Gasly = 19,
    Colapinto = 20,
    Bottas = 21,
    Perez = 22,
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
            "Hadjar" => Ok(DriverId::Hadjar),
            "Russell" => Ok(DriverId::Russell),
            "Antonelli" => Ok(DriverId::Antonelli),
            "Stroll" => Ok(DriverId::Stroll),
            "Alonso" => Ok(DriverId::Alonso),
            "Gasly" => Ok(DriverId::Gasly),
            "Lawson" => Ok(DriverId::Lawson),
            "Lindblad" => Ok(DriverId::Lindblad),
            "Ocon" => Ok(DriverId::Ocon),
            "Bearman" => Ok(DriverId::Bearman),
            "Albon" => Ok(DriverId::Albon),
            "Sainz" => Ok(DriverId::Sainz),
            "Hulkenberg" => Ok(DriverId::Hulkenberg),
            "Bortoleto" => Ok(DriverId::Bortoleto),
            "Colapinto" => Ok(DriverId::Colapinto),
            "Bottas" => Ok(DriverId::Bottas),
            "Perez" => Ok(DriverId::Perez),
            _ => Err(ParseDriverIdError),
        }
    }
}

pub struct RaceResults {
    race: Race,
}

impl RaceResults {
    pub fn new(race: &Race) -> Self {
        let gone = [];
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
