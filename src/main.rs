use rawptr::MemReader;
use rawptr::types::ByteAddress;

fn main() {
    let dummy: u64 = 0xdeadbeef;

    let reader: MemReader = MemReader::new();

    let ptr: *const u64 = &dummy;
    let adrs: ByteAddress = ptr as usize as ByteAddress;
    reader.print_with_range(adrs, -8, 16);
}
