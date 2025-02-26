use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleRecord {
    pub winner: String,
    pub loser: String,
    pub date: DateTime<Utc>,
    pub turns: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonStats {
    pub battles_won: u32,
    pub battles_lost: u32,
    pub total_damage_dealt: u32,
    pub total_damage_taken: u32,
    pub evolution_count: u32,
    pub levels_gained: u32,
    pub moves_used: HashMap<String, u32>,
    pub creation_date: DateTime<Utc>,
    pub total_exp_gained: u32,
}

#[derive(Debug)]
pub struct TrainerStats {
    pub pokemon_stats: HashMap<String, PokemonStats>,
    pub battle_history: Vec<BattleRecord>,
    pub total_pokemon_caught: u32,
    pub total_pokemon_released: u32,
    pub start_date: DateTime<Utc>,
    pub total_battles: u32,
}

impl TrainerStats {
    pub fn new() -> Self {
        Self {
            pokemon_stats: HashMap::new(),
            battle_history: Vec::new(),
            total_pokemon_caught: 0,
            total_pokemon_released: 0,
            start_date: Utc::now(),
            total_battles: 0,
        }
    }

    pub fn display_detailed_stats(&self) {
        println!("\n🌟 Detailed Trainer Statistics 🌟\n");
        println!("👤 Trainer Overview");
        println!("-------------------");
        println!("🎮 Journey Started: {}", self.start_date.format("%Y-%m-%d %H:%M:%S UTC"));
        println!("📊 Total Battles: {}", self.total_battles);
        println!("✨ Total Pokémon Caught: {}", self.total_pokemon_caught);
        println!("🌿 Total Pokémon Released: {}", self.total_pokemon_released);
    }
}