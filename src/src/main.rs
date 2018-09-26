extern crate num;
#[macro_use]
extern crate num_derive;

mod n64;
mod binary_helpers;

use n64::memory::MemoryMapping;
use n64::cpu::CPU;
use n64::rom::RomHeader;
use n64::rom::Rom;
use n64::connector::Connector;
use n64::n64::N64;
use std::env;

fn main() 
{
    let filename = get_filename();
    let mut n64: N64 = N64::new(&filename);
    n64.run_pif_rom();
    n64.register_debug();
    n64.run();
}

fn get_filename() -> String
{
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 1
    {
        panic!("Emulator needs a rom to function!");
    }
    return String::from(args.remove(1));
}