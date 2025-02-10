use clap::{Parser, ValueEnum};

use fantasyf1::{puller, score};

#[derive(ValueEnum, Debug, Clone)]
enum Mode {
    Races,
    Results,
    Constructors,
    Score,
}

#[derive(Debug, Clone, Parser)]
struct Args {
    #[arg(short, long)]
    year: Option<u32>,
    #[arg(short, long)]
    round: Option<u32>,
    #[arg(short, long, value_enum)]
    mode: Mode,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    eprintln!("Args: {:?}", args);

    match args.mode {
        Mode::Results => {
            let results = puller::get_race_results(args.year, args.round)
                .await
                .expect("Failed to get results");

            println!("{}", serde_json::to_string_pretty(&results).unwrap());
        }
        Mode::Races => {
            let races = puller::get_races(args.year)
                .await
                .expect("Failed to get races");

            for race in races {
                println!(
                    "Race: {}, Date: {}, QualiData: {} Location: {}",
                    race.race_name,
                    race.date,
                    race.get_quali_date(),
                    race.circuit.location.locality,
                );
            }
        }
        Mode::Constructors => {
            let constructors = puller::get_constructors(args.year)
                .await
                .expect("Failed to get constructors");

            for constructor in constructors {
                println!("Constructor: {}", constructor.name);
            }
        }
        Mode::Score => {
            let race = puller::get_race_results(args.year, args.round)
                .await
                .expect("Failed to get results");

            println!("Score for: {}", race.race_name);
            for result in race.results {
                println!(
                    "{}: {} -> {} 10th: {}",
                    result.driver.family_name,
                    result
                        .get_position()
                        .map(|p| p.to_string())
                        .unwrap_or(format!("{}({})", result.position, result.position_text)),
                    score::to_score(&result),
                    score::tenth_score(&result),
                );
            }
        }
    }
}
