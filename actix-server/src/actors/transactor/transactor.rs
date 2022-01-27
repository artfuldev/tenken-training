use actix::{Actor, Context, Handler};
use regex::Regex;
use lazy_static::lazy_static;

use crate::{messages::{WriteRequested, LatestRequested}};
use super::entry::*;
use super::file_handle::*;
use super::indexed_file_handle::*;

pub struct Transactor {
    pub state: Option<(String, u64)>,
    file: IndexedFileHandle
}

impl Transactor {
    pub fn new(mut file: IndexedFileHandle) -> Self {
        let handle = &mut file;
        let state = handle.read_preface().unwrap_or(None).map(|p| (p.key, p.timestamp));
        Transactor {
            state,
            file
        }
    }

    fn get_timestamp(&self, value: &String) -> u64 {
        lazy_static! {
            static ref TIMESTAMP: Regex = Regex::new("\"eventTransmissionTime\":\\s*(\\d+),").unwrap();
        }
        TIMESTAMP.captures(&value).unwrap().get(1).unwrap().as_str().parse::<u64>().unwrap()
    }

    fn store(&mut self, key: String, value: String) -> () {
        let timestamp = self.get_timestamp(&value);
        match &self.state {
            None => {
                self.state = Some((key.clone(), timestamp.clone()));
                self.file.write(Entry { key, timestamp, value });
            },
            Some((current_key, current_timestamp)) => {
                if key != *current_key || timestamp <= *current_timestamp {
                    return;
                }    
                self.state = Some((key.clone(), timestamp.clone()));
                self.file.write_update(EntryUpdate { timestamp, value });
            }
        }
    }
}

impl Actor for Transactor {
    type Context = Context<Self>;
}

impl Handler<WriteRequested> for Transactor {
    type Result = ();

    fn handle(&mut self, msg: WriteRequested, _ctx: &mut Self::Context) -> Self::Result {
        self.store(msg.key, msg.value);
    }
}

impl Handler<LatestRequested> for Transactor {
    type Result = Option<String>;

    fn handle(&mut self, msg: LatestRequested, _ctx: &mut Self::Context) -> Self::Result {
        let LatestRequested(requested_key) = msg;
        match &self.state {
            None => None,
            Some((key, _)) => {
                if *key != requested_key {
                    None
                } else {
                    self.file.read_value().unwrap_or(None)
                }
            }
        }
    }
}
