use std::ops::Add;
use std::ops::Sub;
use std::ops::Deref;

//pub type ByteAddress = *const u8;
#[derive(Debug, Clone, Copy)]
pub struct ByteAddress(*const u8);

pub trait ByteAddressMethods {

}

impl ByteAddressMethods for ByteAddress {

}

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
