mod forwarder;

use std::collections::HashMap;

use futures::future::Future;
use futures::stream;
use futures::stream::Stream;
use lazy_static::lazy_static;

#[derive(Default, Clone)]
pub struct Log {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub entrypoint: String,
    pub ident: String,
    pub data: String,
    pub datetime: String,
    pub extra: HashMap<String, String>,
}

trait LogSubscriber: Send + Sync {
    fn send(&self, log: Log) -> Box<dyn Future<Item = (), Error = ()> + Send>;
}

pub struct LogExchange {
    subs: HashMap<String, Vec<Box<dyn LogSubscriber>>>,
    default_sub: Option<Box<dyn LogSubscriber>>,
}

lazy_static! {
    static ref LOG_EXCHANGE: LogExchange = LogExchange::new();
}

impl LogExchange {
    fn new() -> Self {
        LogExchange {
            default_sub: None,
            subs: HashMap::new(),
        }
    }

    pub fn get() -> &'static Self {
        &LOG_EXCHANGE
    }

    pub fn attach(&mut self, name: String, sub: Box<dyn LogSubscriber>) -> usize {
        let subs = self.subs.entry(name).or_insert(vec![]);
        subs.push(sub);
        subs.len() - 1
    }

    pub fn detach(&mut self, name: &String, index: usize) {
        let mut subs = self.subs.get_mut(name).unwrap();
        subs.remove(index);
    }

    pub fn send(&'static self, log: Log) -> impl Future<Item = (), Error = ()> + Send {
        let subs = self.get_subs(&log.name);
        stream::iter_ok(subs).for_each(move |sub| sub.send(log.clone()))
    }

    fn get_subs(&'static self, name: &String) -> Vec<&'static Box<dyn LogSubscriber>> {
        let mut res = vec![self.default_sub.as_ref().unwrap()];
        match self.subs.get(name) {
            Some(subs) => {
                res.extend(subs);
                res
            }
            None => res,
        }
    }
}
