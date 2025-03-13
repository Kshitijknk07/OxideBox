use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::battle::Battle;
use crate::moves::{Move, PokemonType};
use crate::database::Database;
use rusqlite::Result;
use crate::evolution::EvolutionManager;
use crate::stats::PokemonStats;
use crate::stats::TrainerStats;
use colored::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ContainerState {
    Created,
    Running,
    Paused,
    Stopped,
    Failed,
    Evolved,
}

#[derive(Debug, Clone)]
pub struct ContainerResources {
    pub cpu_limit: f64,
    pub memory_limit: u64,
    pub storage_limit: u64,
    pub current_cpu: f64,
    pub current_memory: u64,
    pub current_storage: u64,
}

#[derive(Debug, Clone)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub state: ContainerState,
    pub level: u32,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: u32,
    pub defense: u32,
    pub speed: u32,
    pub pokemon_type: PokemonType,
    pub moves: Vec<Move>,
    pub exp: u32,
    pub exp_to_next_level: u32,
    pub stats: PokemonStats,
    pub resources: ContainerResources,
    pub created_at: SystemTime,
    pub namespace: String,
    pub labels: HashMap<String, String>,
}

impl Container {
    pub fn new(
        name: &str,
        namespace: &str,
        level: u32,
        hp: i32,
        attack: u32,
        defense: u32,
        speed: u32,
        pokemon_type: PokemonType,
    ) -> Self {
        let exp_to_next_level = Self::calculate_exp_to_next_level(level);
        let now = SystemTime::now();
        let id = format!("pokemon-{}-{}", name, now.duration_since(UNIX_EPOCH).unwrap().as_secs());
        
        let mut labels = HashMap::new();
        labels.insert("type".to_string(), pokemon_type.to_string());
        labels.insert("namespace".to_string(), namespace.to_string());
        
        Self {
            id,
            name: name.to_string(),
            state: ContainerState::Created,
            level,
            hp,
            max_hp: hp,
            attack,
            defense,
            speed,
            pokemon_type,
            moves: Vec::new(),
            exp: 0,
            exp_to_next_level,
            stats: PokemonStats::new(),
            resources: ContainerResources {
                cpu_limit: 1.0,
                memory_limit: 512 * 1024 * 1024,
                storage_limit: 1024 * 1024 * 1024,
                current_cpu: 0.0,
                current_memory: 0,
                current_storage: 0,
            },
            created_at: now,
            namespace: namespace.to_string(),
            labels,
        }
    }

    fn calculate_exp_to_next_level(level: u32) -> u32 {
        (level * level * 100) as u32
    }

    pub fn is_active(&self) -> bool {
        self.hp > 0 && self.state == ContainerState::Running
    }

    pub fn learn_move(&mut self, new_move: Move) -> bool {
        if self.moves.len() < 4 {
            self.moves.push(new_move);
            true
        } else {
            false
        }
    }

    pub fn update_resources(&mut self, cpu: f64, memory: u64, storage: u64) {
        self.resources.current_cpu = cpu.min(self.resources.cpu_limit);
        self.resources.current_memory = memory.min(self.resources.memory_limit);
        self.resources.current_storage = storage.min(self.resources.storage_limit);
    }

    pub fn display_status(&self) {
        println!("{}", "=== Pokemon Container Status ===".bright_cyan());
        println!("{}: {}", "ID".bright_green(), self.id);
        println!("{}: {}", "Name".bright_green(), self.name);
        println!("{}: {}", "Namespace".bright_green(), self.namespace);
        println!("{}: {}", "State".bright_green(), format!("{:?}", self.state).bright_yellow());
        println!("{}: {}", "Level".bright_green(), self.level);
        println!("{}: {}/{}", "HP".bright_green(), self.hp, self.max_hp);
        println!("{}: {}", "Type".bright_green(), self.pokemon_type.to_string().bright_magenta());
        println!("{}: {}", "Moves".bright_green(), self.moves.len());
        println!("{}: {}/{}", "EXP".bright_green(), self.exp, self.exp_to_next_level);
        println!("{}", "=== Resource Usage ===".bright_cyan());
        println!("{}: {:.1}%", "CPU".bright_green(), (self.resources.current_cpu / self.resources.cpu_limit) * 100.0);
        println!("{}: {:.1}%", "Memory".bright_green(), (self.resources.current_memory as f64 / self.resources.memory_limit as f64) * 100.0);
        println!("{}: {:.1}%", "Storage".bright_green(), (self.resources.current_storage as f64 / self.resources.storage_limit as f64) * 100.0);
        println!("{}", "=====================".bright_cyan());
    }
}

pub struct ContainerManager {
    containers: HashMap<String, Container>,
    namespaces: HashMap<String, Vec<String>>,
    pub trainer_stats: TrainerStats,
    db: Database,
}

impl ContainerManager {
    pub fn new() -> Self {
        let mut db = Database::new().expect("Failed to create database");
        let mut namespaces = HashMap::new();
        
        // Load existing namespaces from database
        if let Ok(existing_namespaces) = db.get_namespaces() {
            for namespace in existing_namespaces {
                namespaces.insert(namespace.clone(), Vec::new());
            }
        }

        ContainerManager {
            containers: HashMap::new(),
            namespaces,
            trainer_stats: TrainerStats::new(),
            db,
        }
    }

    pub fn create_namespace(&mut self, name: &str) -> bool {
        if self.db.create_namespace(name).unwrap_or(false) {
            self.namespaces.insert(name.to_string(), Vec::new());
            true
        } else {
            false
        }
    }

    pub fn delete_namespace(&mut self, name: &str) -> bool {
        if self.db.delete_namespace(name).unwrap_or(false) {
            self.namespaces.remove(name);
            true
        } else {
            false
        }
    }

    pub fn summon(&mut self, namespace: &str, name: &str, level: u8, hp: u16, attack: u16, defense: u16, speed: u16, pokemon_type: PokemonType) -> bool {
        if !self.namespaces.contains_key(namespace) {
            return false;
        }

        let container = Container::new(
            name,
            namespace,
            level as u32,
            hp as i32,
            attack as u32,
            defense as u32,
            speed as u32,
            pokemon_type,
        );
        
        // Save to database
        if let Err(_) = self.db.save_pokemon(&container) {
            return false;
        }

        // Update trainer stats
        self.trainer_stats.total_pokemon_caught += 1;
        
        true
    }

    pub fn start_container(&mut self, id: &str) -> bool {
        if let Some(container) = self.containers.get_mut(id) {
            container.state = ContainerState::Running;
            true
        } else {
            false
        }
    }

    pub fn stop_container(&mut self, id: &str) -> bool {
        if let Some(container) = self.containers.get_mut(id) {
            container.state = ContainerState::Stopped;
            true
        } else {
            false
        }
    }

    pub fn pause_container(&mut self, id: &str) -> bool {
        if let Some(container) = self.containers.get_mut(id) {
            container.state = ContainerState::Paused;
            true
        } else {
            false
        }
    }

    pub fn get_container(&self, id: &str) -> Option<&Container> {
        self.containers.get(id)
    }

    pub fn get_container_mut(&mut self, id: &str) -> Option<&mut Container> {
        self.containers.get_mut(id)
    }

    pub fn list_containers(&self, namespace: Option<&str>) {
        println!("{}", "=== Pokemon Containers ===".bright_cyan());
        for (id, container) in &self.containers {
            if let Some(ns) = namespace {
                if container.namespace != ns {
                    continue;
                }
            }
            println!("{}: {} ({})", 
                id.bright_green(),
                container.name.bright_yellow(),
                format!("{:?}", container.state).bright_magenta()
            );
        }
        println!("{}", "=====================".bright_cyan());
    }

    pub fn battle(&mut self, id1: &str, id2: &str, evolution_manager: &EvolutionManager) -> bool {
        if id1 == id2 {
            println!("{}", "⚠️ A Pokemon cannot battle itself!".bright_red());
            return false;
        }

        let p1 = self.containers.remove(id1);
        let p2 = self.containers.remove(id2);

        match (p1, p2) {
            (Some(mut p1), Some(mut p2)) => {
                if p1.state != ContainerState::Running || p2.state != ContainerState::Running {
                    println!("{}", "⚠️ Both Pokemon must be running to battle!".bright_red());
                    self.containers.insert(id1.to_string(), p1);
                    self.containers.insert(id2.to_string(), p2);
                    return false;
                }

                Battle::start_battle(&mut p1, &mut p2, evolution_manager);
                self.containers.insert(id1.to_string(), p1);
                self.containers.insert(id2.to_string(), p2);
                true
            }
            (Some(p1), None) => {
                println!("{}", "⚠️ Second Pokemon not found!".bright_red());
                self.containers.insert(id1.to_string(), p1);
                false
            }
            (None, Some(p2)) => {
                println!("{}", "⚠️ First Pokemon not found!".bright_red());
                self.containers.insert(id2.to_string(), p2);
                false
            }
            (None, None) => {
                println!("{}", "⚠️ Both Pokemon not found!".bright_red());
                false
            }
        }
    }

    pub fn save_to_db(&self, id: &str) -> Result<(), rusqlite::Error> {
        let mut db = Database::new()?;
        if let Some(pokemon) = self.containers.get(id) {
            db.save_pokemon(pokemon)?;
        }
        Ok(())
    }

    pub fn load_from_db(&mut self, id: &str) -> Result<(), rusqlite::Error> {
        let db = Database::new()?;
        if let Some(pokemon) = db.load_pokemon(id)? {
            self.containers.insert(id.to_string(), pokemon);
        }
        Ok(())
    }

    pub fn display_stats(&self) {
        self.trainer_stats.display_detailed_stats();
    }
}