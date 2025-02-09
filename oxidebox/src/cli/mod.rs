pub mod commands;

use std::env;

pub fn run_cli() {
    // Capture the command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: oxidebox <command>");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "start" => commands::start_container(),
        "stop" => commands::stop_container(),
        "status" => commands::status(),
        "restart" => commands::restart_container(),
        "logs" => commands::view_logs(),
        "help" => commands::help(),
        _ => println!("Unknown command: {}. Try 'start', 'stop', 'status', 'restart', 'logs', or 'help'.", command),
    }
}
