use futures::future::ok;
use futures::future::Future;

use super::Log;
use super::LogSubscriber;

pub struct Forwarder {}

impl Forwarder {
    pub fn get() -> Self {
        Self {}
    }
}

impl LogSubscriber for Forwarder {
    fn send(&self, log: Log) -> Box<dyn Future<Item = (), Error = ()> + Send> {
        Box::new(ok(()))
    }
}
