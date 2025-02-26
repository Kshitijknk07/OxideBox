use crate::container::Container;
use rand::Rng;

pub struct Battle;

impl Battle {
    pub fn start_battle(pokemon1: &mut Container, pokemon2: &mut Container) {
        println!("⚔️ Battle begins between {} and {}!", pokemon1.name, pokemon2.name);

        while pokemon1.is_active() && pokemon2.is_active() {
            Battle::attack(pokemon1, pokemon2);
            if pokemon2.is_active() {
                Battle::attack(pokemon2, pokemon1);
            }
        }

        let winner = if pokemon1.is_active() { &pokemon1.name } else { &pokemon2.name };
        println!("🏆 {} wins the battle!", winner);
    }

    fn attack(attacker: &mut Container, defender: &mut Container) {
        let mut rng = rand::thread_rng();
        let random_factor: f32 = rng.gen_range(0.8..1.2);
        
        let base_damage = ((attacker.attack as f32 * random_factor) - (defender.defense as f32 / 2.0)) as i32;
        let damage = base_damage.max(1);

        println!("⚡ {} attacks {}!", attacker.name, defender.name);
        defender.take_damage(damage);
    }
}
