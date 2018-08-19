mod n64;
mod binary_helpers;

use n64::memory::MemoryMapping;
use n64::rom::RomHeader;

fn main() {
    let test = MemoryMapping::new(0);
    println!("{} -- {}", test.sector.to_string(), test.mapped_address);
    let mut input: Vec<u8> = vec![0; 0x1000];
    for num in 0..0x100
    {
        input[num] = num as u8;
    }
    let rom_header = RomHeader::new(input);

}
