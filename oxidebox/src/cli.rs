use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Summon {
        pokemon: String,
    },
    Recall {
        pokemon: String,
    },
    Release {
        pokemon: String,
    },
    Pokedex,
    Battle {
        pokemon1: String,
        pokemon2: String,
    },
    Save {
        pokemon: String,
    },
    Load {
        pokemon: String,
    },
    Stats,
}
