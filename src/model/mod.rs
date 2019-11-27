mod container;
mod node;

pub use container::EruContainer;
pub use node::Node;

pub trait Sandbox {
    fn started(&self);
    fn died(&self);
    fn report(&self);
}
