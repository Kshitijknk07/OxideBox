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

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🔍 Implementation Notes

- Uses `rusqlite` for database operations
- Implements `clap` for command-line argument parsing
- Uses `chrono` for timestamp management
- Features colored terminal output using the `colored` crate
- Thread-safe battle system implementation
- Efficient memory management through Rust's ownership system

---

## 🌟 What is OxideBox?

**OxideBox** is a command-line Pokémon battle simulator written in Rust that captures the essence of Pokémon battles while adding unique twists. Designed for both fun and performance, it leverages Rust's powerful features to create an engaging battle experience.

---

## ✨ Key Features

### 🏃‍♂️ Core Battle System
- Dynamic turn-based battle mechanics
- Real-time battle statistics tracking
- Experience points and leveling system
- Type-based battle interactions
- Move learning and management (up to 4 moves per Pokémon)

### 📊 Advanced Statistics
- Detailed battle history and records
- Individual Pokémon stats tracking
- Comprehensive trainer statistics
- Move usage analytics
- Damage dealt/received tracking

### 🎯 Pokémon Management
- Summon new Pokémon with custom stats
- Release Pokémon back to the wild
- Recall system for active Pokémon
- Built-in Pokédex functionality
- Evolution system

### 💾 Data Persistence
- **SQLite** database integration
- Save and load Pokémon data
- Battle record storage
- Persistent trainer statistics

---

## 🛠️ Technical Highlights

- **Written in pure Rust** 🦀
- Efficient memory management
- Thread-safe battle system
- Modular and extensible architecture
- SQLite database for persistent data
- Chrono integration for temporal tracking

---

## 🎨 User Experience

OxideBox provides an engaging **command-line interface** with:
- **Emoji-enhanced output** for better readability
- **Clear battle progression indicators**
- **Intuitive command structure**
- **Real-time battle feedback**
- **Detailed status reporting**

---

## 🎮 Gameplay Commands

### 🏆 Battle Commands
```bash
battle <pokemon1_name> <pokemon2_name>   # Start a battle between two Pokémon
```

### 🛡️ Pokémon Management
```bash
summon <name> <level> <hp> <attack> <defense> <speed> <type>   # Create a new Pokémon
recall <pokemon_name>    # Recall a Pokémon from battle
release <pokemon_name>   # Release a Pokémon
pokedex                  # View all captured Pokémon
```

### ⚡ Move Management
```bash
learn <pokemon_name> <move_name>    # Teach a move to a Pokémon
```

### 💾 Database Operations
```bash
save <pokemon_name>    # Save a Pokémon to the database
load <pokemon_name>    # Load a Pokémon from the database
```

### 📊 Statistics
```bash
stats    # View trainer statistics
```

---

## 📝 Example Usage

### 1️⃣ Create two Pokémon
```bash
summon Pikachu 5 100 55 40 90 Electric
summon Charmander 5 100 52 43 65 Fire
```

### 2️⃣ Teach them moves
```bash
learn Pikachu Thunderbolt
learn Charmander Ember
```

### 3️⃣ Start a battle
```bash
battle Pikachu Charmander
```

### 4️⃣ Save your Pokémon
```bash
save Pikachu
```

### 5️⃣ View your collection
```bash
pokedex
```

---


