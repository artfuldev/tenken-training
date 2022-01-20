pub struct EntryPreface<K> {
    pub key: K,
    pub timestamp: u64,
}

pub struct Entry<K, V> {
    pub key: K,
    pub timestamp: u64,
    pub value: V
}

pub struct EntryUpdate<V> {
    pub timestamp: u64,
    pub value: V
}
