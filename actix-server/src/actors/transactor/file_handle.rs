use std::error;

use thiserror::Error;

use super::entry::*;

#[derive(Debug, Error)]
pub enum FileHandleError {
    #[error("failed to read into buffer")]
    ReadIntoBufferFailed,
    #[error("failed to read data length")]
    ReadDataLengthFailed,
    #[error("failed to convert data length offset")]
    DataLengthOffsetConversionFailed,
    #[error("failed to read as string")]
    ReadDataAsStringFailed,
    #[error("failed to read into header buffer")]
    ReadIntoHeaderBufferFailed,
    #[error("failed to convert key size")]
    KeySizeConversionFailed,
    #[error("failed to read key")]
    KeyReadFailed,
    #[error("failed to read timestamp")]
    TimestampReadFailed,
    #[error("failed to write key")]
    KeyWriteFailed,
    #[error("failed to write update")]
    UpdateWriteFailed
}

pub type FhResult<T> = Result<T, FileHandleError>;

pub trait FileHandle<K, V> {
    fn read(&mut self) -> FhResult<Option<Entry<K, V>>>;
    fn read_value(&self) -> FhResult<Option<V>>;
    fn read_preface(&mut self) -> FhResult<Option<EntryPreface<K>>>;
    fn write(&mut self, entry: Entry<K, V>) -> FhResult<()>;
    fn write_key(&mut self, key: K) -> FhResult<()>;
    fn write_update(&self, update: EntryUpdate<V>) -> FhResult<()>;
}