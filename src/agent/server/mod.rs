use std::sync::Arc;

use futures::future::Future;
use futures::sync::oneshot;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};

use super::CosmosApp;
use crate::config::get_config;
use crate::logging;

pub struct HTTPServer<T: CosmosApp + 'static> {
    pub app: &'static T,

    finish: Option<oneshot::Sender<()>>,
}

impl<T: CosmosApp + 'static> HTTPServer<T> {
    pub fn new(app: &'static T) -> HTTPServer<T> {
        HTTPServer {
            app: app,
            finish: None,
        }
    }

    pub fn start(&mut self) {
        let (tx, _rx) = oneshot::channel();
        self.finish = Some(tx);

        let app = Arc::new(self.app);
        let conf = get_config();
        let addr = conf.api.addr.parse().unwrap();
        logging::info(&format!("listening on {}", addr));

        let server = Server::bind(&addr)
            .serve(move || {
                let app = app.clone();
                service_fn(move |req| Self::dispatch(app.clone(), req))
            })
            //.with_graceful_shutdown(rx)
            .map_err(|err| {
                logging::error(&format!("server error: {:#?}", err));
            });

        hyper::rt::spawn(server);
    }

    pub fn _stop(&mut self) {
        logging::info("server stopping");
        self.finish
            .take()
            .expect("server not started")
            .send(())
            .unwrap();
    }

    fn dispatch(app: Arc<&T>, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
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
