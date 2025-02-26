use crate::container::Container;
use rand::Rng;

pub struct Battle;

impl Battle {
    pub fn start_battle(pokemon1: &mut Container, pokemon2: &mut Container) {
        println!("⚔️ Battle begins between {} and {}!", pokemon1.name, pokemon2.name);

        while pokemon1.is_active() && pokemon2.is_active() {
            Battle::turn(pokemon1, pokemon2);
            if pokemon2.is_active() {
                Battle::turn(pokemon2, pokemon1);
            }
        }

        let winner = if pokemon1.is_active() { &pokemon1.name } else { &pokemon2.name };
        println!("🏆 {} wins the battle!", winner);
    }

    fn turn(attacker: &mut Container, defender: &mut Container) {
        println!("\n{}'s turn!", attacker.name);
        println!("Available moves:");
        for (i, move_) in attacker.moves.iter().enumerate() {
            println!("{}. {} (PP: {}/{})", i + 1, move_.name, move_.pp, move_.max_pp);
        }

        let mut rng = rand::thread_rng();
        let move_index = if !attacker.moves.is_empty() {
            rng.gen_range(0..attacker.moves.len())
        } else {
            0 // Use default attack if no moves are available
        };

        attacker.use_move(move_index, defender);
    }
}
