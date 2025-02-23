mod cli;
mod container;
mod battle;

use clap::Parser;
use cli::{Cli, Commands};
use container::Container;
use battle::Battle;
use std::collections::HashMap;

fn main() {
    let cli = Cli::parse();

    // Initialize a HashMap to store active Pokémon
    let mut containers: HashMap<String, Container> = HashMap::new();
    containers.insert("Pikachu".to_string(), Container::new("Pikachu", 10, 100, 25, 10, 15));
    containers.insert("Charizard".to_string(), Container::new("Charizard", 12, 120, 30, 15, 12));

    match cli.command {
        Commands::Summon { pokemon } => {
            if containers.contains_key(&pokemon) {
                println!("⚡ {} is already summoned!", pokemon);
            } else {
                let new_pokemon = Container::new(&pokemon, 5, 80, 20, 8, 12);
                containers.insert(pokemon.clone(), new_pokemon);
                println!("⚡ Summoning Pokémon: {}!", pokemon);
            }
        }
        Commands::Recall { pokemon } => {
            if containers.remove(&pokemon).is_some() {
                println!("🛑 Recalling Pokémon: {}!", pokemon);
            } else {
                println!("⚠️ Pokémon {} is not currently active!", pokemon);
            }
        }
        Commands::Pokedex => {
            println!("📖 Fetching Pokédex...");
            if containers.is_empty() {
                println!("⚠️ No Pokémon are currently active.");
            } else {
                for pokemon in containers.keys() {
                    println!("➡️ {}", pokemon);
                }
            }
        }
        Commands::Release { pokemon } => {
            if containers.remove(&pokemon).is_some() {
                println!("🌿 Releasing Pokémon: {} back into the wild!", pokemon);
            } else {
                println!("⚠️ Pokémon {} was not found!", pokemon);
            }
        }
        Commands::Battle { pokemon1, pokemon2 } => {
            if pokemon1 == pokemon2 {
                println!("⚠️ A Pokémon cannot battle itself!");
                return;
            }

            // Temporarily remove Pokémon from the HashMap for the battle
            let p1 = containers.remove(&pokemon1);
            let p2 = containers.remove(&pokemon2);

            // Check if both Pokémon exist
            if p1.is_none() || p2.is_none() {
                println!("⚠️ One or both Pokémon not found!");

                // Reinsert the Pokémon if they were removed but not used in the battle
                if let Some(p1) = p1 {
                    containers.insert(pokemon1, p1);
                }
                if let Some(p2) = p2 {
                    containers.insert(pokemon2, p2);
                }
                return;
            }

            // Unwrap the Pokémon since we've confirmed they exist
            let mut p1 = p1.unwrap();
            let mut p2 = p2.unwrap();

            // Start the battle
            Battle::start_battle(&mut p1, &mut p2);

            // Reinsert the Pokémon into the HashMap after the battle
            containers.insert(pokemon1, p1);
            containers.insert(pokemon2, p2);
        }
    }
}