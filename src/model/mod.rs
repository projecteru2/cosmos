mod container;
mod node;

pub use container::EruContainer;
pub use node::Node;

pub trait Sandbox {
    type Event: std::fmt::Debug;

    fn handle_event(&self, event: Self::Event);
    fn started(&self);
    fn died(&self);
    fn report(&self);
}
