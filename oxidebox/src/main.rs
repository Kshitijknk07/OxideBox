mod cli;
mod container;
mod battle;
mod moves;
mod database;
mod evolution;
mod stats;

use clap::Parser;
use cli::{Cli, Commands};
use container::ContainerManager;
use moves::PokemonType;  // Single import for PokemonType
use evolution::EvolutionManager;

fn main() {
    let cli = Cli::parse();
    let mut container_manager = ContainerManager::new();
    let evolution_manager = EvolutionManager::new();

    // Initialize with some default Pokémon
    // Update Pokemon initialization
    container_manager.summon("Pikachu", 10, 100, 25, 10, 15, PokemonType::Electric);
    container_manager.summon("Charizard", 12, 120, 30, 15, 12, PokemonType::Fire);

    // In your main function where commands are matched:
    match cli.command {
        Commands::Summon { pokemon } => {
            container_manager.summon(&pokemon, 5, 80, 20, 8, 12, PokemonType::Normal);
        }
        Commands::Recall { pokemon } => {
            container_manager.recall(&pokemon);
        }
        Commands::Pokedex => {
            container_manager.pokedex();
        }
        Commands::Release { pokemon } => {
            container_manager.release(&pokemon);
        }
        Commands::Battle { pokemon1, pokemon2 } => {
            container_manager.battle(&pokemon1, &pokemon2, &evolution_manager);
        }
        Commands::Save { pokemon } => {
            if let Err(e) = container_manager.save_to_db(&pokemon) {
                println!("⚠️ Failed to save: {}", e);
            }
        },
        Commands::Load { pokemon } => {
            if let Err(e) = container_manager.load_from_db(&pokemon) {
                println!("⚠️ Failed to load: {}", e);
            }
        },
        Commands::Stats => {
            container_manager.display_stats();
        }
    }
}