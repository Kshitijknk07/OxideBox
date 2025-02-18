use std::collections::HashMap;

#[derive(Debug)]
pub struct Container {
    pub name: String,
    pub status: String,  // "active", "stopped", "released"
    pub level: u32,      // Pokémon level
    pub hp: u32,         // Pokémon health points
}

pub struct ContainerManager {
    containers: HashMap<String, Container>,
    teams: HashMap<String, Vec<String>>, // Team name -> List of container names
}

impl ContainerManager {
    pub fn new() -> Self {
        ContainerManager {
            containers: HashMap::new(),
            teams: HashMap::new(),
        }
    }

    // Summon a new Pokémon with level and HP
    pub fn summon(&mut self, name: &str, level: u32, hp: u32) {
        let container = Container {
            name: name.to_string(),
            status: "active".to_string(),
            level,
            hp,
        };
        self.containers.insert(name.to_string(), container);
        println!("⚡ Summoned Pokémon: {} (Level: {}, HP: {})", name, level, hp);
    }

    // Recall a Pokémon (change status to "stopped")
    pub fn recall(&mut self, name: &str) {
        if let Some(container) = self.containers.get_mut(name) {
            container.status = "stopped".to_string();
            println!("🛑 Recalling Pokémon: {} ({})", name, container.name);
        } else {
            println!("⚠️ Pokémon {} not found!", name);
        }
    }

    // Release a Pokémon (remove from containers)
    pub fn release(&mut self, name: &str) {
        if let Some(container) = self.containers.remove(name) {
            println!("🌿 Releasing Pokémon: {} ({}) back into the wild!", name, container.name);
        } else {
            println!("⚠️ Pokémon {} not found!", name);
        }
    }

    // Display detailed Pokédex information
    pub fn pokedex(&self) {
        println!("📖 Fetching Pokédex...");
        for (name, container) in &self.containers {
            println!(
                "Pokémon: {} | Level: {} | HP: {} | Status: {}",
                name, container.level, container.hp, container.status
            );
        }
    }

    // Create a new team
    pub fn create_team(&mut self, team_name: &str) {
        if self.teams.contains_key(team_name) {
            println!("⚠️ Team {} already exists!", team_name);
        } else {
            self.teams.insert(team_name.to_string(), Vec::new());
            println!("🌟 Created new team: {}", team_name);
        }
    }

    // Add a Pokémon to a team
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

    // Remove a Pokémon from a team
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

    // Display information about a specific team
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