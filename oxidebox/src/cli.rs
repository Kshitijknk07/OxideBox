use clap::{Parser, Subcommand};
use crate::moves::PokemonType;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    CreateNamespace {
        name: String,
    },
    DeleteNamespace {
        name: String,
    },
    Summon {
        namespace: String,
        name: String,
        level: u8,
        hp: u16,
        attack: u16,
        defense: u16,
        speed: u16,
        pokemon_type: PokemonType,
    },
    Start {
        id: String,
    },
    Stop {
        id: String,
    },
    Pause {
        id: String,
    },
    List {
        namespace: Option<String>,
    },
    Status {
        id: String,
    },
    Battle {
        id1: String,
        id2: String,
    },
    Save {
        id: String,
    },
    Load {
        id: String,
    },
    Stats,
}
