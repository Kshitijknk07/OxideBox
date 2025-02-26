use rusqlite::{Connection, Result, OptionalExtension};
use rusqlite::params;
use crate::container::Container;
use crate::moves::{Move, PokemonType};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("pokemon.db")?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS pokemon (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                level INTEGER,
                hp INTEGER,
                attack INTEGER,
                defense INTEGER,
                speed INTEGER,
                pokemon_type TEXT,
                status TEXT,
                exp INTEGER,
                exp_to_next_level INTEGER
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS moves (
                id INTEGER PRIMARY KEY,
                pokemon_id INTEGER,
                name TEXT NOT NULL,
                move_type TEXT,
                power INTEGER,
                accuracy INTEGER,
                pp INTEGER,
                max_pp INTEGER,
                FOREIGN KEY(pokemon_id) REFERENCES pokemon(id)
            )",
            [],
        )?;

        Ok(Database { conn })
    }

    pub fn save_pokemon(&mut self, container: &Container) -> Result<()> {
        let tx = self.conn.transaction()?;
        
        tx.execute(
            "INSERT INTO pokemon (name, level, hp, attack, defense, speed, pokemon_type, status, exp, exp_to_next_level)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                container.name,
                container.level,
                container.hp,
                container.attack,
                container.defense,
                container.speed,
                format!("{:?}", container.pokemon_type),
                container.status,
                container.exp,
                container.exp_to_next_level,
            ],
        )?;

        let pokemon_id = tx.last_insert_rowid();

        // Save moves
        for move_ in &container.moves {
            tx.execute(
                "INSERT INTO moves (pokemon_id, name, move_type, power, accuracy, pp, max_pp)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    pokemon_id,
                    move_.name,
                    format!("{:?}", move_.move_type),
                    move_.power,
                    move_.accuracy,
                    move_.pp,
                    move_.max_pp,
                ],
            )?;
        }

        tx.commit()?;
        println!("💾 Saved {} to database!", container.name);
        Ok(())
    }

    pub fn load_pokemon(&self, name: &str) -> Result<Option<Container>> {
        let mut stmt = self.conn.prepare(
            "SELECT level, hp, attack, defense, speed, pokemon_type, status 
             FROM pokemon WHERE name = ?1"
        )?;

        let pokemon = stmt.query_row(params![name], |row| {
            let pokemon_type: String = row.get(5)?;
            let container = Container::new(
                name,
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                // Parse pokemon_type string back to enum
                match pokemon_type.as_str() {
                    "Fire" => PokemonType::Fire,
                    "Water" => PokemonType::Water,
                    "Electric" => PokemonType::Electric,
                    // Add other types...
                    _ => PokemonType::Normal,
                }
            );
            Ok(container)
        }).optional()?;

        if let Some(mut container) = pokemon {
            // Load moves
            let mut stmt = self.conn.prepare(
                "SELECT name, move_type, power, accuracy, pp, max_pp 
                 FROM moves m 
                 JOIN pokemon p ON m.pokemon_id = p.id 
                 WHERE p.name = ?1"
            )?;

            let moves = stmt.query_map(params![name], |row| {
                let move_type: String = row.get(1)?;
                Ok(Move::new(
                    &row.get::<_, String>(0)?,
                    match move_type.as_str() {
                        "Fire" => PokemonType::Fire,
                        "Water" => PokemonType::Water,
                        "Electric" => PokemonType::Electric,
                        // Add other types...
                        _ => PokemonType::Normal,
                    },
                    row.get(2)?,
                    row.get(3)?,
                    row.get(5)?
                ))
            })?;

            for move_result in moves {
                container.learn_move(move_result?);
            }

            println!("📂 Loaded {} from database!", name);
            Ok(Some(container))
        } else {
            Ok(None)
        }
    }
}