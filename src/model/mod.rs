mod container;
mod node;

pub use container::EruContainer;
pub use node::Node;

pub trait Sandbox {
    type Event: std::fmt::Debug;

    fn handle_event(&self, event: Self::Event);
    fn report(&self);
}
