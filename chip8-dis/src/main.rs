mod chip8;

use chip8::ops::Op;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let file = env::args().skip(1)
                          .nth(0)
                          .and_then(|path| File::open(path).ok());

    if file.is_none() {
        eprintln!("Missing expected file path as first argument or the specified file does not exist.");
        return;
    }
    
    let file = file.unwrap();

    file.bytes()
        .filter_map(Result::ok)
        .map(|b| (b as u16).swap_bytes())
        .filter_map(Op::from_bin)
        .zip(1..)
        .for_each(|(op, i)| {
            println!("{:04X}: {:02X?}", i, op);
        });
}
