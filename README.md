# 🧪 OxideBox

**OxideBox** is a command-line Pokémon container and battle simulator written in Rust. It lets you create, manage, and battle Pokémon, all with persistent storage using SQLite. Think of it as a blend of Pokémon and Docker—Pokémon are "containers" you can summon, pause, start, stop, and battle, all from your terminal.

---

## What Is This Project?

- **Pokémon as Containers:** Each Pokémon is managed like a container, with its own stats, moves, and state (Created, Running, Paused, Stopped).
- **Battle System:** Simulate turn-based Pokémon battles, including type effectiveness and move usage.
- **Persistent Storage:** All Pokémon and their stats are saved in a local SQLite database.
- **Namespaces:** Organize your Pokémon into different groups (namespaces) for better management.
- **Trainer Stats:** Track your overall progress, wins, losses, and more.
- **Daily Challenges:** Complete daily tasks to earn rewards and track your progress.

---

## Features

- **Create and manage Pokémon** with custom stats and up to 4 moves.
- **Battle** between any two "Running" Pokémon.
- **Save/load** Pokémon to/from the database.
- **View detailed stats** for both trainers and Pokémon.
- **Organize Pokémon** using namespaces.
- **Modern, colorful CLI output** for a fun experience.
- **Daily Challenges** with rewards and progress tracking.

---

## How To Use

### 1. **Build the Project**

```bash
cargo build --release
```

### 2. **Run Commands**

All commands are run with `cargo run -- <command> [args]`.

#### **Namespace Management**

- **Create a namespace:**
  ```bash
  cargo run -- create-namespace <name>
  ```
- **Delete a namespace:**
  ```bash
  cargo run -- delete-namespace <name>
  ```

#### **Pokémon Management**

- **Summon a Pokémon:**
  ```bash
  cargo run -- summon <namespace> <name> <level> <hp> <attack> <defense> <speed> <type>
  ```
  Example:
  ```bash
  cargo run -- summon myteam Pikachu 5 100 55 40 90 Electric
  ```

- **Start, Stop, Pause a Pokémon container:**
  ```bash
  cargo run -- start <id>
  cargo run -- stop <id>
  cargo run -- pause <id>
  ```

- **List all Pokémon (optionally by namespace):**
  ```bash
  cargo run -- list
  cargo run -- list <namespace>
  ```

- **View Pokémon status:**
  ```bash
  cargo run -- status <id>
  ```

#### **Battling**

- **Battle two Pokémon (both must be Running):**
  ```bash
  cargo run -- battle <id1> <id2>
  ```

#### **Persistence**

- **Save a Pokémon to the database:**
  ```bash
  cargo run -- save <id>
  ```
- **Load a Pokémon from the database:**
  ```bash
  cargo run -- load <id>
  ```

#### **Trainer Stats**

- **View your overall stats:**
  ```bash
  cargo run -- stats
  ```

#### **Daily Challenges**

- **View current daily challenges:**
  ```bash
  cargo run -- challenges
  ```
  This will show:
  - Current active challenges
  - Progress bars for each challenge
  - Rewards for completing challenges
  - Time remaining until reset

- **Claim rewards for completed challenges:**
  ```bash
  cargo run -- claim-reward <challenge_id>
  ```

##### Challenge Types
- **Battle Wins:** Win a certain number of battles
- **Pokémon Catches:** Catch new Pokémon
- **Move Usage:** Use moves in battles

##### Rewards
- Experience Points (EXP)
- Items (Rare Candy, Poké Balls, PP Up, etc.)
- Progress tracking for trainer stats

---

## How Does It Work?

- **Pokémon are containers:** Each has stats, moves, and a state.
- **Namespaces:** Like folders for your Pokémon.
- **Battle system:** Turn-based, with random move selection and type effectiveness.
- **Database:** All data is stored in `pokemon.db` (SQLite).
- **Colorful output:** Uses the `colored` crate for a modern CLI feel.
- **Daily Challenges:** Automatically generated and reset every 24 hours.

---

## What's Done (as of now)?

- [x] Pokémon container creation and management
- [x] Namespaces for organization
- [x] Persistent storage with SQLite
- [x] Turn-based battle system
- [x] Trainer and Pokémon stats tracking
- [x] Colorful, user-friendly CLI output
- [x] Save/load Pokémon
- [x] List and status commands
- [x] Daily challenge system with rewards
- [x] Progress tracking for challenges
- [x] Automatic challenge reset

---

## Learning & Extending

- **To learn:** Read the code in `src/`—it's modular and commented.
- **To extend:** Add new commands in `cli.rs` and `main.rs`. Add new Pokémon types or moves in `moves.rs`.
- **To reset:** Just delete `pokemon.db` to start fresh.

---

## Requirements

- Rust (latest stable)
- SQLite3 (for the database)

---

## Why?

This project is for learning Rust, database integration, and CLI design—while having fun with Pokémon concepts!

---

## License

MIT

---

**Have fun training and battling your Pokémon—OxideBox style!**