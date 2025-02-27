# 🎮 OxideBox - A Rust-Powered Pokémon Battle Simulator

Welcome to **OxideBox**, where Rust meets the world of Pokémon! This project brings the thrill of Pokémon battles to life with a modern and efficient implementation.

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


