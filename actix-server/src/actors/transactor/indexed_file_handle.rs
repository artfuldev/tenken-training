use std::{fs::File, os::unix::prelude::FileExt};

use super::entry::*;
use super::file_handle::*;

const SIZE_U64: u64 = 2048;
const HEADER_BUFFER_SIZE: usize = 109;

pub struct IndexedFileHandle {
    offset: u64,
    key_size: u64,
    file: File,
}

impl IndexedFileHandle {
    pub fn new(index: u64, file: File) -> Self {
        IndexedFileHandle {
            offset: index * SIZE_U64,
            key_size: 0,
            file,
        }
    }

    fn read_preface_from_buffer(&mut self, buffer: &[u8]) -> FhResult<Option<EntryPreface<String>>> {
        let key_size = buffer[0];
        self.key_size = u64::try_from(key_size).map_err(|_| FileHandleError::KeySizeConversionFailed)?;
        if key_size == 0 {
            return Ok(None);
        }
        let key_size_usize = usize::try_from(key_size).map_err(|_| FileHandleError::KeySizeConversionFailed)?;
        let key_bytes = buffer[1..key_size_usize].to_vec();
        let key = String::from_utf8(key_bytes).map_err(|_| FileHandleError::KeyReadFailed)?;
        let timestamp_bytes = buffer[(key_size_usize)..(key_size_usize + 8)].to_vec().try_into().map_err(|_| FileHandleError::TimestampReadFailed)?;
        let timestamp = u64::from_be_bytes(timestamp_bytes);
        if timestamp == 0 {
            return Ok(None);
        }
        Ok(Some(EntryPreface { key, timestamp }))
    }

    fn read_value_from_buffer(&self, buffer: &[u8]) -> FhResult<Option<String>> {
        if self.key_size == 0 {
            return Ok(None);
        }
        let header_offset = usize::try_from(self.key_size + 8).map_err(|_| FileHandleError::KeySizeConversionFailed)?;
        let data_length = u64::from_be_bytes(buffer[(header_offset)..(8 + header_offset)].try_into().map_err(|_| FileHandleError::ReadDataLengthFailed)?);
        let data_length_offset = usize::try_from(data_length).map_err(|_| FileHandleError::DataLengthOffsetConversionFailed)?;
        let data = String::from_utf8(buffer[(8 + header_offset)..(8 + header_offset + data_length_offset)].to_vec()).map_err(|_| FileHandleError::ReadDataAsStringFailed)?;
        Ok(Some(data))
    }
}

impl FileHandle<String, String> for IndexedFileHandle {

    fn read(&mut self) -> FhResult<Option<Entry<String, String>>> {
        let mut buffer: [u8; 2048] = [0; 2048];
        self.file.read_exact_at(&mut buffer, self.offset).map_err(|_| FileHandleError::ReadIntoBufferFailed)?;
        match self.read_preface_from_buffer(&buffer) {
            Err(e) => Err(e),
            Ok(_preface) => {
                match _preface {
                    None => Ok(None),
                    Some(preface) => {
                        match self.read_value_from_buffer(&buffer) {
                            Err(e) => Err(e),
                            Ok(_value) => {
                                match _value {
                                    None => Ok(None),
                                    Some(value) => Ok(Some(Entry { key: preface.key, timestamp: preface.timestamp, value }))
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn read_value(&self) -> FhResult<Option<String>> {
        let mut buffer: [u8; 2048] = [0; 2048];
        self.file.read_exact_at(&mut buffer, self.offset).map_err(|_| FileHandleError::ReadIntoBufferFailed)?;
        self.read_value_from_buffer(&buffer)
    }

    fn read_preface(&mut self) -> FhResult<Option<EntryPreface<String>>> {
        let mut buffer = [0; HEADER_BUFFER_SIZE];
        self.file.read_exact_at(&mut buffer, self.offset).map_err(|_| FileHandleError::ReadIntoHeaderBufferFailed)?;
        self.read_preface_from_buffer(&buffer)
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
