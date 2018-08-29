extern crate num;
#[macro_use]
extern crate num_derive;

mod n64;
mod binary_helpers;

use n64::memory::MemoryMapping;
use n64::cpu::CPU;
use n64::rom::RomHeader;
use n64::rom::Rom;
use std::env;

fn main() {
    let mut cpu: CPU = CPU::new();
    cpu.cpu_registers.set_pif_rom_values();
    cpu.cop0_registers.set_pif_rom_values();
    cpu.cpu_registers.Debug();
    cpu.cop0_registers.Debug();
    // let args: Vec<String> = env::args().collect();
    // let filename: &String = &args[1];
    // let rom = Rom::new(filename);
    // rom.rom_header.debug();
}
