use std::{time::SystemTime, fs::File};

use fxhash::FxHashMap;

pub trait Database<K, V> {
    fn get(&self, key: K) -> Option<V>;
    fn put(&mut self, key: K, value: V) -> ();
}

pub struct Tenken {
    cache: FxHashMap<String, String>
}

impl Default for Tenken {
    fn default() -> Self {
        Tenken { cache: FxHashMap::<String, String>::default() }
    }
}

impl Database<String, String> for Tenken {
    fn get(&self, key: String) -> Option<String> {
        self.cache.get(&key).map(|x| x.clone())
    }

    fn put(&mut self, key: String, value: String) -> () {
        self.cache.insert(key, value);
    }
}
