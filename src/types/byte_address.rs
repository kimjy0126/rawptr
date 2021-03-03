use std::ops::Add;
use std::ops::Sub;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ByteAddress(pub *const u8);

impl Add<u64> for ByteAddress {
    type Output = Self;

    fn add(self, other: u64) -> Self {
        ByteAddress((self.0 as u64 + other) as *const u8)
    }
}

impl Sub<u64> for ByteAddress {
    type Output = Self;

    fn sub(self, other: u64) -> Self {
        ByteAddress((self.0 as u64 - other) as *const u8)
    }
}

impl Deref for ByteAddress {
    type Target = *const u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u64> for ByteAddress {
    fn from(item: u64) -> Self {
        Self(item as *const u8)
    }
}
