use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use colored::*;

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

impl PokemonStats {
    pub fn new() -> Self {
        Self {
            battles_won: 0,
            battles_lost: 0,
            total_damage_dealt: 0,
            total_damage_taken: 0,
            evolution_count: 0,
            levels_gained: 0,
            moves_used: HashMap::new(),
            creation_date: Utc::now(),
            total_exp_gained: 0,
        }
    }
}

#[derive(Debug)]
pub struct TimeBasedStats {
    pub daily_catches: u32,
    pub weekly_wins: u32,
    pub monthly_exp: u32,
}

#[derive(Debug)]
pub struct TrainerStats {
    pub pokemon_stats: HashMap<String, PokemonStats>,
    pub battle_history: Vec<BattleRecord>,
    pub total_pokemon_caught: u32,
    pub total_pokemon_released: u32,
    pub start_date: DateTime<Utc>,
    pub total_battles: u32,
    pub total_wins: u32,
    pub total_losses: u32,
    pub favorite_pokemon_type: String,
    pub most_used_move: String,
    pub total_exp_gained: u32,
    pub time_based_stats: TimeBasedStats,
    pub daily_catches: u32,
    pub weekly_wins: u32,
    pub monthly_exp: u32,
}

impl TrainerStats {
    pub fn new() -> Self {
        Self {
            time_based_stats: TimeBasedStats {
                daily_catches: 0,
                weekly_wins: 0,
                monthly_exp: 0,
            },
            daily_catches: 0,
            weekly_wins: 0,
            monthly_exp: 0,
            pokemon_stats: HashMap::new(),
            battle_history: Vec::new(),
            total_pokemon_caught: 0,
            total_pokemon_released: 0,
            start_date: Utc::now(),
            total_battles: 0,
            total_wins: 0,
            total_losses: 0,
            favorite_pokemon_type: "Normal".to_string(),
            most_used_move: "Tackle".to_string(),
            total_exp_gained: 0,
        }
    }

    pub fn display_detailed_stats(&self) {
        println!("{}", "=== Trainer Statistics ===".bright_cyan());
        println!("{}: {}", "Total Pokemon Caught".bright_green(), self.total_pokemon_caught);
        println!("{}: {}", "Total Pokemon Released".bright_green(), self.total_pokemon_released);
        println!("{}: {}", "Total Battles".bright_green(), self.total_battles);
        println!("{}: {}", "Total Wins".bright_green(), self.total_wins);
        println!("{}: {}", "Total Losses".bright_green(), self.total_losses);
        println!("{}: {}", "Win Rate".bright_green(), 
            if self.total_battles > 0 {
                format!("{:.1}%", (self.total_wins as f64 / self.total_battles as f64) * 100.0)
            } else {
                "0.0%".to_string()
            }
        );
        println!("{}: {}", "Favorite Pokemon Type".bright_green(), self.favorite_pokemon_type);
        println!("{}: {}", "Most Used Move".bright_green(), self.most_used_move);
        println!("{}: {}", "Total EXP Gained".bright_green(), self.total_exp_gained);
        println!("{}", "=====================".bright_cyan());
    }

    pub fn add_daily_catch(&mut self) {
        self.daily_catches += 1;
    }

    pub fn add_weekly_win(&mut self) {
        self.weekly_wins += 1;
    }

    pub fn add_monthly_exp(&mut self, exp: u32) {
        self.monthly_exp += exp;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_trainer_stats() {
        let stats = TrainerStats::new();
        assert_eq!(stats.total_battles, 0);
        assert_eq!(stats.total_wins, 0);
        assert_eq!(stats.total_losses, 0);
    }
}