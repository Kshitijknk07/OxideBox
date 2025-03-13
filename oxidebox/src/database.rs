use crate::container::Container;
use crate::moves::{Move, PokemonType};
use rusqlite::params;
use rusqlite::{Connection, OptionalExtension, Result};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("pokemon.db")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS namespaces (
                name TEXT PRIMARY KEY
            )",
            [],
        )?;

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
                exp_to_next_level INTEGER,
                namespace TEXT,
                created_at INTEGER,
                FOREIGN KEY(namespace) REFERENCES namespaces(name)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS moves (
                id INTEGER PRIMARY KEY,
                pokemon_id INTEGER,
                name TEXT NOT NULL,
                pokemon_type TEXT,
                power INTEGER,
                accuracy INTEGER,
                pp INTEGER,
                max_pp INTEGER,
                description TEXT,
                FOREIGN KEY(pokemon_id) REFERENCES pokemon(id)
            )",
            [],
        )?;

        Ok(Database { conn })
    }

    pub fn create_namespace(&mut self, name: &str) -> Result<bool> {
        let result = self.conn.execute(
            "INSERT INTO namespaces (name) VALUES (?1)",
            params![name],
        );
        Ok(result.is_ok())
    }

    pub fn delete_namespace(&mut self, name: &str) -> Result<bool> {
        let result = self.conn.execute(
            "DELETE FROM namespaces WHERE name = ?1",
            params![name],
        );
        Ok(result.is_ok())
    }

    pub fn get_namespaces(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT name FROM namespaces")?;
        let namespaces = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let mut result = Vec::new();
        for namespace in namespaces {
            result.push(namespace?);
        }
        Ok(result)
    }

    pub fn save_pokemon(&mut self, container: &Container) -> Result<()> {
        let tx = self.conn.transaction()?;

        tx.execute(
            "INSERT INTO pokemon (name, level, hp, attack, defense, speed, pokemon_type, status, exp, exp_to_next_level, namespace, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                container.name,
                container.level,
                container.hp,
                container.attack,
                container.defense,
                container.speed,
                format!("{:?}", container.pokemon_type),
                format!("{:?}", container.state),
                container.exp,
                container.exp_to_next_level,
                container.namespace,
                container.created_at.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            ],
        )?;

        let pokemon_id = tx.last_insert_rowid();

        for move_ in &container.moves {
            tx.execute(
                "INSERT INTO moves (pokemon_id, name, pokemon_type, power, accuracy, pp, max_pp, description)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    pokemon_id,
                    move_.name,
                    format!("{:?}", move_.pokemon_type),
                    move_.power,
                    move_.accuracy,
                    move_.pp,
                    move_.max_pp,
                    move_.description,
                ],
            )?;
        }

        tx.commit()?;
        println!("💾 Saved {} to database!", container.name);
        Ok(())
    }

    pub fn load_pokemon(&self, id: &str) -> Result<Option<Container>> {
        let mut stmt = self.conn.prepare(
            "SELECT name, level, hp, attack, defense, speed, pokemon_type, status, exp, exp_to_next_level, namespace, created_at
             FROM pokemon WHERE id = ?1",
        )?;

        let pokemon = stmt
            .query_row(params![id], |row| {
                let pokemon_type: String = row.get(6)?;
                let created_at = std::time::UNIX_EPOCH + std::time::Duration::from_secs(row.get(11)?);
                
                let container = Container::new(
                    &row.get::<_, String>(0)?,
                    &row.get::<_, String>(10)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    match pokemon_type.as_str() {
                        "Normal" => PokemonType::Normal,
                        "Fire" => PokemonType::Fire,
                        "Water" => PokemonType::Water,
                        "Electric" => PokemonType::Electric,
                        "Grass" => PokemonType::Grass,
                        "Ice" => PokemonType::Ice,
                        "Fighting" => PokemonType::Fighting,
                        "Poison" => PokemonType::Poison,
                        "Ground" => PokemonType::Ground,
                        "Flying" => PokemonType::Flying,
                        "Psychic" => PokemonType::Psychic,
                        "Bug" => PokemonType::Bug,
                        "Rock" => PokemonType::Rock,
                        "Ghost" => PokemonType::Ghost,
                        "Dragon" => PokemonType::Dragon,
                        "Dark" => PokemonType::Dark,
                        "Steel" => PokemonType::Steel,
                        "Fairy" => PokemonType::Fairy,
                        _ => PokemonType::Normal,
                    },
                );
                Ok(container)
            })
            .optional()?;

        if let Some(mut container) = pokemon {
            let mut stmt = self.conn.prepare(
                "SELECT name, power, accuracy, pp, pokemon_type, description
                 FROM moves m
                 JOIN pokemon p ON m.pokemon_id = p.id
                 WHERE p.id = ?1",
            )?;

            let moves = stmt.query_map(params![id], |row| {
                let pokemon_type: String = row.get(4)?;
                Ok(Move::new(
                    &row.get::<_, String>(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    match pokemon_type.as_str() {
                        "Normal" => PokemonType::Normal,
                        "Fire" => PokemonType::Fire,
                        "Water" => PokemonType::Water,
                        "Electric" => PokemonType::Electric,
                        "Grass" => PokemonType::Grass,
                        "Ice" => PokemonType::Ice,
                        "Fighting" => PokemonType::Fighting,
                        "Poison" => PokemonType::Poison,
                        "Ground" => PokemonType::Ground,
                        "Flying" => PokemonType::Flying,
                        "Psychic" => PokemonType::Psychic,
                        "Bug" => PokemonType::Bug,
                        "Rock" => PokemonType::Rock,
                        "Ghost" => PokemonType::Ghost,
                        "Dragon" => PokemonType::Dragon,
                        "Dark" => PokemonType::Dark,
                        "Steel" => PokemonType::Steel,
                        "Fairy" => PokemonType::Fairy,
                        _ => PokemonType::Normal,
                    },
                    &row.get::<_, String>(5)?,
                ))
            })?;

            for move_result in moves {
                container.learn_move(move_result?);
            }

            println!("📂 Loaded {} from database!", id);
            Ok(Some(container))
        } else {
            Ok(None)
        }
    }
}
