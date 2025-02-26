use crate::moves::PokemonType;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Evolution {
    pub from: String,
    pub to: String,
    pub level: u32,
    pub stat_multipliers: StatMultipliers,
}

#[derive(Debug, Clone)]
pub struct StatMultipliers {
    pub hp: f32,
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
}

pub struct EvolutionManager {
    evolutions: HashMap<String, Evolution>,
}

impl EvolutionManager {
    pub fn new() -> Self {
        let mut evolutions = HashMap::new();
        
        // Define evolution chains
        evolutions.insert("Pikachu".to_string(), Evolution {
            from: "Pikachu".to_string(),
            to: "Raichu".to_string(),
            level: 25,
            stat_multipliers: StatMultipliers {
                hp: 1.4,
                attack: 1.5,
                defense: 1.3,
                speed: 1.4,
            },
        });

        // Add more evolution chains here
        
        EvolutionManager { evolutions }
    }

    pub fn get_evolution(&self, pokemon_name: &str) -> Option<&Evolution> {
        self.evolutions.get(pokemon_name)
    }
}