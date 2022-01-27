use thiserror::Error;

use super::entry::*;

#[derive(Debug, Error)]
pub enum FileHandleError {
    #[error("failed to read into buffer")]
    ReadIntoBufferFailed,
    #[error("failed to read into header buffer")]
    ReadIntoHeaderBufferFailed,
    #[error("failed to write buffer")]
    WriteBufferFailed
}

pub type FhResult<T> = Result<T, FileHandleError>;

pub trait FileHandle<K, V> {
    fn read(&mut self) -> FhResult<Option<Entry<K, V>>>;
    fn read_value(&self) -> FhResult<Option<V>>;
    fn read_preface(&mut self) -> FhResult<Option<EntryPreface<K>>>;
    fn write(&mut self, entry: Entry<K, V>) -> FhResult<()>;
    fn write_update(&self, update: EntryUpdate<V>) -> FhResult<()>;
}
