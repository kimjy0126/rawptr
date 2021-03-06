#[derive(Debug, PartialEq)]
pub struct ByteValue(pub Vec<u8>);

impl ByteValue {
    pub fn from_vec(v: Vec<u8>) -> ByteValue {
        ByteValue(v)
    }

    pub fn to_u32(&self) -> Vec<u32> {
        let length: usize = self.0.len();
        let mut result: Vec<u32> = vec![];

        for i in 0..length / 4 {
            let mut val: u32 = 0;
            for j in 0..4 {
                val += ((self.0[i * 4 + j]) as u32) << (8 * j);
            }
            result.push(val);
        }

        if length % 4 != 0 {
            let mut val: u32 = 0;
            for j in 0..length % 4 {
                val += ((self.0[length / 4 * 4 + j]) as u32) << (8 * j);
            }
            result.push(val);
        }

        result
    }

    pub fn from_u32(val: u32) -> ByteValue {
        ByteValue(vec![(0x000000ff & val) as u8, ((0x0000ff00 & val) >> 8) as u8,
                  ((0x00ff0000 & val) >> 16) as u8, (((0xff000000) & val) >> 24) as u8])
    }
}
