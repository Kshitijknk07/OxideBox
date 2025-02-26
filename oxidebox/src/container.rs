use std::collections::HashMap;
use crate::battle::Battle;  
use crate::moves::{Move, PokemonType, TypeEffectiveness};
use crate::database::Database;
use rusqlite::Result;

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
}

impl Container {
    // Update the new method signature
    pub fn new(name: &str, level: u32, hp: i32, attack: u32, defense: u32, speed: u32, pokemon_type: PokemonType) -> Self {
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
        }
    }

    // Fix the is_active method
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
    pub fn take_damage(&mut self, damage: i32) {
        if damage >= self.hp {
            self.hp = 0;
            self.status = "Fainted".to_string();
            println!("💀 {} has fainted!", self.name);
        } else {
            self.hp -= damage;
            println!("🔥 {} took {} damage! HP: {}", self.name, damage, self.hp);
        }
    }
    pub fn use_move(&mut self, move_index: usize, target: &mut Container) -> bool {
        if move_index >= self.moves.len() {
            println!("⚠️ Invalid move index!");
            return false;
        }

        let battle_move = &self.moves[move_index];
        let damage = self.calculate_damage(battle_move.power);
        
        println!("⚡ {} used {}!", self.name, battle_move.name);
        target.take_damage(damage);
        true
    }

    fn calculate_damage(&self, move_power: u32) -> i32 {
        (self.attack as f32 * move_power as f32 / 50.0) as i32
    }
}
/// Manages a collection of `Container` instances and teams.
pub struct ContainerManager {
    containers: HashMap<String, Container>,
}

impl ContainerManager {
    /// Creates a new `ContainerManager` instance.
    pub fn new() -> Self {
        ContainerManager {
            containers: HashMap::new(),
        }
    }

    /// Summons a new Pokémon and adds it to the containers.
    pub fn summon(&mut self, name: &str, level: u32, hp: i32, attack: u32, defense: u32, speed: u32, pokemon_type: PokemonType) {
        let container = Container::new(name, level, hp, attack, defense, speed, pokemon_type);
        self.containers.insert(name.to_string(), container);
        println!("⚡ Summoned Pokémon: {} (Level: {}, HP: {})", name, level, hp);
    }
    /// Recalls a Pokémon by updating its status to "Stopped".
    pub fn recall(&mut self, name: &str) {
        if let Some(container) = self.containers.get_mut(name) {
            container.status = "Stopped".to_string();
            println!("🛑 Recalling Pokémon: {}", name);
        } else {
            println!("⚠️ Pokémon {} not found!", name);
        }
    }

    /// Releases a Pokémon by removing it from the containers.
    pub fn release(&mut self, name: &str) {
        if self.containers.remove(name).is_some() {
            println!("🌿 Releasing Pokémon: {} back into the wild!", name);
        } else {
            println!("⚠️ Pokémon {} not found!", name);
        }
    }

    /// Displays detailed information about all Pokémon in the Pokédex.
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
    pub fn battle(&mut self, pokemon1: &str, pokemon2: &str) -> bool {
        if pokemon1 == pokemon2 {
            println!("⚠️ A Pokémon cannot battle itself!");
            return false;
        }

        // Take ownership of both Pokémon
        if let (Some(mut p1), Some(mut p2)) = (
            self.containers.remove(pokemon1),
            self.containers.remove(pokemon2)
        ) {
            Battle::start_battle(&mut p1, &mut p2);
            
            // Return Pokémon to containers
            self.containers.insert(pokemon1.to_string(), p1);
            self.containers.insert(pokemon2.to_string(), p2);
            true
        } else {
            println!("⚠️ One or both Pokémon not found!");
            false
        }
    }
    pub fn save_to_db(&self, name: &str) -> Result<(), rusqlite::Error> {
        let mut db = Database::new()?;  // Make db mutable
        if let Some(pokemon) = self.containers.get(name) {
            db.save_pokemon(pokemon)?;
        }
        Ok(())
    }

    pub fn load_from_db(&mut self, name: &str) -> Result<(), rusqlite::Error> {
        let db = Database::new()?;
        if let Some(pokemon) = db.load_pokemon(name)? {
            self.containers.insert(name.to_string(), pokemon);
        }
        Ok(())
    }
}