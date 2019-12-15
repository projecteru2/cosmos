use shiplift::rep::Event;

use super::SandboxEvent;

#[derive(Debug)]
pub struct DockerEvent {
    pub id: String,
    pub action: String,
}

impl SandboxEvent for DockerEvent {
    fn sandbox_id(&self) -> String {
        self.id.clone()
    }
}

impl DockerEvent {
    pub fn new(event: Event) -> Option<Self> {
        match event {
            Event {
                action,
                id: Some(id),
                ..
            } => Some(Self { id, action }),
            _ => None,
        }
    }
}
