use rawptr::MemReader;
use rawptr::types::ByteAddress;

fn main() {
    let dummy: u64 = 0x4847464544434241;
    let kk: u64 = 0x5453455454534554;
    let dummy2: u64 = 0x6867666564636261;

    let hello: u64 = 0x7473657474736574;
    println!("{}", hello);
    let reader: MemReader = MemReader::new().set_alignment(16);

    let ptr: *const u64 = &dummy;
    let adrs: ByteAddress = ByteAddress::from(ptr as u64);
    
//    let ptr2: *const u64 = &dummy2;
    println!("{}", kk);
    reader.print_with_range(adrs, -16, 16 * 20);

    let result = reader.read(adrs, 0, 22);
    println!("{:016x} | {:016x} | {:016x}", result[0], result[1], result[2]);

    let result = reader.read(adrs, 24, 8);
    println!("{:016x}", result[0]);

    let reader8: MemReader = MemReader::new().set_alignment(8);
    reader8.print(ByteAddress::from(result[0]));
    let result = reader8.read(ByteAddress::from(result[0]), 0, 8);
    println!();
    let reader = reader.set_alignment(32);
    reader.print_with_range(ByteAddress::from(result[0]), -32 * 20, 32 * 160);
}
