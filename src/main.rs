use clap::{Parser, ValueEnum};

use fantasyf1::{puller, result};

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

            println!("Race          \tLocation\tDate      \tQualiData");
            for race in races {
                println!(
                    "{race:15}\t{location:15}\t{date:20}\t{quali:20}",
                    race = race.name(),
                    date = race.date,
                    quali = race.get_quali_date(),
                    location = race.location()
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
            result::print_results(&race);
        }
    }
}
