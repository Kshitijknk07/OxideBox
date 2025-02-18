// team.rs

use std::collections::HashMap;
use crate::container::{Container, ContainerManager};

pub struct TeamManager {
    teams: HashMap<String, Vec<String>>, // Team name -> List of container names
}

impl TeamManager {
    pub fn new() -> Self {
        TeamManager {
            teams: HashMap::new(),
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
    pub fn add_to_team(&mut self, team_name: &str, container_name: &str, containers: &HashMap<String, Container>) {
        if let Some(team) = self.teams.get_mut(team_name) {
            if containers.contains_key(container_name) {
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
    pub fn team_info(&self, team_name: &str, containers: &HashMap<String, Container>) {
        if let Some(team) = self.teams.get(team_name) {
            println!("🌟 Team {}:", team_name);
            for container_name in team {
                if let Some(container) = containers.get(container_name) {
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
