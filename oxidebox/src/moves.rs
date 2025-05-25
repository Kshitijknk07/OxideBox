use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

impl PokemonType {
    pub fn all_types() -> Vec<Self> {
        vec![
            Self::Normal,
            Self::Fire,
            Self::Water,
            Self::Electric,
            Self::Grass,
            Self::Ice,
            Self::Fighting,
            Self::Poison,
            Self::Ground,
            Self::Flying,
            Self::Psychic,
            Self::Bug,
            Self::Rock,
            Self::Ghost,
            Self::Dragon,
            Self::Dark,
            Self::Steel,
            Self::Fairy,
        ]
    }
}

impl fmt::Display for PokemonType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for PokemonType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::all_types()
            .into_iter()
            .find(|t| t.to_string().to_lowercase() == s.to_lowercase())
            .ok_or_else(|| format!("Invalid Pokemon type: {}", s))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub name: String,
    pub power: u32,
    pub accuracy: u8,
    pub pp: u8,
    pub max_pp: u8,
    pub pokemon_type: PokemonType,
    pub category: MoveCategory,
    pub description: String,
    pub effect: Option<MoveEffect>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MoveCategory {
    Physical,
    Special,
    Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveEffect {
    pub effect_type: EffectType,
    pub chance: u8,
    pub turns: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    StatChange { stat: Stat, stages: i8 },
    StatusCondition(StatusCondition),
    WeatherChange(Weather),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stat {
    Attack,
    Defense,
    SpecialAttack,
    SpecialDefense,
    Speed,
    Accuracy,
    Evasion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatusCondition {
    Burn,
    Freeze,
    Paralysis,
    Poison,
    Sleep,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Weather {
    Sunny,
    Rain,
    Sandstorm,
    Hail,
}

impl Move {
    pub fn new(
        name: &str,
        power: u32,
        accuracy: u8,
        pp: u8,
        pokemon_type: PokemonType,
        category: MoveCategory,
        description: &str,
        effect: Option<MoveEffect>,
    ) -> Self {
        Self {
            name: name.to_string(),
            power,
            accuracy: accuracy.clamp(0, 100),
            pp: pp.clamp(0, 64),
            max_pp: pp.clamp(0, 64),
            pokemon_type,
            category,
            description: description.to_string(),
            effect,
        }
    }

    #[allow(dead_code)]
    pub fn use_move(&mut self) -> Result<(), String> {
        if self.pp == 0 {
            return Err(format!("No PP left for {}!", self.name));
        }
        self.pp -= 1;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn restore_pp(&mut self, amount: u8) {
        self.pp = (self.pp + amount).min(self.max_pp);
    }

    #[allow(dead_code)]
    pub fn is_usable(&self) -> bool {
        self.pp > 0
    }
}

#[derive(Debug, Clone)]
pub struct TypeEffectiveness {
    #[allow(dead_code)]
    effectiveness: HashMap<(PokemonType, PokemonType), f32>,
}

impl Default for TypeEffectiveness {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeEffectiveness {
    pub fn new() -> Self {
        let mut effectiveness = HashMap::new();

        // Normal type
        effectiveness.insert((PokemonType::Normal, PokemonType::Rock), 0.5);
        effectiveness.insert((PokemonType::Normal, PokemonType::Ghost), 0.0);
        effectiveness.insert((PokemonType::Normal, PokemonType::Steel), 0.5);

        // Fire type
        effectiveness.insert((PokemonType::Fire, PokemonType::Fire), 0.5);
        effectiveness.insert((PokemonType::Fire, PokemonType::Water), 0.5);
        effectiveness.insert((PokemonType::Fire, PokemonType::Grass), 2.0);
        effectiveness.insert((PokemonType::Fire, PokemonType::Ice), 2.0);
        effectiveness.insert((PokemonType::Fire, PokemonType::Bug), 2.0);
        effectiveness.insert((PokemonType::Fire, PokemonType::Steel), 2.0);
        effectiveness.insert((PokemonType::Fire, PokemonType::Rock), 0.5);

        // Water type
        effectiveness.insert((PokemonType::Water, PokemonType::Fire), 2.0);
        effectiveness.insert((PokemonType::Water, PokemonType::Water), 0.5);
        effectiveness.insert((PokemonType::Water, PokemonType::Grass), 0.5);
        effectiveness.insert((PokemonType::Water, PokemonType::Ground), 2.0);
        effectiveness.insert((PokemonType::Water, PokemonType::Rock), 2.0);

        Self { effectiveness }
    }

    #[allow(dead_code)]
    pub fn get_multiplier(&self, attacker_type: PokemonType, defender_type: PokemonType) -> f32 {
        self.effectiveness
            .get(&(attacker_type, defender_type))
            .copied()
            .unwrap_or(1.0)
    }
}
