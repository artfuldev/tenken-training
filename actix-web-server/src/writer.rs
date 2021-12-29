use std::{fs::{File, OpenOptions}, io, result::Result};
use serde_json::{to_string};
use actix::{Actor, ActorContext, ActorState, Handler, Message};
use std::io::Write;
use thiserror::Error;

use crate::dto::ProbeData;


pub struct Writer {
    state: ActorState,
    file: File,
}

impl Writer {
    pub fn new() -> io::Result<Self> {
      let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("db.dat")?;
      Ok(Writer {
          state: ActorState::Started,
          file,
      })
    }
}

impl ActorContext for Writer {
    fn stop(&mut self) {
      self.state = ActorState::Stopping;
      self.state = ActorState::Stopped;
    }

    fn terminate(&mut self) {
      self.state = ActorState::Stopped;
    }

    fn state(&self) -> ActorState {
      self.state
    }
}

impl Actor for Writer {
    type Context = actix::Context<Self>;
}

pub struct ProbePayloadReceived {
    probe_id: String,
    payload: ProbeData,
}

impl ProbePayloadReceived {
  pub fn new(probe_id: String, payload: ProbeData) -> Self {
    ProbePayloadReceived { probe_id, payload }
  }
}

impl Message for ProbePayloadReceived {
    type Result = Result<(), WriterError>;
}

#[derive(Error, Debug)]
pub enum WriterError {
  #[error("serialization failed")]
  SerializationError,
  #[error("writing to file failed")]
  FileIOError
}

impl Handler<ProbePayloadReceived> for Writer {
    type Result = Result<(), WriterError>;

    fn handle(&mut self, msg: ProbePayloadReceived, _: &mut Self::Context) -> Self::Result {
      let string_data = to_string(&msg.payload);
      match string_data {
        Ok(serialized) =>
          match writeln!(self.file, "{}:::{}", msg.probe_id, serialized) {
            Ok(_) => Ok(()),
            Err(_) => Err(WriterError::FileIOError)
          },
        Err(_) => Err(WriterError::SerializationError)
      }
    }
}
