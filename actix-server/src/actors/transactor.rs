use std::{fs::OpenOptions, io::{SeekFrom, Seek, Read}};

use actix::{Actor, Context, Addr, Handler};
use regex::Regex;
use lazy_static::lazy_static;

use crate::{actors::Writer, messages::{WriteAtRequested, WriteRequested, LatestRequested}};

pub struct Transactor {
    index: u64,
    capacity: u64,
    state: Option<(String, u64)>,
    writer: Addr<Writer>
}

impl Transactor {
    pub fn new(index: u64, capacity: u64, writer: Addr<Writer>) -> Self {
        Transactor {
            index,
            capacity,
            state: None,
            writer
        }
    }

    fn get_timestamp(&self, value: &String) -> u64 {
        lazy_static! {
            static ref TIMESTAMP: Regex = Regex::new("\"eventTransmissionTime\":\\s*(\\d+),").unwrap();
        }
        TIMESTAMP.captures(&value).unwrap().get(1).unwrap().as_str().parse::<u64>().unwrap()
    }

    pub fn restore(&mut self, key: String, timestamp: u64) -> () {
        self.state = Some((key, timestamp));
    }

    fn store(&mut self, next_key: String, next_value: String) -> () {
        let next_timestamp = self.get_timestamp(&next_value); 
        let index = self.index;
        let capacity = self.capacity;
        match &self.state {
            None => {
                let offset = index * capacity;
                let key_size = vec![u8::try_from(next_key.len()).expect("key size over bounds")];
                let key_bytes = next_key.as_bytes().to_vec();
                let timestamp = next_timestamp.to_be_bytes().to_vec();
                let data = next_value.as_bytes().to_vec();
                let data_length = data.len().to_be_bytes().to_vec();
                self.state = Some((next_key, next_timestamp));
                self.writer.do_send(WriteAtRequested { offset, data: [key_size, key_bytes, timestamp, data_length, data].concat() });
            }
            Some((key, timestamp)) => {
                if next_key != *key || next_timestamp <= *timestamp {
                    return;
                }
                let offset = (index * capacity) + 1 + u64::try_from(key.as_bytes().len()).expect("key bytes length over bounds");
                let timestamp = next_timestamp.to_be_bytes().to_vec();
                let data = next_value.as_bytes().to_vec();
                let data_length = data.len().to_be_bytes().to_vec();
                self.state = Some((next_key, next_timestamp));
                self.writer.do_send(WriteAtRequested { offset, data: [timestamp, data_length, data].concat() });
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
                    let mut file = OpenOptions::new()
                        .read(true)
                        .open("db.dat")
                        .expect("Unable to open database file");
                    let mut buffer: [u8; 2048] = [0; 2048];
                    file.seek(SeekFrom::Start(self.index * self.capacity)).expect("seek transactor failed");
                    file.read_exact(&mut buffer).expect("read transactor failed");
                    let key_size = buffer[0];
                    let key_size_offset = usize::try_from(key_size).expect("key size conversion to offset failed");
                    let header_offset = 1 + key_size_offset + 8;
                    let data_length = u64::from_be_bytes(buffer[header_offset..(header_offset + 8)].to_vec().try_into().expect("failed to read data length"));
                    let data_length_offset = usize::try_from(data_length).expect("data size conversion to offset failed");
                    let data = String::from_utf8(buffer[(header_offset + 8)..(header_offset + 8 + data_length_offset)].to_vec()).expect("value read failed");
                    Some(data)
                }
            }
        }
    }
}
