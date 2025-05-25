use crate::container::Container;
use crate::evolution::EvolutionManager;
use crate::stats::BattleRecord;
use chrono::Utc;
use rand::Rng;

pub struct Battle;

impl Battle {
    #[allow(dead_code)]
    pub fn calculate_exp_reward(opponent_level: u32) -> u32 {
        (opponent_level * 10) + 100
    }

    pub fn start_battle(
        pokemon1: &mut Container,
        pokemon2: &mut Container,
        _evolution_managerr: &EvolutionManager,
    ) {
        let mut turns = 0;
        println!(
            "{}",
            "╔══════════════════════════════════════╗".bright_red()
        );
        println!(
            "{}",
            format!(
                "║ ⚔️  Battle: {} vs {} ║",
                pokemon1.name.bright_yellow(),
                pokemon2.name.bright_yellow()
            )
            .bright_red()
            .bold()
        );
        println!(
            "{}",
            "╠══════════════════════════════════════╣".bright_red()
        );

        let mut rng = rand::thread_rng();

        while pokemon1.is_active() && pokemon2.is_active() {
            turns += 1;

            if pokemon1.is_active() {
                let move_index = rng.gen_range(0..pokemon1.moves.len());
                Self::execute_move(pokemon1, pokemon2, move_index);
            }

            if pokemon2.is_active() {
                let move_index = rng.gen_range(0..pokemon2.moves.len());
                Self::execute_move(pokemon2, pokemon1, move_index);
            }
        }

        let (winner, loser) = if pokemon1.is_active() {
            pokemon1.stats.battles_won += 1;
            pokemon2.stats.battles_lost += 1;
            (pokemon1, pokemon2)
        } else {
            pokemon2.stats.battles_won += 1;
            pokemon1.stats.battles_lost += 1;
            (pokemon2, pokemon1)
        };

        let _battle_record = BattleRecord {
            winner: winner.name.clone(),
            loser: loser.name.clone(),
            date: Utc::now(),
            turns,
        };

        println!(
            "{}",
            format!(
                "║ 🏆 Winner: {} in {} turns! ║",
                winner.name.bright_green().bold(),
                turns
            )
            .bright_green()
            .bold()
        );
        println!(
            "{}",
            "╚══════════════════════════════════════╝".bright_red()
        );
    }

    fn execute_move(attacker: &mut Container, defender: &mut Container, move_index: usize) {
        if let Some(battle_move) = attacker.moves.get(move_index) {
            let count = attacker
                .stats
                .moves_used
                .entry(battle_move.name.clone())
                .or_insert(0);
            *count += 1;

            println!("💫 {} uses {}!", attacker.name, battle_move.name);
            let damage = battle_move.power;
            defender.hp -= damage as i32;

            attacker.stats.total_damage_dealt += damage;
            defender.stats.total_damage_taken += damage;

            println!("💥 {} takes {} damage!", defender.name, damage);
        }
    }
}
