use super::entry::*;

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

pub type FhResult<T> = Result<T, FileHandleError>;

pub trait FileHandle<K, V> {
    fn read(&mut self) -> FhResult<Option<Entry<K, V>>>;
    fn read_value(&self) -> FhResult<Option<V>>;
    fn read_preface(&mut self) -> FhResult<Option<EntryPreface<K>>>;
    fn write(&mut self, entry: Entry<K, V>) -> FhResult<()>;
    fn write_key(&mut self, key: K) -> FhResult<()>;
    fn write_update(&self, update: EntryUpdate<V>) -> FhResult<()>;
}
