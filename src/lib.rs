pub mod mem_reader;
pub mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn types_from_u32() {
        use crate::types::*;
        let bytes: ByteValue = ByteValue::from_u32(0xdeadbeef);
        assert_eq!(bytes, [0xef, 0xbe, 0xad, 0xde]);
    }
}
