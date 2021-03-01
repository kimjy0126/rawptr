pub type ByteValue = Vec<u8>;

pub trait ByteValueMethods {
    fn to_u32(&self) -> Vec<u32>;
    fn from_u32(val: u32) -> ByteValue;
}

impl ByteValueMethods for ByteValue {
    #![allow(unused)]
    fn to_u32(&self) -> Vec<u32> {
        let length: usize = self.len();
        vec![]
    }

    fn from_u32(val: u32) -> ByteValue {
        vec![(0x000000ff & val) as u8, ((0x0000ff00 & val) >> 8) as u8,
             ((0x00ff0000 & val) >> 16) as u8, (((0xff000000) & val) >> 24) as u8]
    }
}
