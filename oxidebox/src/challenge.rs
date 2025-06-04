use chrono::{DateTime, Utc};
use colored::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChallengeType {
    BattleWin,
    CatchPokemon,
    LevelUp,
    UseMoves,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub id: String,
    pub challenge_type: ChallengeType,
    pub description: String,
    pub target: u32,
    pub progress: u32,
    pub reward_exp: u32,
    pub reward_items: Vec<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct ChallengeManager {
    pub active_challenges: HashMap<String, Challenge>,
    pub completed_challenges: Vec<Challenge>,
    pub daily_reset_time: DateTime<Utc>,
}

impl ChallengeManager {
    pub fn new() -> Self {
        Self {
            active_challenges: HashMap::new(),
            completed_challenges: Vec::new(),
            daily_reset_time: Utc::now(),
        }
    }

    pub fn generate_daily_challenges(&mut self) {
        self.active_challenges.clear();
        
        // Generate 3 random daily challenges
        let challenges = vec![
            Challenge {
                id: format!("daily-{}", Utc::now().timestamp()),
                challenge_type: ChallengeType::BattleWin,
                description: "Win 3 battles today".to_string(),
                target: 3,
                progress: 0,
                reward_exp: 500,
                reward_items: vec!["Rare Candy".to_string()],
                completed: false,
                created_at: Utc::now(),
                expires_at: Utc::now() + chrono::Duration::days(1),
            },
            Challenge {
                id: format!("daily-{}", Utc::now().timestamp() + 1),
                challenge_type: ChallengeType::CatchPokemon,
                description: "Catch 2 Pokémon today".to_string(),
                target: 2,
                progress: 0,
                reward_exp: 300,
                reward_items: vec!["Poké Ball".to_string(), "Great Ball".to_string()],
                completed: false,
                created_at: Utc::now(),
                expires_at: Utc::now() + chrono::Duration::days(1),
            },
            Challenge {
                id: format!("daily-{}", Utc::now().timestamp() + 2),
                challenge_type: ChallengeType::UseMoves,
                description: "Use 10 moves in battles".to_string(),
                target: 10,
                progress: 0,
                reward_exp: 200,
                reward_items: vec!["PP Up".to_string()],
                completed: false,
                created_at: Utc::now(),
                expires_at: Utc::now() + chrono::Duration::days(1),
            },
        ];

        for challenge in challenges {
            self.active_challenges.insert(challenge.id.clone(), challenge);
        }
    }

    pub fn update_challenge_progress(&mut self, challenge_type: ChallengeType, amount: u32) {
        for (_, challenge) in self.active_challenges.iter_mut() {
            if challenge.challenge_type == challenge_type && !challenge.completed {
                challenge.progress += amount;
                if challenge.progress >= challenge.target {
                    challenge.completed = true;
                    self.completed_challenges.push(challenge.clone());
                    println!("{}", format!("🎉 Completed daily challenge: {}", challenge.description).bright_green());
                    println!("{}", format!("Rewards: {} EXP, {}", 
                        challenge.reward_exp,
                        challenge.reward_items.join(", ")
                    ).bright_yellow());
                }
            }
        }
    }

    pub fn display_challenges(&self) {
        println!("{}", "=== Daily Challenges ===".bright_cyan());
        for (_, challenge) in &self.active_challenges {
            let progress_bar = format!("[{}{}]",
                "=".repeat((challenge.progress as f32 / challenge.target as f32 * 20.0) as usize),
                " ".repeat(20 - (challenge.progress as f32 / challenge.target as f32 * 20.0) as usize)
            );
            println!("{}", format!("{} ({}/{})", challenge.description, challenge.progress, challenge.target).bright_white());
            println!("{}", progress_bar.bright_green());
            println!("{}", format!("Rewards: {} EXP, {}", 
                challenge.reward_exp,
                challenge.reward_items.join(", ")
            ).bright_yellow());
            println!("{}", "-------------------".bright_cyan());
        }
    }

    pub fn check_daily_reset(&mut self) {
        if Utc::now() > self.daily_reset_time {
            self.generate_daily_challenges();
            self.daily_reset_time = Utc::now() + chrono::Duration::days(1);
        }
    }
} 