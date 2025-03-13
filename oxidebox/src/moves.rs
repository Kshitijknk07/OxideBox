use std::fmt;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    Dark,
    Steel,
    Fairy,
}

impl fmt::Display for PokemonType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PokemonType::Normal => write!(f, "Normal"),
            PokemonType::Fire => write!(f, "Fire"),
            PokemonType::Water => write!(f, "Water"),
            PokemonType::Electric => write!(f, "Electric"),
            PokemonType::Grass => write!(f, "Grass"),
            PokemonType::Ice => write!(f, "Ice"),
            PokemonType::Fighting => write!(f, "Fighting"),
            PokemonType::Poison => write!(f, "Poison"),
            PokemonType::Ground => write!(f, "Ground"),
            PokemonType::Flying => write!(f, "Flying"),
            PokemonType::Psychic => write!(f, "Psychic"),
            PokemonType::Bug => write!(f, "Bug"),
            PokemonType::Rock => write!(f, "Rock"),
            PokemonType::Ghost => write!(f, "Ghost"),
            PokemonType::Dragon => write!(f, "Dragon"),
            PokemonType::Dark => write!(f, "Dark"),
            PokemonType::Steel => write!(f, "Steel"),
            PokemonType::Fairy => write!(f, "Fairy"),
        }
    }
}

impl FromStr for PokemonType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(PokemonType::Normal),
            "fire" => Ok(PokemonType::Fire),
            "water" => Ok(PokemonType::Water),
            "electric" => Ok(PokemonType::Electric),
            "grass" => Ok(PokemonType::Grass),
            "ice" => Ok(PokemonType::Ice),
            "fighting" => Ok(PokemonType::Fighting),
            "poison" => Ok(PokemonType::Poison),
            "ground" => Ok(PokemonType::Ground),
            "flying" => Ok(PokemonType::Flying),
            "psychic" => Ok(PokemonType::Psychic),
            "bug" => Ok(PokemonType::Bug),
            "rock" => Ok(PokemonType::Rock),
            "ghost" => Ok(PokemonType::Ghost),
            "dragon" => Ok(PokemonType::Dragon),
            "dark" => Ok(PokemonType::Dark),
            "steel" => Ok(PokemonType::Steel),
            "fairy" => Ok(PokemonType::Fairy),
            _ => Err(format!("Invalid Pokemon type: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Move {
    pub name: String,
    pub power: u32,
    pub accuracy: u32,
    pub pp: u32,
    pub max_pp: u32,
    pub pokemon_type: PokemonType,
    pub description: String,
}

impl Move {
    pub fn new(name: &str, power: u32, accuracy: u32, pp: u32, pokemon_type: PokemonType, description: &str) -> Self {
        Self {
            name: name.to_string(),
            power,
            accuracy,
            pp,
            max_pp: pp,
            pokemon_type,
            description: description.to_string(),
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