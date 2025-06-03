use crate::container::{Container, ContainerManager};
use std::collections::HashMap;

pub struct TeamManager {
    teams: HashMap<String, Vec<String>>,
}

pub struct TeamStats {
    pub battles_won: u32,
    pub battles_lost: u32,
    pub total_exp: u32,
    pub average_level: f32,
}

impl TeamManager {
    pub fn new() -> Self {
        TeamManager {
            teams: HashMap::new(),
        }
    }

    pub fn create_team(&mut self, team_name: &str) -> bool {
        if self.teams.contains_key(team_name) {
            println!("⚠️ Team {} already exists!", team_name);
            false
        } else {
            self.teams.insert(team_name.to_string(), Vec::new());
            println!("🌟 Created new team: {}", team_name);
            true
        }
    }

    pub fn add_to_team(
        &mut self,
        team_name: &str,
        container_name: &str,
        containers: &HashMap<String, Container>,
    ) -> bool {
        if let Some(team) = self.teams.get_mut(team_name) {
            if containers.contains_key(container_name) {
                team.push(container_name.to_string());
                println!("➕ Added {} to team {}", container_name, team_name);
                true
            } else {
                println!("⚠️ Pokémon {} not found!", container_name);
                false
            }
        } else {
            println!("⚠️ Team {} not found!", team_name);
            false
        }
    }

    pub fn remove_from_team(&mut self, team_name: &str, container_name: &str) -> bool {
        if let Some(team) = self.teams.get_mut(team_name) {
            if let Some(index) = team.iter().position(|name| name == container_name) {
                team.remove(index);
                println!("➖ Removed {} from team {}", container_name, team_name);
                true
            } else {
                println!(
                    "⚠️ Pokémon {} not found in team {}!",
                    container_name, team_name
                );
                false
            }
        } else {
            println!("⚠️ Team {} not found!", team_name);
            false
        }
    }

    pub fn team_info(&self, team_name: &str, containers: &HashMap<String, Container>) {
        if let Some(team) = self.teams.get(team_name) {
            println!("🌟 Team {}:", team_name);
            for container_name in team {
                if let Some(container) = containers.get(container_name) {
                    println!(
                        "Pokémon: {} | Level: {} | HP: {} | Status: {:?}",
                        container.name, container.level, container.hp, container.stats
                    );
                } else {
                    println!("⚠️ Pokémon {} not found!", container_name);
                }
            }
        } else {
            println!("⚠️ Team {} not found!", team_name);
        }
    }

    pub fn calculate_team_stats(&self, containers: &HashMap<String, Container>) -> TeamStats {
        let mut stats = TeamStats {
            battles_won: 0,
            battles_lost: 0,
            total_exp: 0,
            average_level: 0.0,
        };

        for (_, team) in &self.teams {
            let mut total_level = 0;
            for container_name in team {
                if let Some(container) = containers.get(container_name) {
                    stats.total_exp += container.exp;
                    total_level += container.level;
                }
            }
            if !team.is_empty() {
                stats.average_level = total_level as f32 / team.len() as f32;
            }
        }
        stats
    }
}
