mod chip8;

use chip8::ops::Op;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let file = env::args().nth(1).and_then(|path| File::open(path).ok());

    if file.is_none() {
        eprintln!("Missing expected file path as first argument or the specified file does not exist.");
        return;
    }
    
    let file = file.unwrap();
    let bytes = file.bytes()
                    .filter_map(Result::ok)
                    .map(|byte| u8::from_be(byte))
                    .collect::<Vec<_>>();
    
    bytes.chunks(2)
        .filter(|slice| slice.len() == 2)
        .map(from_u8s)
        .for_each(|byte| {
            println!("{:04X}: {:02X?}", byte, Op::from_bin(byte));
        });
}

fn from_u8s(slice: &[u8]) -> u16 {
    (slice[0] as u16) << 8 | slice[1] as u16
}