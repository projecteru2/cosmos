use tokio;
use futures::channel::oneshot;

use std::rc::Rc;

use crate::logging;
use super::CosmosApp;

pub struct Server {
    finish: Option<oneshot::Receiver<()>>,
}

impl Server {
    pub fn new(_app: Rc<impl CosmosApp>) -> Self {
        Server {finish: None}
    }
    pub fn start(&mut self) {
        let (tx, rx) = oneshot::channel();
        self.finish = Some(rx);
        tokio::spawn(async {
            tx.send(()).unwrap();
        });
    }

    pub async fn wait(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.finish.as_mut().expect("server not started").await?;
        logging::debug("server exited");
        Ok(())
    }
}
