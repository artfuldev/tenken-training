use std::{fs::{File, OpenOptions}, io::Write};

use actix::{Actor, Message, Handler};
use thiserror::Error;

pub(crate) struct Writer {
    file: File
}

impl Default for Writer {
    fn default() -> Self {
        Writer {
            file:
                OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open("db.dat")
                    .unwrap()
        }
    }
}

pub(crate) enum Log {
    WriteAhead(String),
    Debug(String)
}

impl Actor for Writer {
    type Context = actix::Context<Self>;
}

#[derive(Error, Debug)]
pub(crate) enum WriterError {
    #[error("generic")]
    GenericError
}

impl Message for Log {
    type Result = Result<(), WriterError>;
}

impl Handler<Log> for Writer {
    type Result = Result<(), WriterError>;

    fn handle(&mut self, msg: Log, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Log::WriteAhead(message) => match self.file.write_all(message.as_bytes()) {
                Ok(_) => Ok(()),
                Err(_) => Err(WriterError::GenericError)
            },
            Log::Debug(message) => {
                println!("{}", message);
                Ok(())
            }
        }
    }
}
