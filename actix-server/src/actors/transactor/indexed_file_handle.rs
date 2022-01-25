use std::{fs::File, os::unix::prelude::FileExt};

use super::entry::*;
use super::file_handle::*;
use super::partition::*;

pub struct IndexedFileHandle {
    offset: u64,
    key_size: Option<KeySize>,
    file: File,
}

impl IndexedFileHandle {
    pub fn new(index: u64, file: File) -> Self {
        IndexedFileHandle {
            offset: index * PARTITION_SIZE as u64,
            key_size: None,
            file,
        }
    }

    fn read_preface_from_buffer(&mut self, buffer: &[u8]) -> Option<EntryPreface<String>> {
        self.key_size = key_size(buffer);
        self.key_size.and_then(|key_size|
            key(key_size, buffer).and_then(|key|
                    timestamp(key_size, buffer).and_then(|timestamp|
                        Some(EntryPreface { key, timestamp }))))
    }

    fn read_value_from_buffer(&self, buffer: &[u8]) -> Option<String> {
        self.key_size.and_then(|key_size|
            value_size(key_size, buffer).and_then(|value_size|
                value(key_size, value_size, buffer)))
    }
}

impl FileHandle<String, String> for IndexedFileHandle {

    fn read(&mut self) -> FhResult<Option<Entry<String, String>>> {
        let mut buffer: [u8; PARTITION_SIZE] = [0; PARTITION_SIZE];
        self.file.read_exact_at(&mut buffer, self.offset)
            .map_err(|_| FileHandleError::ReadIntoBufferFailed)?;
        Ok(
            self.read_preface_from_buffer(&buffer).and_then(|preface|
                self.read_value_from_buffer(&buffer).and_then(|value|
                    Some(Entry { key: preface.key, timestamp: preface.timestamp, value }))))
    }

    fn read_value(&self) -> FhResult<Option<String>> {
        let mut buffer: [u8; PARTITION_SIZE] = [0; PARTITION_SIZE];
        self.file.read_exact_at(&mut buffer, self.offset).map_err(|_| FileHandleError::ReadIntoBufferFailed)?;
        Ok(self.read_value_from_buffer(&buffer))
    }

    fn read_preface(&mut self) -> FhResult<Option<EntryPreface<String>>> {
        let mut buffer = [0; HEADER_SIZE];
        self.file.read_exact_at(&mut buffer, self.offset).map_err(|_| FileHandleError::ReadIntoHeaderBufferFailed)?;
        Ok(self.read_preface_from_buffer(&buffer))
    }

    fn write(&mut self, entry: Entry<String, String>) -> FhResult<()> {
        self.write_key(entry.key)?;
        self.write_update(EntryUpdate { timestamp: entry.timestamp, value: entry.value })?;
        Ok(())
    }
    
    fn write_key(&mut self, key: String) -> FhResult<()> {
        self.key_size = KeySize::try_new(key.len() as u8);
        let key_size = vec![self.key_size.map(|k| k.value()).unwrap_or(0)];
        let key_bytes = key.as_bytes().to_vec();
        self.file.write_at(&mut [key_size, key_bytes].concat(), self.offset).map_err(|_| FileHandleError::KeyWriteFailed)?;
        Ok(())
    }

    fn write_update(&self, update: EntryUpdate<String>) -> FhResult<()> {
        let timestamp = update.timestamp.to_be_bytes().to_vec();
        let data_length = update.value.len().to_be_bytes().to_vec();
        let data = update.value.as_bytes().to_vec();
        self.file.write_at(&mut [timestamp, data_length, data].concat(), self.offset + self.key_size.clone().map(|k| k.value() as u64).unwrap_or(0) + 1).map_err(|_| FileHandleError::UpdateWriteFailed)?;
        Ok(())
    }
}
