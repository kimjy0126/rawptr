extern crate rawptr;

use rawptr::MemReader;
use rawptr::types::ByteAddress;

fn main() {
    let v: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let pv: ByteAddress = ByteAddress::from(&v as *const Vec<u8> as u64);

    let reader: MemReader = MemReader::new().set_alignment(6);
    let ppv = ByteAddress::from(reader.read(pv, 0, 8)[0]);

    reader.print_with_range(ppv, 0, 15);
    println!("\x1b[1;34mYou can see the addresses on the left side of \':\' are always divided by 6");
    println!("This is because in this example, the alignment value of reader is set to 6. This is alignmnet!");
    println!();
    println!("Also, you can always see whole vector values with three or four lines.");
    println!("This is part of alignment too.\x1b[0m");
}
