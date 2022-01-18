use std::fs::{File, OpenOptions};

use actix::{ Actor, Context, Handler };

use crate::messages::{ LatestRequested, WriteRequested };

pub struct Tenken;

impl Tenken {
    pub fn new(file_name: String, capacity: u64, partition_size: u64, truncate: bool) -> Self {
        let file: File = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(truncate)
            .open(file_name)
            .expect("Unable to open database");
        file
            .set_len(capacity * partition_size)
            .expect("Unable to set length of database file");
        Tenken
    }
}

impl Actor for Tenken {
    type Context = Context<Self>;
}

impl Handler<LatestRequested> for Tenken {
    type Result = Option<String>;

    fn handle(&mut self, msg: LatestRequested, ctx: &mut Self::Context) -> Self::Result {
        None
    }
}

impl Handler<WriteRequested> for Tenken {
    type Result = ();

    fn handle(&mut self, msg: WriteRequested, ctx: &mut Self::Context) -> Self::Result {
        
    }
}
