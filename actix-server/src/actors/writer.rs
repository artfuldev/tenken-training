use std::{fs::{File, OpenOptions}, io::{Seek, SeekFrom, Write}};

use actix::{ Actor, Context, Handler };

use crate::messages::WriteAtRequested;

pub struct Writer {
    file: File
}

impl Writer {
    pub fn new(size: u64, truncate: bool) -> Self {
        let file: File = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(truncate)
            .open("db.dat")
            .expect("Unable to open database file");
        file.set_len(size).expect("Unable to set length of database file");
        Writer { file }
    }
}

impl Actor for Writer {
    type Context = Context<Self>;
}

impl Handler<WriteAtRequested> for Writer {
    type Result = ();

    fn handle(&mut self, msg: WriteAtRequested, _ctx: &mut Self::Context) -> Self::Result {
        self.file.seek(SeekFrom::Start(msg.offset)).expect("failed to seek to offset");
        self.file.write_all(&msg.data).expect("failed to write bytes to file");
    }
}
