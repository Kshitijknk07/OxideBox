use std::collections::HashMap;
use crate::battle::Battle;  

#[derive(Debug, Clone)]
pub struct Container {
    pub name: String,
    pub status: String,
    pub level: u32, 
    pub hp: i32,
    pub attack: u32,
    pub defense: u32,
    pub speed: u32,
}

impl Container {
    /// Creates a new `Container` instance.
    pub fn new(name: &str, level: u32, hp: i32, attack: u32, defense: u32, speed: u32) -> Self {
        Self {
            name: name.to_string(),
            level,
            hp,
            attack,
            defense,
            speed,
            status: "Active".to_string(),
        }
    }

    /// Checks if the Pokémon is still active (HP > 0).
    pub fn is_active(&self) -> bool {
        self.hp > 0
    }

    /// Applies damage to the Pokémon and updates its status if it faints.
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
    pub fn summon(&mut self, name: &str, level: u32, hp: i32, attack: u32, defense: u32, speed: u32) {
        let container = Container::new(name, level, hp, attack, defense, speed);
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
}