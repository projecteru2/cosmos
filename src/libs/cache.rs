use std::collections::HashMap;
use std::sync::Mutex;

static mut CACHE: Option<HealthCache> = None;

type Healthy = bool;

pub struct HealthCache {
    cache: Mutex<HashMap<String, Healthy>>,
}

impl HealthCache {
    pub fn get(&self, id: &String) -> Option<Healthy> {
        match self.cache.lock().unwrap().get(id) {
            Some(healthy) => Some(*healthy),
            None => None,
        }
    }

    pub fn set(&self, id: String, healthy: Healthy) {
        self.cache.lock().unwrap().insert(id, healthy);
    }
}

pub fn get_cache() -> &'static HealthCache {
    unsafe {
        match CACHE {
            None => {
                CACHE = Some(HealthCache {
                    cache: Mutex::new(HashMap::new()),
                });
            }
            _ => {}
        };
        CACHE.as_ref().unwrap()
    }
}
