// commands.rs

use crate::container::ContainerManager;
use crate::team::TeamManager;

pub enum Command {
    Summon { name: String, level: u32, hp: u32 },
    Recall { name: String },
    Release { name: String },
    CreateTeam { team_name: String },
    AddToTeam { team_name: String, container_name: String },
    RemoveFromTeam { team_name: String, container_name: String },
    Pokedex,
    TeamInfo { team_name: String },
}

pub struct CommandHandler {
    container_manager: ContainerManager,
    team_manager: TeamManager,
}

impl CommandHandler {
    pub fn new(container_manager: ContainerManager, team_manager: TeamManager) -> Self {
        CommandHandler {
            container_manager,
            team_manager,
        }
    }

    pub fn handle_command(&mut self, command: Command) {
        match command {
            Command::Summon { name, level, hp } => {
                self.container_manager.summon(&name, level, hp);
            }
            Command::Recall { name } => {
                self.container_manager.recall(&name);
            }
            Command::Release { name } => {
                self.container_manager.release(&name);
            }
            Command::CreateTeam { team_name } => {
                self.team_manager.create_team(&team_name);
            }
            Command::AddToTeam { team_name, container_name } => {
                self.team_manager.add_to_team(&team_name, &container_name, &self.container_manager.containers);
            }
            Command::RemoveFromTeam { team_name, container_name } => {
                self.team_manager.remove_from_team(&team_name, &container_name);
            }
            Command::Pokedex => {
                self.container_manager.pokedex();
            }
            Command::TeamInfo { team_name } => {
                self.team_manager.team_info(&team_name, &self.container_manager.containers);
            }
        }
    }
}
