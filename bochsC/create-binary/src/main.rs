use std::io::Write;

fn main() {
    let filename = std::env::args().nth(1).expect("Please specify filename as first argument");

    let mut bytes = [0u8; 512];

    bytes[0] = 0xe9;
    bytes[1] = 0xfd;
    bytes[2] = 0xff;
    bytes[bytes.len() - 2 ] = 0x55;
    bytes[bytes.len() - 1 ] = 0xaa;

    std::fs::File::create(filename).expect("failed to open file").write_all(&bytes).expect("failed to write content to file");
}
