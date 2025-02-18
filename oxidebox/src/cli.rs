use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "OxideBox")]
#[clap(about = "A lightweight Pokémon-themed container runtime")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Summon { pokemon: String },  // Run a container
    Recall { pokemon: String },  // Stop a container
    Pokedex,                     // List active containers
    Release { pokemon: String }, // Remove a container
}
