extern crate rawptr;

use rawptr::MemReader;
use rawptr::types::ByteAddress;

fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32];
    let pv: *const Vec<i32> = &v;

    let reader: MemReader = MemReader::new().set_alignment(32);
    reader.print_with_range(ByteAddress::from(pv as u64), 0, 4 * 32);
    println!("\x1b[1;34mYou can see the address of the real vector contents, and the length of the vector\x1b[0m\n");

    let ppv: ByteAddress = ByteAddress::from(reader.read(ByteAddress::from(pv as u64), 0, 8)[0]);
    reader.print_with_range(ppv, 0, 4 * 32);
    println!("\x1b[1;34mThis is what is in the address of the real vector: the real contents!\x1b[0m");
}
