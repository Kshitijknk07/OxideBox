mod cli;

fn main() {
    // Print RDR2-inspired welcome message
    println!("░▒▓▓▓▒░  OXIDEBOX  ░▒▓▓▓▒░");
    println!("    The Container Frontier\n");
    println!("Welcome to OxideBox, partner. Your journey into the container frontier begins now...\n");

    // Call the CLI commands
    cli::run_cli();
}
