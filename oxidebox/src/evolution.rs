use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Evolution {
    #[allow(dead_code)]
    pub from: String,
    #[allow(dead_code)]
    pub to: String,
    #[allow(dead_code)]
    pub level: u32,
    #[allow(dead_code)]
    pub stat_multipliers: StatMultipliers,
}

#[derive(Debug, Clone)]
pub struct StatMultipliers {
    #[allow(dead_code)]
    pub hp: f32,
    #[allow(dead_code)]
    pub attack: f32,
    #[allow(dead_code)]
    pub defense: f32,
    #[allow(dead_code)]
    pub speed: f32,
}

pub struct EvolutionManager {
    #[allow(dead_code)]
    evolutions: HashMap<String, Evolution>,
}

impl EvolutionManager {
    pub fn new() -> Self {
        let mut evolutions = HashMap::new();

        evolutions.insert(
            "Pikachu".to_string(),
            Evolution {
                from: "Pikachu".to_string(),
                to: "Raichu".to_string(),
                level: 25,
                stat_multipliers: StatMultipliers {
                    hp: 1.4,
                    attack: 1.5,
                    defense: 1.3,
                    speed: 1.4,
                },
            },
        );

        EvolutionManager { evolutions }
    }

    #[allow(dead_code)]
    pub fn get_evolution(&self, pokemon_name: &str) -> Option<&Evolution> {
        self.evolutions.get(pokemon_name)
    }
}
