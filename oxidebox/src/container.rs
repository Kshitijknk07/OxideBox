use std::collections::HashMap;

#[derive(Debug)]
pub struct Container {
    pub name: String,
    pub status: String,  // "active", "stopped", "released"
}

pub struct ContainerManager {
    containers: HashMap<String, Container>,
}

impl ContainerManager {
    pub fn new() -> Self {
        ContainerManager {
            containers: HashMap::new(),
        }
    }

    pub fn summon(&mut self, name: &str) {
        let container = Container {
            name: name.to_string(),
            status: "active".to_string(),
        };
        self.containers.insert(name.to_string(), container);
        println!("⚡ Summoned Pokémon: {}", name);
    }

    pub fn recall(&mut self, name: &str) {
        if let Some(container) = self.containers.get_mut(name) {
            container.status = "stopped".to_string();
            println!("🛑 Recalling Pokémon: {} ({})", name, container.name);
        } else {
            println!("⚠️ Pokémon {} not found!", name);
        }
    }

    pub fn release(&mut self, name: &str) {
        if let Some(container) = self.containers.remove(name) {
            println!("🌿 Releasing Pokémon: {} ({}) back into the wild!", name, container.name);
        } else {
            println!("⚠️ Pokémon {} not found!", name);
        }
    }

    pub fn pokedex(&self) {
        println!("📖 Fetching Pokédex...");
        for (name, container) in &self.containers {
            println!("Pokémon: {} | Status: {}", name, container.status);
        }
    }
}