mod cli;
mod container;
mod evolution;
mod stats;
mod moves;
mod battle;
mod database;
mod team;
mod command;
mod challenge;

use crate::cli::{Cli, Commands};
use crate::container::ContainerManager;
use crate::evolution::EvolutionManager;
use crate::challenge::ChallengeManager;
use clap::Parser;
use colored::*;

fn main() {
    let cli = Cli::parse();
    let mut container_manager = ContainerManager::new();
    let evolution_manager = EvolutionManager::new();
    let mut challenge_manager = ChallengeManager::new();
    challenge_manager.generate_daily_challenges();

    match cli.command {
        Commands::CreateNamespace { name } => {
            if container_manager.create_namespace(&name) {
                println!(
                    "{}",
                    format!("✨ Created namespace: {}", name).bright_green()
                );
            } else {
                println!(
                    "{}",
                    format!("⚠️ Namespace {} already exists!", name).bright_red()
                );
            }
        }
        Commands::DeleteNamespace { name } => {
            if container_manager.delete_namespace(&name) {
                println!(
                    "{}",
                    format!("🗑️ Deleted namespace: {}", name).bright_green()
                );
            } else {
                println!(
                    "{}",
                    format!("⚠️ Namespace {} not found!", name).bright_red()
                );
            }
        }
        Commands::Summon {
            namespace,
            name,
            level,
            hp,
            attack,
            defense,
            speed,
            pokemon_type,
        } => {
            if container_manager.summon(
                &namespace,
                &name,
                level,
                hp,
                attack,
                defense,
                speed,
                pokemon_type,
            ) {
                challenge_manager.update_challenge_progress(crate::challenge::ChallengeType::CatchPokemon, 1);
                println!(
                    "{}",
                    format!(
                        "🎉 Successfully summoned {}!\n\
                        ┌─────────────────────────────┐\n\
                        │ Name:   {:<18} │\n\
                        │ Level:  {:<18} │\n\
                        │ HP:     {:<18} │\n\
                        │ Type:   {:<18} │\n\
                        └─────────────────────────────┘",
                        name.bright_yellow().bold(),
                        name,
                        level,
                        hp,
                        pokemon_type.to_string().bright_magenta()
                    )
                    .bright_green()
                );
            } else {
                println!(
                    "{}",
                    format!("⚠️ Failed to summon Pokémon in namespace: {}", namespace).bright_red()
                );
            }
        }
        Commands::Start { id } => {
            if container_manager.start_container(&id) {
                println!("{}", format!("▶️ Started container: {}", id).bright_green());
            } else {
                println!("{}", format!("⚠️ Container {} not found!", id).bright_red());
            }
        }
        Commands::Stop { id } => {
            if container_manager.stop_container(&id) {
                println!("{}", format!("⏹️ Stopped container: {}", id).bright_green());
            } else {
                println!("{}", format!("⚠️ Container {} not found!", id).bright_red());
            }
        }
        Commands::Pause { id } => {
            if container_manager.pause_container(&id) {
                println!("{}", format!("⏸️ Paused container: {}", id).bright_green());
            } else {
                println!("{}", format!("⚠️ Container {} not found!", id).bright_red());
            }
        }
        Commands::List { namespace: _ } => {
            if let Err(e) = container::ContainerManager::list_all_from_db() {
                eprintln!("Error listing containers: {}", e);
            }
        }
        Commands::Status { id } => {
            if let Some(container) = container_manager.get_container(&id) {
                container.display_status();
            } else {
                println!("{}", format!("⚠️ Container {} not found!", id).bright_red());
            }
        }
        Commands::Battle { id1, id2 } => {
            if container_manager.battle(&id1, &id2, &evolution_manager) {
                challenge_manager.update_challenge_progress(crate::challenge::ChallengeType::BattleWin, 1);
                challenge_manager.update_challenge_progress(crate::challenge::ChallengeType::UseMoves, 2);
                println!("{}", "⚔️ Battle completed!".bright_green());
            } else {
                println!("{}", "⚠️ Battle failed!".bright_red());
            }
        }
        Commands::Save { id } => match container_manager.save_to_db(&id) {
            Ok(_) => println!("{}", format!("💾 Saved container: {}", id).bright_green()),
            Err(e) => println!(
                "{}",
                format!("⚠️ Failed to save container: {}", e).bright_red()
            ),
        },
        Commands::Load { id } => match container_manager.load_from_db(&id) {
            Ok(_) => println!("{}", format!("📥 Loaded container: {}", id).bright_green()),
            Err(e) => println!(
                "{}",
                format!("⚠️ Failed to load container: {}", e).bright_red()
            ),
        },
        Commands::Stats => {
            container_manager.display_stats();
        }
        Commands::Challenges => {
            challenge_manager.check_daily_reset();
            challenge_manager.display_challenges();
        }
        Commands::ClaimReward { challenge_id } => {
            if let Some(challenge) = challenge_manager.active_challenges.get(&challenge_id) {
                if challenge.completed {
                    println!("{}", format!("🎁 Claimed rewards for challenge: {}", challenge.description).bright_green());
                    println!("{}", format!("Received: {} EXP, {}", 
                        challenge.reward_exp,
                        challenge.reward_items.join(", ")
                    ).bright_yellow());
                    container_manager.trainer_stats.total_exp_gained += challenge.reward_exp;
                } else {
                    println!("{}", "⚠️ Challenge not completed yet!".bright_red());
                }
            } else {
                println!("{}", "⚠️ Challenge not found!".bright_red());
            }
        }
    }
}
