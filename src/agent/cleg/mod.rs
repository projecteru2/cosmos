use std::rc::Rc;

use super::CosmosApp;


pub struct Cleg {}

impl Cleg {
    pub fn new(_app: Rc<impl CosmosApp>) -> Self {
        Cleg {}
    }

    pub fn start(&self) {}

    pub async fn wait(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
