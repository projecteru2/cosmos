mod container;
mod event;
mod node;

pub use container::EruContainer;
pub use event::DockerEvent;
pub use node::Node;

pub trait Sandbox {
    type Event: std::fmt::Debug;

    fn handle_event(&self, event: Self::Event);
    fn report(&self);
}

pub trait SandboxEvent {
    fn sandbox_id(&self) -> String;
}
