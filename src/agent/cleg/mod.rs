use std::sync::Arc;

use futures::channel::oneshot;
use futures::stream::StreamExt;

use super::CosmosApp;
use crate::logging;

pub struct Cleg<T: CosmosApp + 'static> {
    pub app: &'static T,

    finish: Option<oneshot::Receiver<()>>,
}

impl<T: CosmosApp> Cleg<T> {
    pub fn new(app: &'static T) -> Self {
        Cleg { app, finish: None }
    }

    pub fn start(&mut self) {
        let (tx, rx) = oneshot::channel();
        self.finish = Some(rx);

        let app = Arc::new(self.app);
        tokio::spawn(async move {
            let mut stream = app.watch();
            while let Some(item) = stream.next().await {
                logging::info(&format!("{:#?}", item));
            }
            tx.send(()).unwrap();
        });
    }

    pub async fn wait(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.finish.as_mut().expect("cleg not started").await?;
        Ok(())
    }
}
