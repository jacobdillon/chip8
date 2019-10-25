mod chip8;

use chip8::ops::Op;
use std::env;
use std::fs;
use std::io::Result;

fn main() {
    let path = env::args().nth(1);

    if path.is_none() {
        eprintln!("Missing expected file path as first argument.");
        return;
    }
    let path = path.unwrap();

    let bytes: Result<Vec<_>> = fs::read(&path).map(|vec| vec.iter().map(|&byte| u8::from_be(byte)).collect());
    match bytes {
        Ok(bytes) => {
            bytes.chunks(2)
                .filter(|slice| slice.len() == 2)
                .map(from_u8s)
                .for_each(|byte| {
                    println!("{:04X}: {:02X?}", byte, Op::from_bin(byte));
                });
        }
        Err(_) => eprintln!("The file \"{}\" does not exist.", path),
    }
}

fn from_u8s(slice: &[u8]) -> u16 {
    (slice[0] as u16) << 8 | slice[1] as u16
}