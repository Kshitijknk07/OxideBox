pub struct Container {
    id: String,
    name: String,
    status: ContainerStatus,
}

pub enum ContainerStatus {
    Running,
    Stopped,
}

impl Container {
    pub fn new(id: &str, name: &str) -> Self {
        Container {
            id: id.to_string(),
            name: name.to_string(),
            status: ContainerStatus::Stopped,
        }
    }

    pub fn start(&mut self) {
        self.status = ContainerStatus::Running;
        println!("Container {} started", self.name);
    }

    pub fn stop(&mut self) {
        self.status = ContainerStatus::Stopped;
        println!("Container {} stopped", self.name);
    }

    pub fn status(&self) -> &str {
        match self.status {
            ContainerStatus::Running => "Running",
            ContainerStatus::Stopped => "Stopped",
        }
    }
}
