use std::collections::HashMap;
use crate::battle::Battle;  
use crate::moves::{Move, PokemonType};
use crate::database::Database;
use rusqlite::Result;
use crate::evolution::{Evolution, EvolutionManager};
use crate::stats::PokemonStats;
use chrono::Utc;
use crate::stats::TrainerStats;

#[derive(Debug, Clone)]
pub struct Container {
    pub name: String,
    pub status: String,
    pub level: u32,
    pub hp: i32,
    pub attack: u32,
    pub defense: u32,
    pub speed: u32,
    pub pokemon_type: PokemonType,
    pub moves: Vec<Move>,
    pub exp: u32,
    pub exp_to_next_level: u32,
    pub stats: PokemonStats,
}

impl Container {
    pub fn new(name: &str, level: u32, hp: i32, attack: u32, defense: u32, speed: u32, pokemon_type: PokemonType) -> Self {
        let exp_to_next_level = Self::calculate_exp_to_next_level(level);
        Self {
            name: name.to_string(),
            level,
            hp,
            attack,
            defense,
            speed,
            pokemon_type,
            status: "Active".to_string(),
            moves: Vec::new(),
            exp: 0,
            exp_to_next_level,
            stats: PokemonStats {
                battles_won: 0,
                battles_lost: 0,
                total_damage_dealt: 0,
                total_damage_taken: 0,
                evolution_count: 0,
                levels_gained: 0,
                moves_used: HashMap::new(),
                creation_date: Utc::now(),
                total_exp_gained: 0,
            },
        }
    }
    fn calculate_exp_to_next_level(level: u32) -> u32 {
        (level * level * 100) as u32
    }
    pub fn is_active(&self) -> bool {
        self.hp > 0
    }
    pub fn learn_move(&mut self, new_move: Move) {
        if self.moves.len() < 4 {
            println!("✨ {} learned {}!", self.name, new_move.name);
            self.moves.push(new_move);
        } else {
            println!("⚠️ {} already knows 4 moves!", self.name);
        }
    }
}

pub struct ContainerManager {
    containers: HashMap<String, Container>,
    trainer_stats: TrainerStats,
}

impl ContainerManager {
    pub fn new() -> Self {
        Self {
            containers: HashMap::new(),
            trainer_stats: TrainerStats::new(),
        }
    }

    pub fn display_stats(&self) {
        self.trainer_stats.display_detailed_stats();
    }

    pub fn summon(&mut self, name: &str, level: u32, hp: i32, attack: u32, defense: u32, speed: u32, pokemon_type: PokemonType) {
        let container = Container::new(name, level, hp, attack, defense, speed, pokemon_type);
        self.containers.insert(name.to_string(), container);
        self.trainer_stats.total_pokemon_caught += 1;
        println!("⚡ Summoned Pokémon: {} (Level: {}, HP: {})", name, level, hp);
    }

    pub fn recall(&mut self, name: &str) {
        if let Some(container) = self.containers.get_mut(name) {
            container.status = "Stopped".to_string();
            println!("🛑 Recalling Pokémon: {}", name);
        } else {
            println!("⚠️ Pokémon {} not found!", name);
        }
    }

    pub fn release(&mut self, name: &str) {
        if self.containers.remove(name).is_some() {
            self.trainer_stats.total_pokemon_released += 1;
            println!("🌿 Releasing Pokémon: {} back into the wild!", name);
        } else {
            println!("⚠️ Pokémon {} not found!", name);
        }
    }

    pub fn pokedex(&self) {
        println!("📖 Fetching Pokédex...");
        if self.containers.is_empty() {
            println!("⚠️ No Pokémon are currently active.");
            return;
        }
        for container in self.containers.values() {
            println!(
                "Pokémon: {} | Level: {} | HP: {} | Status: {}",
                container.name, container.level, container.hp, container.status
            );
        }
    }

    pub fn get_container(&self, name: &str) -> Option<&Container> {
        self.containers.get(name)
    }

    pub fn get_container_mut(&mut self, name: &str) -> Option<&mut Container> {
        self.containers.get_mut(name)
    }

    pub fn get_containers_mut(&mut self) -> &mut HashMap<String, Container> {
        &mut self.containers
    }

    pub fn battle(&mut self, pokemon1: &str, pokemon2: &str, evolution_manager: &EvolutionManager) -> bool {
        if pokemon1 == pokemon2 {
            println!("⚠️ A Pokémon cannot battle itself!");
            return false;
        }

        if let (Some(mut p1), Some(mut p2)) = (
            self.containers.remove(pokemon1),
            self.containers.remove(pokemon2)
        ) {
            Battle::start_battle(&mut p1, &mut p2, evolution_manager);
            
            self.containers.insert(pokemon1.to_string(), p1);
            self.containers.insert(pokemon2.to_string(), p2);
            true
        } else {
            println!("⚠️ One or both Pokémon not found!");
            false
        }
    }

    pub fn save_to_db(&self, name: &str) -> Result<(), rusqlite::Error> {
        let mut db = Database::new()?;
        if let Some(pokemon) = self.containers.get(name) {
            db.save_pokemon(pokemon)?;
        }
        Ok(())
    }

    pub fn load_from_db(&mut self, name: &str) -> Result<(), rusqlite::Error> {
        let mut db = Database::new()?;
        if let Some(pokemon) = db.load_pokemon(name)? {
            self.containers.insert(name.to_string(), pokemon);
            println!("📥 Loaded Pokémon {} from database", name);
        } else {
            println!("⚠️ No Pokémon found in database with name: {}", name);
        }
        Ok(())
    }
}