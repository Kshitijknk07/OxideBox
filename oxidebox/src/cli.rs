use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("oxidebox")
        .about("A CLI tool for managing containers")
        .arg(
            Arg::new("horse")
                .short('o') 
                .long("horse")
                .value_name("HORSE")
                .help("Specifies the horse name")
                .required(true),
        )
        .arg(
            Arg::new("steed_id")
                .short('s')
                .long("steed-id")
                .value_name("STEED_ID")
                .help("Specifies the steed ID")
                .required(true),
        )
}