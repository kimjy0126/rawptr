use rawptr::MemReader;
use rawptr::types::ByteAddress;

fn main() {
    let dummy: u64 = 0x4847464544434241;
    let dummy2: u64 = 0x6867666564636261;

    let reader: MemReader = MemReader::new().set_alignment(16);;

    let ptr: *const u64 = &dummy;
    let adrs: ByteAddress = ptr as usize as ByteAddress;
    reader.print_with_range(adrs, -16, 16 * 10);
}
