use std::char;

use crate::types::*;

/// Struct which is used to read virtual memory contents.
pub struct MemReader {
    offset: i32,
    range: u32,
    alignment: u32,
}

impl MemReader {
    /// Make `MemReader` with default config.
    ///
    /// In default config, `offset` is `0`, `range` is `8`, and `alignment` is `8`.
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

    /// Set `range` value for `MemReader`.
    ///
    /// `range` notates how many bytes `MemReader` should read. For example, if `range` is `16`,
    /// and the starting address is `0x8880`, `MemReader` reads `16` bytes from `0x8880` and ends
    /// up at `0x8890`, which derived from starting address + `range`.
    pub fn set_range(mut self, range: u32) -> MemReader {
        self.range = range;
        self
    }

    pub fn set_alignment(mut self, alignment: u32) -> MemReader {
        self.alignment = alignment;
        self
    }

    /// Prints what is in the given address, with preset config.
    pub fn print(&self, address: ByteAddress) {
        self.print_with_range(address, self.offset, self.range);
    }

    /// Prints what is in the given address, with given config.
    pub fn print_with_range(&self, address: ByteAddress, offset: i32, range: u32) {
        let starting_address: ByteAddress = (address as i64 + offset as i64) as ByteAddress;
        let alignment = self.alignment;

        for i in 0..range / alignment {
            print!("\x1b[0;32m{:?}\x1b[0m: ", (starting_address as u64 + (i * alignment) as u64) as ByteAddress);
            let mut ch: Vec<char> = vec![];
            for j in 0..alignment {
                unsafe {
                    let value: u8 = *((starting_address as u64 + (i * alignment) as u64 + j as u64) as *const u8);
                    print!("{:02x} ", value);
                    if 32 <= value && value <= 126 {
                        ch.push(char::from_u32(value as u32).unwrap());
                    } else {
                        ch.push('.');
                    }
                }
            }
            print!("        ");
            for c in ch.iter() {
                print!("{}", c);
            }
            println!();
        }
    }

    pub fn read(&self, address: ByteAddress, offset: i32, range: u32) -> Vec<u64> {
        let starting_address: ByteAddress = (address as i64 + offset as i64) as ByteAddress;
        let mut result: Vec<u64> = vec![];

        for i in 0..range / 8 {
            let mut val: u64 = 0;
            for j in 0..8 {
                unsafe {
                    let value: u8 = *((starting_address as u64 + (i * 8) as u64 + j as u64) as *const u8);
                    val += ((value as u64) << (8 * j)) as u64;
                }
            }
            result.push(val);
        }

        result
    }
}
