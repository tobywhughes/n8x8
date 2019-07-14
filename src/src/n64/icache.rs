pub struct ICache { 
    pub line: Vec<ICacheLine>
}

impl ICache { 
    pub fn new() -> ICache { 
        return ICache {
            line: vec![ICacheLine::new(); 512]
        }
    }

    pub fn parse_line_index_from_virtual_index(&self, virtual_address: u32) -> u32{
        return (virtual_address & 0x00003FE0) >> 5;
    }
}

#[derive(Clone)]
pub struct ICacheLine {
    pub valid: bool,
    physical_tag: u32,
    pub data: Vec<u32>,
}

impl ICacheLine { 
    pub fn new() -> ICacheLine {
        return ICacheLine {
            valid: false,
            physical_tag: 0,
            data: vec![0_u32;8]
        }
    }

    pub fn get_physical_tag(&self) -> u32 { self.physical_tag & 0x000FFFFF }
    pub fn set_physical_tag(&mut self, value: u32 ) { self.physical_tag = value & 0x000FFFFF }
}