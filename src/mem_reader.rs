/// Struct which is used to read virtual memory contents.
pub struct MemReader {
    /// Notate where reader starts to read memory.
    /// For example, if `offset` is -8 and given address is 0x8888, reader starts at 0x8880, which
    /// is derived from given address + `offset`.
    offset: i32,
    range: u32,
    alignment: u32,
}

impl MemReader {
    pub fn new() -> MemReader {
        MemReader {
            offset: 0,
            range: 8,
            alignment: 8,
        }
    }
}
