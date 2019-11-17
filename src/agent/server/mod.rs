use std::sync::Arc;

use futures::channel::oneshot;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use tokio;

use super::CosmosApp;
use crate::config::Config;
use crate::logging;

pub struct HTTPServer<T: CosmosApp + 'static> {
    pub app: &'static T,

    finish: Option<oneshot::Receiver<()>>,
}

impl<T: CosmosApp + 'static> HTTPServer<T> {
    pub fn new(app: &'static T) -> HTTPServer<T> {
        HTTPServer {
            app: app,
            finish: None,
        }
    }

    pub fn start(&mut self) {
        let (tx, rx) = oneshot::channel();
        self.finish = Some(rx);

        let app = Arc::new(self.app);
        tokio::spawn(async move {
            let conf = Config::get();
            let addr = conf.api.addr.parse().unwrap();
            let svc = make_service_fn(|_| {
                let app = app.clone();
                async move {
                    Ok::<_, hyper::Error>(service_fn(move |req: Request<Body>| {
                        Self::dispatch(app.clone(), req)
                    }))
                }
            });

            logging::info(&format!("listening on {}", addr));
            let server = Server::bind(&addr).serve(svc);
            server.await.unwrap();
            logging::debug("server exit");
            tx.send(()).unwrap();
        });
    }

    pub async fn wait(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.finish.as_mut().unwrap().await?;
        Ok(())
    }

    async fn dispatch(app: Arc<&T>, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/version") => Ok(Response::new(Body::from(app.version()))),

            _ => {
                let mut not_found = Response::default();
                *not_found.status_mut() = StatusCode::NOT_FOUND;
                Ok(not_found)
            }
        }
    }
}
