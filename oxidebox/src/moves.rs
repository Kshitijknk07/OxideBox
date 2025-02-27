use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PokemonType {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
}

#[derive(Debug, Clone)]
pub struct Move {
    pub name: String,
    pub move_type: PokemonType,
    pub power: u32,
    pub accuracy: u32,
    pub pp: u32,
    pub max_pp: u32,
}

impl Move {
    pub fn new(name: &str, move_type: PokemonType, power: u32, accuracy: u32, max_pp: u32) -> Self {
        Move {
            name: name.to_string(),
            move_type,
            power,
            accuracy,
            pp: max_pp,
            max_pp,
        }
    }

    pub fn use_move(&mut self) -> bool {
        if self.pp > 0 {
            self.pp -= 1;
            true
        } else {
            println!("❌ No PP left for {}!", self.name);
            false
        }
    }

    pub fn restore_pp(&mut self) {
        self.pp = self.max_pp;
    }
}

pub struct TypeEffectiveness {
    effectiveness: HashMap<(PokemonType, PokemonType), f32>,
}

impl TypeEffectiveness {
    pub fn new() -> Self {
        let mut effectiveness = HashMap::new();
        
        
        effectiveness.insert((PokemonType::Normal, PokemonType::Rock), 0.5);
        effectiveness.insert((PokemonType::Normal, PokemonType::Ghost), 0.0);
        
        
        effectiveness.insert((PokemonType::Fire, PokemonType::Fire), 0.5);
        effectiveness.insert((PokemonType::Fire, PokemonType::Water), 0.5);
        effectiveness.insert((PokemonType::Fire, PokemonType::Grass), 2.0);
        effectiveness.insert((PokemonType::Fire, PokemonType::Ice), 2.0);
        
        
        effectiveness.insert((PokemonType::Water, PokemonType::Fire), 2.0);
        effectiveness.insert((PokemonType::Water, PokemonType::Water), 0.5);
        effectiveness.insert((PokemonType::Water, PokemonType::Grass), 0.5);
        
       

        TypeEffectiveness { effectiveness }
    }

    pub fn get_multiplier(&self, attacker_type: &PokemonType, defender_type: &PokemonType) -> f32 {
        *self.effectiveness.get(&(attacker_type.clone(), defender_type.clone()))
            .unwrap_or(&1.0)
    }
}