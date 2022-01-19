use std::{fs::{OpenOptions, File}, os::unix::prelude::FileExt};

use actix::{Actor, Context, Handler};
use regex::Regex;
use lazy_static::lazy_static;

use crate::{messages::{WriteRequested, LatestRequested}};

const SIZE_U64: u64 = 2048;
const HEADER_BUFFER_SIZE: usize = 109;

pub struct IndexedFileHandle {
    header_buffer: [u8; HEADER_BUFFER_SIZE],
    offset: u64,
    key_size: u64,
    file: File,
}

pub struct EntryPreface<K> {
    pub key: K,
    pub timestamp: u64,
}

pub struct Entry<K, V> {
    key: K,
    timestamp: u64,
    value: V
}

pub struct EntryUpdate<V> {
    timestamp: u64,
    value: V
}

pub enum FileHandleError {
    ReadIntoBufferFailed,
    ReadDataLengthFailed,
    DataLengthOffsetConversionFailed,
    ReadDataAsStringFailed,
    ReadIntoHeaderBufferFailed,
    KeySizeConversionFailed,
    KeyReadFailed,
    TimestampReadFailed,
    KeyWriteFailed,
    UpdateWriteFailed
}

type FhResult<T> = Result<T, FileHandleError>;

pub trait FileHandle<K, V> {
    fn read(&mut self) -> FhResult<Option<Entry<K, V>>>;
    fn read_value(&self) -> FhResult<Option<V>>;
    fn read_preface(&mut self) -> FhResult<Option<EntryPreface<K>>>;
    fn write(&mut self, entry: Entry<K, V>) -> FhResult<()>;
    fn write_key(&mut self, key: K) -> FhResult<()>;
    fn write_update(&self, update: EntryUpdate<V>) -> FhResult<()>;
}

impl IndexedFileHandle {
    pub fn new(index: u64) -> Self {
        let file =
            OpenOptions::new()
                .read(true)
                .write(true)
                .open("db.dat")
                .expect("Unable to open database file");
        IndexedFileHandle {
            offset: index * SIZE_U64,
            key_size: 0,
            file,
            header_buffer: [0; HEADER_BUFFER_SIZE]
        }
    }
}

impl FileHandle<String, String> for IndexedFileHandle {
    fn read(&mut self) -> FhResult<Option<Entry<String, String>>> {
        match self.read_preface()? {
            None => Ok(None),
            Some(preface) => {
                match self.read_value()? {
                    None => Ok(None),
                    Some(value) => Ok(Some(Entry {
                        key: preface.key,
                        timestamp: preface.timestamp,
                        value
                    }))
                }
            }
        }
    }

    fn read_value(&self) -> FhResult<Option<String>> {
        let mut buffer: [u8; 2048] = [0; 2048];
        if self.key_size == 0 {
            return Ok(None);
        }
        let header_offset = usize::try_from(self.key_size + 8).map_err(|_| FileHandleError::KeySizeConversionFailed)?;
        self.file.read_exact_at(&mut buffer, self.offset).map_err(|_| FileHandleError::ReadIntoBufferFailed)?;
        let data_length = u64::from_be_bytes(buffer[(header_offset)..(8 + header_offset)].try_into().map_err(|_| FileHandleError::ReadDataLengthFailed)?);
        let data_length_offset = usize::try_from(data_length).map_err(|_| FileHandleError::DataLengthOffsetConversionFailed)?;
        let data = String::from_utf8(buffer[(8 + header_offset)..(8 + header_offset + data_length_offset)].to_vec()).map_err(|_| FileHandleError::ReadDataAsStringFailed)?;
        Ok(Some(data))
    }

    fn read_preface(&mut self) -> FhResult<Option<EntryPreface<String>>> {
        self.file.read_exact_at(&mut self.header_buffer, self.offset).map_err(|_| FileHandleError::ReadIntoHeaderBufferFailed)?;
        let key_size = self.header_buffer[0];
        self.key_size = u64::try_from(key_size).map_err(|_| FileHandleError::KeySizeConversionFailed)?;
        if key_size == 0 {
            return Ok(None);
        }
        let key_size_usize = usize::try_from(key_size).map_err(|_| FileHandleError::KeySizeConversionFailed)?;
        let key_bytes = self.header_buffer[1..key_size_usize].to_vec();
        let key = String::from_utf8(key_bytes).map_err(|_| FileHandleError::KeyReadFailed)?;
        let timestamp_bytes = self.header_buffer[(key_size_usize)..(key_size_usize + 8)].to_vec().try_into().map_err(|_| FileHandleError::TimestampReadFailed)?;
        let timestamp = u64::from_be_bytes(timestamp_bytes);
        if timestamp == 0 {
            return Ok(None);
        }
        Ok(Some(EntryPreface { key, timestamp }))
    }

    fn write(&mut self, entry: Entry<String, String>) -> FhResult<()> {
        self.write_key(entry.key)?;
        self.write_update(EntryUpdate { timestamp: entry.timestamp, value: entry.value })?;
        Ok(())
    }
    
    fn write_key(&mut self, key: String) -> FhResult<()> {
        self.key_size = u64::try_from(key.len()).map_err(|_| FileHandleError::KeySizeConversionFailed)?; 
        let key_size = vec![u8::try_from(self.key_size).map_err(|_| FileHandleError::KeySizeConversionFailed)?];
        let key_bytes = key.as_bytes().to_vec();
        self.file.write_at(&mut [key_size, key_bytes].concat(), self.offset).map_err(|_| FileHandleError::KeyWriteFailed)?;
        Ok(())
    }

    fn write_update(&self, update: EntryUpdate<String>) -> FhResult<()> {
        let timestamp = update.timestamp.to_be_bytes().to_vec();
        let data_length = update.value.len().to_be_bytes().to_vec();
        let data = update.value.as_bytes().to_vec();
        self.file.write_at(&mut [timestamp, data_length, data].concat(), self.offset + self.key_size).map_err(|_| FileHandleError::UpdateWriteFailed)?;
        Ok(())
    }
}

pub struct Transactor {
    state: Option<(String, u64)>,
    file: IndexedFileHandle
}

impl Transactor {
    pub fn new(file: IndexedFileHandle) -> Self {
        Transactor {
            state: None,
            file
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
        if let Some((key, timestamp)) = &self.state {
            if next_key != *key || next_timestamp <= *timestamp {
                return;
            }
        }
        self.state = Some((next_key.clone(), next_timestamp.clone()));
        self.file.write(Entry { key: next_key, timestamp: next_timestamp, value: next_value });
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
