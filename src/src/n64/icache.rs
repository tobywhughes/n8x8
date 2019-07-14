//16kb
//512 lines

pub struct CacheLine {
    pub valid: bool,
    physical_tag: u32,
    pub data: Vec<u32>,
}

impl CacheLine { 
    pub fn new() -> CacheLine {
        return CacheLine {
            valid: false,
            physical_tag: 0,
            data: vec![0_u32;8]
        }
    }

    pub fn get_physical_tag(&self) -> u32 { self.physical_tag & 0x000FFFFF }
    pub fn set_physical_tag(&mut self, value: u32 ) { self.physical_tag = value & 0x000FFFFF }
}