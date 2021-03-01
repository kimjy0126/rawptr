/// Struct which is used to read virtual memory contents.
pub struct MemReader {
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

    /// Set `offset` value for `MemReader`.
    ///
    /// `offset` notates where `MemReader` starts to read memory. For example, if `offset` is `-8`
    /// and given address is `0x8888`, `Memreader` starts reading at `0x8880`, which is derived
    /// from given address + `offset`.
    pub fn set_offset(mut self, offset: i32) -> MemReader {
        self.offset = offset;
        self
    }

    /// Set `range` value for `MemReader`
    ///
    /// `range` notates how many bytes `MemReader` should read. For example, if `range` is `16`,
    /// and the starting address is `0x8880`, `MemReader` reads `16` bytes from `0x8880` and ends
    /// up at `0x8890`, which derived from starting address + `range`.
    pub fn set_range(mut self, range: u32) -> MemReader {
        self.range = range;
        self
    }

    pub fn alignment(mut self, alignment: u32) -> MemReader {
        self.alignment = alignment;
        self
    }
}
