use actix::{Actor, Context, Handler};

use crate::{messages::{WriteRequested, LatestRequested}};
use super::entry::*;
use super::file_handle::*;
use super::indexed_file_handle::*;
use super::payload::*;

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

    fn store(&mut self, key: String, original_value: String) -> () {
        let timestamp = get_timestamp(&original_value);
        let value = with_received_time(&original_value);
        match &self.state {
            None => {
                self.state = Some((key.clone(), timestamp.clone()));
                self.file.write(Entry { key, timestamp, value });
            },
            Some((current_key, current_timestamp)) => {
                if key != *current_key || timestamp <= *current_timestamp {
                    return;
                }    
                self.state = Some((key, timestamp.clone()));
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
