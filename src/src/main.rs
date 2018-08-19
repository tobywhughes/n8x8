mod n64;
use n64::memory::MemoryMapping;

fn main() {
    let test = MemoryMapping::new(0);
    println!("{} -- {}", test.sector.to_string(), test.mapped_address)
}
