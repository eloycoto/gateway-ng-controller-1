use crate::envoy_helpers::EnvoyExport;

use std::sync::Mutex;

pub struct LocalCache {
    cache: Vec<EnvoyExport>,
    version: u32,
    temp_cache: Vec<EnvoyExport>,
    subscribers: Vec<Box<dyn FnMut(Vec<EnvoyExport>) + Send + 'static>>,
}

impl LocalCache {
    pub fn new() -> LocalCache {
        LocalCache {
            cache: Vec::new(),
            version: 0,
            temp_cache: Vec::new(),
            subscribers: Vec::new(),
        }
    }

    // TODO Remove dead_code lint
    #[allow(dead_code)]
    pub fn add(&mut self, element: EnvoyExport) {
        self.temp_cache.push(element)
    }

    pub fn add_multiple(&mut self, elements: &mut Vec<EnvoyExport>) {
        self.temp_cache.append(elements);
    }

    pub fn subscribe(&mut self, cb: impl FnMut(Vec<EnvoyExport>) + Send + 'static) {
        self.subscribers.push(Box::new(cb));
    }

    pub fn publish(&mut self) {
        for callback in self.subscribers.iter_mut() {
            callback(self.cache.clone());
        }
    }

    pub fn release(&mut self) -> u32 {
        self.cache = self.temp_cache.clone();
        self.version += 1;
        self.temp_cache = Vec::new();
        self.publish();
        self.version
    }
}

lazy_static! {
    static ref CACHE: Mutex<LocalCache> = Mutex::new(LocalCache::new());
}

// TODO Remove dead_code lint
#[allow(dead_code)]
pub fn add(entry: EnvoyExport) {
    let mut cache = CACHE.lock().unwrap();
    cache.add(entry);
}

pub fn add_multiple(entries: &mut Vec<EnvoyExport>) {
    let mut cache = CACHE.lock().unwrap();
    cache.add_multiple(entries)
}

pub fn release() -> u32 {
    let mut cache = CACHE.lock().unwrap();
    cache.release()
}

pub fn subcribe_release(callback: impl FnMut(Vec<EnvoyExport>) + Send + 'static) -> bool {
    let mut cache = CACHE.lock().unwrap();
    cache.subscribe(callback);
    return true;
}
