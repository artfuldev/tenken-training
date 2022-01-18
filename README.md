# tenken-training
A place where we train for the tenken submission

# Outline

To support a pre-determined number of probes, we use a database that has pre
allocated space for each probe. The probe's payload can be viewed as a composite
of 2 parts: fixed size + variable size. The fixed size bytes are stored in a
predetermined format while the variable size bytes are stored with a length
encoding.

So each payload is of the form:

`|fixed_bytes (f bytes)|variable_size (4 bytes)|variable_bytes (v bytes)|`

Since the maximum size of the payload is predetermined, the space for n probes
can be pre-allocated. Every single partition of this structure follows the form:

`|key_size (2 bytes)|key_bytes (k bytes)|event_timestamp (8 bytes)| probe_payload (f + 4 + v bytes)|`

When the database is started, it goes through all the partitions. Every
partition starts with the key size. if the `key_size` is `0x00` it means it's an
empty partition. By reading the first (2 + k + 8) bytes of every partition,
there is enough data about where to start reading the probe payload inside the
partition. Every partition has an index, so coupled with the index, calculating
the byte offset to read the probe payload is a simple arithmetic operation:

`payload_offset = (payload_index * max_payload_size) + k + 10`

The combined information of `timestamp, payload_offset` is stored against every
`key` in a hashmap as an index cache upon reading the partitions one by one.

Before writing to the database, the in-memory hashmap can be looked up quickly
for the latest timestamp without reading from disk in order to drop writes to
disk when the data requested to be written is older.

This also means, to store an index in memory, we do not need to hold the entire
payload in memory to service READ requests. The index is always in memory, while
the data is persisted to disk and is only used to service READ requests. The
only data point required to save on writes which is expensive is the timestamp,
and that information is always held in working memory of the system.

There are separate read and write handles to allow for lock-free concurrency.
The read handles can be cloned separately and shared multiple times while the
writer is single-threaded. This prevents inconsistent writes. Writes to disk are
done partition by partition, by using an indexed write.

When the data is read on boot, the database file is read partition by partition
and all empty partitions are pushed to a vacant queue. When new write requests
arrive, the probe id is looked up in the index. If the key is not present in the
index, a partition is pulled from the vacant queue. If the vacant queue is empty
on a write request, the write request fails or is dropped as the capacity of the
database has been exceeded.

The timestamp and payload offset can be stored in a single unsigned u64 by
combining 2 u32 parts into one with a 4 byte offset in order to save in-memory
space for the index.
