mod cli;

use clap::ArgMatches;

fn main() {
    let matches = cli::build_cli().get_matches();

    let horse = matches.get_one::<String>("horse").unwrap();
    let steed_id = matches.get_one::<String>("steed_id").unwrap();

    println!("Horse: {}", horse);
    println!("Steed ID: {}", steed_id);
}
