use rawptr::MemReader;
use rawptr::types::ByteAddress;

fn main() {
    let dummy: u64 = 0x4847464544434241;
    let kk: u64 = 0x5453455454534554;
    let dummy2: u64 = 0x6867666564636261;

    let hello: u64 = 0x7473657474736574;
//    println!("{}", hello);
    let reader: MemReader = MemReader::new().set_alignment(16);

    let ptr: *const u64 = &dummy;
    let adrs: ByteAddress = ptr as usize as ByteAddress;
    
//    let ptr2: *const u64 = &dummy2;
//    println!("{}", kk);
//    let ptr2: *const u64 = &dummy2;
    reader.print_with_range(adrs, -16, 16 * 20);
}
