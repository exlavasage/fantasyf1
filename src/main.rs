use clap::Parser;

use fantasyf1::puller;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    year: Option<u32>,
    #[clap(short, long)]
    results: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.results {
        let results = puller::get_race_results(args.year, None)
            .await
            .expect("Failed to get results");

        println!("{}", serde_json::to_string_pretty(&results).unwrap());
    } else {
        let races = puller::get_races(args.year)
            .await
            .expect("Failed to get races");

        for race in races {
            println!(
                "Race: {}, Date: {}, QualiData: {} Location: {}, {}",
                race.race_name,
                race.date,
                race.get_quali_date(),
                race.circuit.location.locality,
                race.circuit.location.country
            );
        }
    }
}
