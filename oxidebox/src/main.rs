mod cli;
use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Summon { pokemon } => {
            println!("⚡ Summoning Pokémon: {}!", pokemon);
        }
        Commands::Recall { pokemon } => {
            println!("🛑 Recalling Pokémon: {}!", pokemon);
        }
        Commands::Pokedex => {
            println!("📖 Fetching Pokédex...");
        }
        Commands::Release { pokemon } => {
            println!("🌿 Releasing Pokémon: {} back into the wild!", pokemon);
        }
    }
}
