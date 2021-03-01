pub mod mem_reader;
pub mod types;

pub use mem_reader::MemReader;

#[cfg(test)]
mod tests {
    use super::types::*;

    #[test]
    fn types_from_u32() {
        let bytes: ByteValue = ByteValue::from_u32(0xdeadbeef);
        assert_eq!(bytes, [0xef, 0xbe, 0xad, 0xde]);
    }

    #[test]
    fn types_to_u32() {
        let bytes: ByteValue = vec![0xef, 0xbe, 0xad, 0xde, 0xfe, 0xca, 0x21, 0x20];
        let result: Vec<u32> = bytes.to_u32();

        assert_eq!(result, [0xdeadbeef, 0x2021cafe]);
    }
}
