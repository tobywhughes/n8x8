mod n64;
mod binary_helpers;

use n64::memory::MemoryMapping;
use n64::rom::RomHeader;
use n64::rom::Rom;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    let rom = Rom::new(filename);
    println!("{:x}", rom.rom_data[0]);
}
