/// * Item	Start Index	Size	End Index	End Index	Slice
///  * Key Size	0	1	1 + 0 - 1	0	0..0
///  * Key	1	k	1 + k - 1	k	1..=k
///  * Timestamp	k + 1	8	k + 1 + 8 - 1	k + 8	k+1..=k+8
///  * Value Size	k + 9	8	k + 9 + 8 - 1	k + 16	k+9..=k+16
///  * Value	k + 17	v	k + 17 + v - 1	k + v + 16	k+17..=k+v+16

pub const PARTITION_SIZE: usize = 2048;
pub const HEADER_SIZE: usize = 109;

#[derive(Debug, Clone, Copy)]
pub struct KeySize(u8);

impl KeySize {
    pub fn try_new(value: u8) -> Option<KeySize> {
        Some(value)
            .filter(|&x| x > 2u8 && x < 101u8)
            .map(|x| KeySize(x))
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn usize(&self) -> usize {
        self.0 as usize
    }
}

pub fn key_size(buffer: &[u8]) -> Option<KeySize> {
    KeySize::try_new(buffer[0])
}

pub fn key(key_size: KeySize, buffer: &[u8]) -> Option<String> {
    String::from_utf8(buffer[1..=key_size.usize()].to_vec()).map_or(None, Some)
}

pub fn timestamp(key_size: KeySize, buffer: &[u8]) -> Option<u64> {
    buffer[(key_size.usize() + 1)..=(key_size.usize() + 8)]
        .try_into()
        .map_or(None, |x| Some(u64::from_be_bytes(x)))
}

pub fn value_size(key_size: KeySize, buffer: &[u8]) -> Option<u64> {
    buffer[(key_size.usize() + 9)..=(key_size.usize() + 16)]
        .try_into()
        .map_or(None, |x| Some(u64::from_be_bytes(x)))
}

pub fn value(key_size: KeySize, value_size: u64, buffer: &[u8]) -> Option<String> {
    String::from_utf8(
        buffer[(key_size.usize() + 17)..=(key_size.usize() + (value_size as usize) + 16)]
            .to_vec())
        .map_or(None, Some)
}

