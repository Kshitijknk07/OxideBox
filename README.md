# 🎮 OxideBox

A modern, high-performance Pokémon battle simulator written in Rust, featuring SQLite persistence and a command-line interface.

## 📖 Overview

OxideBox is a command-line application that simulates Pokémon battles with a focus on performance and data persistence. It uses SQLite for storing Pokémon data and battle statistics, making it perfect for tracking your training progress over time.

## ⚡ Features

### Core Systems
- **Namespace Management**: Organize your Pokémon in isolated environments
- **Battle System**: Turn-based battles with type effectiveness
- **Stats Tracking**: Comprehensive battle and trainer statistics
- **Database Integration**: Persistent storage using SQLite
- **Resource Management**: CPU, memory, and storage monitoring for each Pokémon

### Pokémon Management
- Create and manage multiple Pokémon with custom stats
- Support for all 18 Pokémon types
- Up to 4 moves per Pokémon
- Experience and leveling system
- Container-like state management (Created, Running, Paused, Stopped)

## 🚀 Getting Started

### Prerequisites
- Rust (latest stable version)
- SQLite3

### Installation
1. Clone the repository:
```bash
git clone https://github.com/yourusername/oxidebox.git
cd oxidebox
```

2. Build the project:
```bash
cargo build --release
```

## 🎮 Usage Guide

### Namespace Management
Create isolated environments for your Pokémon:
```bash
cargo run -- create-namespace <name>
cargo run -- delete-namespace <name>
```

### Pokémon Management
1. Summon a new Pokémon:
```bash
cargo run -- summon <namespace> <name> <level> <hp> <attack> <defense> <speed> <type>
```
Example:
```bash
cargo run -- summon trainer1 Pikachu 5 100 55 40 90 Electric
```

2. Container Management:
```bash
cargo run -- start <id>     # Start a Pokémon container
cargo run -- stop <id>      # Stop a Pokémon container
cargo run -- pause <id>     # Pause a Pokémon container
```

3. View Status:
```bash
cargo run -- status <id>    # View detailed Pokémon status
cargo run -- list [namespace]  # List all Pokémon (optionally filtered by namespace)
```

### Battle System
Start a battle between two Pokémon:
```bash
cargo run -- battle <id1> <id2>
```
Note: Both Pokémon must be in the "Running" state to battle.

### Data Persistence
Save and load Pokémon data:
```bash
cargo run -- save <id>      # Save a Pokémon to the database
cargo run -- load <id>      # Load a Pokémon from the database
```

### Statistics
View trainer statistics:
```bash
cargo run -- stats
```

## 📊 Data Structure

### Pokémon Stats
- Level (u8)
- HP (u16)
- Attack (u16)
- Defense (u16)
- Speed (u16)
- Type (PokemonType enum)
- Experience points
- Moves (up to 4)

### Move Properties
- Name
- Power
- Accuracy
- PP (Power Points)
- Type
- Description

### Container Resources
- CPU usage limit
- Memory limit
- Storage limit
- Current resource usage tracking

## 🔧 Technical Details

### Database Schema
- `namespaces`: Stores namespace information
- `pokemon`: Stores Pokémon data with foreign key to namespaces
- `moves`: Stores move data with foreign key to pokemon

### State Management
Pokémon containers can be in the following states:
- Created
- Running
- Paused
- Stopped

Thank You....