mod cli;
mod container;
mod battle;

use clap::Parser;
use cli::{Cli, Commands};
use container::ContainerManager;

fn main() {
    let cli = Cli::parse();
    let mut container_manager = ContainerManager::new();

    // Initialize with some default Pokémon
    container_manager.summon("Pikachu", 10, 100, 25, 10, 15);
    container_manager.summon("Charizard", 12, 120, 30, 15, 12);

    match cli.command {
        Commands::Summon { pokemon } => {
            container_manager.summon(&pokemon, 5, 80, 20, 8, 12);
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
            container_manager.battle(&pokemon1, &pokemon2);
        }
    }
}