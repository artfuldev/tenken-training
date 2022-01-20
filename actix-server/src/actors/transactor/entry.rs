#[derive(Debug)]
pub struct EntryPreface<K> {
    pub key: K,
    pub timestamp: u64,
}

#[derive(Debug)]
pub struct Entry<K, V> {
    pub key: K,
    pub timestamp: u64,
    pub value: V
}

#[derive(Debug)]
pub struct EntryUpdate<V> {
    pub timestamp: u64,
    pub value: V
}
