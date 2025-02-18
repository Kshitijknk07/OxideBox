// main.rs

mod cli;
mod container;

use clap::Parser;
use cli::{Cli, Commands};
use container::ContainerManager;

fn main() {
    let mut container_manager = ContainerManager::new();
    let cli = Cli::parse();

    match cli.command {
        Commands::Summon { pokemon } => {
            container_manager.summon(&pokemon);
        }
        Commands::Recall { pokemon } => {
            container_manager.recall(&pokemon);
        }
        Commands::Release { pokemon } => {
            container_manager.release(&pokemon);
        }
        Commands::Pokedex => {
            container_manager.pokedex();
        }
    }
}
