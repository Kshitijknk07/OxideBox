use std::collections::HashMap;

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
    teams: HashMap<String, Vec<String>>, // Team name -> List of container names
}

impl ContainerManager {
    /// Creates a new `ContainerManager` instance.
    pub fn new() -> Self {
        ContainerManager {
            containers: HashMap::new(),
            teams: HashMap::new(),
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
        } else {
            for container in self.containers.values() {
                println!(
                    "Pokémon: {} | Level: {} | HP: {} | Status: {}",
                    container.name, container.level, container.hp, container.status
                );
            }
        }
    }

    /// Creates a new team.
    pub fn create_team(&mut self, team_name: &str) {
        if self.teams.contains_key(team_name) {
            println!("⚠️ Team {} already exists!", team_name);
        } else {
            self.teams.insert(team_name.to_string(), Vec::new());
            println!("🌟 Created new team: {}", team_name);
        }
    }

    /// Adds a Pokémon to a team.
    pub fn add_to_team(&mut self, team_name: &str, container_name: &str) {
        if let Some(team) = self.teams.get_mut(team_name) {
            if self.containers.contains_key(container_name) {
                team.push(container_name.to_string());
                println!("➕ Added {} to team {}", container_name, team_name);
            } else {
                println!("⚠️ Pokémon {} not found!", container_name);
            }
        } else {
            println!("⚠️ Team {} not found!", team_name);
        }
    }

    /// Removes a Pokémon from a team.
    pub fn remove_from_team(&mut self, team_name: &str, container_name: &str) {
        if let Some(team) = self.teams.get_mut(team_name) {
            if let Some(index) = team.iter().position(|name| name == container_name) {
                team.remove(index);
                println!("➖ Removed {} from team {}", container_name, team_name);
            } else {
                println!("⚠️ Pokémon {} not found in team {}!", container_name, team_name);
            }
        } else {
            println!("⚠️ Team {} not found!", team_name);
        }
    }

    /// Displays information about a specific team.
    pub fn team_info(&self, team_name: &str) {
        if let Some(team) = self.teams.get(team_name) {
            println!("🌟 Team {}:", team_name);
            for container_name in team {
                if let Some(container) = self.containers.get(container_name) {
                    println!(
                        "Pokémon: {} | Level: {} | HP: {} | Status: {}",
                        container.name, container.level, container.hp, container.status
                    );
                } else {
                    println!("⚠️ Pokémon {} not found!", container_name);
                }
            }
        } else {
            println!("⚠️ Team {} not found!", team_name);
        }
    }
}