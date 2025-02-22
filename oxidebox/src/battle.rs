use crate::container::Container;
use rand::Rng;

pub struct Battle;

impl Battle {
    pub fn start_battle(pokemon1: &mut Container, pokemon2: &mut Container) {
        println!("⚔️ Battle begins between {} and {}!", pokemon1.name, pokemon2.name);

        // Determine who attacks first based on speed
        let (first, second) = if pokemon1.speed >= pokemon2.speed {
            (pokemon1, pokemon2)
        } else {
            (pokemon2, pokemon1)
        };

        while first.is_active() && second.is_active() {
            Battle::attack(first, second);
            if second.is_active() {
                Battle::attack(second, first);
            }
        }

        // Declare the winner
        if first.is_active() {
            println!("🏆 {} wins the battle!", first.name);
        } else {
            println!("🏆 {} wins the battle!", second.name);
        }
    }

    fn attack(attacker: &mut Container, defender: &mut Container) {
        let mut rng = rand::thread_rng();
        let random_factor: f32 = rng.gen_range(0.8..1.2); // Random multiplier for variability

        let base_damage = ((attacker.attack as f32 * random_factor) - (defender.defense as f32 / 2.0)) as i32;
        let damage = if base_damage > 0 { base_damage } else { 1 }; // Ensure minimum damage

        println!("⚡ {} attacks {}!", attacker.name, defender.name);
        defender.take_damage(damage);
    }
}
