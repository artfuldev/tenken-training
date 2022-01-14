use fxhash::FxHashMap;
use actix::{Actor, Message, Handler};
use thiserror::Error;

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

impl Actor for Tenken {
    type Context = actix::Context<Self>;
}

pub struct ProbePayloadReceived {
    probe_id: String,
    payload: String
}

impl ProbePayloadReceived {
    pub fn new(probe_id: String, payload: String) -> Self {
      ProbePayloadReceived { probe_id, payload }
    }
}

#[derive(Error, Debug)]
pub enum TenkenError {
  #[error("something failed")]
  GenericError
}

impl Message for ProbePayloadReceived {
    type Result = Result<(), TenkenError>;
}

impl Handler<ProbePayloadReceived> for Tenken {
    type Result = Result<(), TenkenError>;

    fn handle(&mut self, msg: ProbePayloadReceived, _ctx: &mut Self::Context) -> Self::Result {
        self.put(msg.probe_id, msg.payload);
        Ok(())
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
