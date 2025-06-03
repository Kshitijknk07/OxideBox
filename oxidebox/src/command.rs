use crate::cli::Commands;
use crate::container::ContainerManager;
use crate::evolution::EvolutionManager;
use crate::team::TeamManager;

pub struct CommandHandler {
    container_manager: ContainerManager,
    evolution_manager: EvolutionManager,
    team_manager: TeamManager,
}

impl CommandHandler {
    pub fn new() -> Self {
        Self {
            container_manager: ContainerManager::new(),
            evolution_manager: EvolutionManager::new(),
            team_manager: TeamManager::new(),
        }
    }

    pub fn handle_command(&mut self, command: Commands) {
        match command {
            Commands::Summon {
                namespace,
                name,
                level,
                hp,
                attack,
                defense,
                speed,
                pokemon_type,
            } => {
                self.container_manager.summon(
                    &namespace,
                    &name,
                    level,
                    hp,
                    attack,
                    defense,
                    speed,
                    pokemon_type,
                );
            }
            Commands::Start { id } => {
                self.container_manager.start_container(&id);
            }
            Commands::Stop { id } => {
                self.container_manager.stop_container(&id);
            }
            Commands::Pause { id } => {
                self.container_manager.pause_container(&id);
            }
            Commands::List { namespace } => {
                if let Some(ns) = namespace {
                    self.container_manager.list_containers(Some(ns.as_str()));
                } else {
                    self.container_manager.list_containers(None);
                }
            }
            Commands::Status { id } => {
                if let Some(container) = self.container_manager.get_container(&id) {
                    container.display_status();
                }
            }
            Commands::Battle { id1, id2 } => {
                self.container_manager.battle(&id1, &id2, &self.evolution_manager);
            }
            Commands::Save { id } => {
                if let Err(e) = self.container_manager.save_to_db(&id) {
                    eprintln!("Error saving container: {}", e);
                }
            }
            Commands::Load { id } => {
                if let Err(e) = self.container_manager.load_from_db(&id) {
                    eprintln!("Error loading container: {}", e);
                }
            }
            Commands::Stats => {
                self.container_manager.display_stats();
            }
            Commands::CreateNamespace { name } => {
                self.container_manager.create_namespace(&name);
            }
            Commands::DeleteNamespace { name } => {
                self.container_manager.delete_namespace(&name);
            }
        }
    }
}
