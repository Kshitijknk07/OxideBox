
pub fn build_cli() -> Command {
    Command::new("oxidebox")
        .version("0.1")
        .author("Your Name")
        .long_about("OxideBox container runtime")
        .arg(
            Arg::new("horse")
                .help("The horse to ride (image to run)")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("steed_id")
                .help("ID of the horse to tether (container ID)")
                .required(true)
                .index(2),
        )
}
