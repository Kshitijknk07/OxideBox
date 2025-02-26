use rand::Rng;
use crate::container::Container;
use crate::evolution::EvolutionManager;

pub struct Battle;

impl Battle {
    pub fn calculate_exp_reward(opponent_level: u32) -> u32 {
        // Base experience calculation formula
        (opponent_level * 10) + 100
    }

    pub fn start_battle(pokemon1: &mut Container, pokemon2: &mut Container, evolution_manager: &EvolutionManager) {
        println!("⚔️ Battle start: {} vs {}", pokemon1.name, pokemon2.name);
        
        let mut rng = rand::thread_rng();
        
        while pokemon1.is_active() && pokemon2.is_active() {
            // Pokemon 1's turn
            if pokemon1.is_active() {
                let move_index = rng.gen_range(0..pokemon1.moves.len());
                Self::execute_move(pokemon1, pokemon2, move_index);
            }

            // Pokemon 2's turn
            if pokemon2.is_active() {
                let move_index = rng.gen_range(0..pokemon2.moves.len());
                Self::execute_move(pokemon2, pokemon1, move_index);
            }
        }

        // Award experience to the winner
        if pokemon1.is_active() {
            println!("🏆 {} wins!", pokemon1.name);
            pokemon1.gain_exp(Self::calculate_exp_reward(pokemon2.level), evolution_manager);
        } else {
            println!("🏆 {} wins!", pokemon2.name);
            pokemon2.gain_exp(Self::calculate_exp_reward(pokemon1.level), evolution_manager);
        }
    }

    fn execute_move(attacker: &mut Container, defender: &mut Container, move_index: usize) {
        if let Some(battle_move) = attacker.moves.get(move_index) {
            println!("💫 {} uses {}!", attacker.name, battle_move.name);
            // Implement damage calculation and HP reduction here
            let damage = battle_move.power;
            defender.hp -= damage as i32;
            println!("💥 {} takes {} damage!", defender.name, damage);
        }
    }
}
