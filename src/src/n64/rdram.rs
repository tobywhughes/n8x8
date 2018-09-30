const RDRAM_RANGE_0_START: usize = 0x00000000;
const RDRAM_RANGE_0_END: usize = 0x001FFFFF;
const RDRAM_RANGE_1_START: usize = 0x00200000;
const RDRAM_RANGE_1_END: usize = 0x003FFFFF;
const RDRAM_RANGE_2_START: usize = 0x00400000;
const RDRAM_RANGE_2_END: usize = 0x007FFFFF;

use std::io::{Error, ErrorKind};
use binary_helpers::*;

pub struct RDRAM
{
    pub range0: Vec<u8>,
    pub range1: Vec<u8>,
    pub range2: Vec<u8>
}

impl RDRAM
{
    pub fn new() -> RDRAM
    {
        return RDRAM
        {
            range0: vec![0; 0x200000],
            range1: vec![0; 0x200000],
            range2: vec![0; 0x400000],
        }
    }

    pub fn read_u32_from_address(&self, address: usize) -> Result<u32, Error>
    {
        //Only allow alligned addresses (unaligned handled exterior to function)
        if address % 4 != 0
        {
            return Err(Error::new(ErrorKind::Other, "Unaligned Address Call."));
        }

        match address
        {
            RDRAM_RANGE_0_START...RDRAM_RANGE_0_END => Ok(u8_slice_to_u32(self.range0[(address - RDRAM_RANGE_0_START)..((address - RDRAM_RANGE_0_START) + 4)].to_vec())),
            RDRAM_RANGE_1_START...RDRAM_RANGE_1_END => Ok(u8_slice_to_u32(self.range1[(address - RDRAM_RANGE_1_START)..((address - RDRAM_RANGE_1_START) + 4)].to_vec())),
            RDRAM_RANGE_2_START...RDRAM_RANGE_2_END => Ok(u8_slice_to_u32(self.range2[(address - RDRAM_RANGE_2_START)..((address - RDRAM_RANGE_2_START) + 4)].to_vec())),
            _ => Err(Error::new(ErrorKind::Other, "Unused rdram memory address.")),
        }
    }


        pub fn load_u32_to_address(&mut self, address: usize, value: u32) -> Result<(), Error>
    {
        //Only allow alligned addresses (unaligned handled exterior to function)
        if address % 4 != 0
        {
            return Err(Error::new(ErrorKind::Other, "Unaligned Address Call."));
        }
        match address
        {
            RDRAM_RANGE_0_START...RDRAM_RANGE_0_END => Ok(u32_to_u8_vector_by_loc(value, address - RDRAM_RANGE_0_START, &mut self.range0)),
            RDRAM_RANGE_1_START...RDRAM_RANGE_1_END => Ok(u32_to_u8_vector_by_loc(value, address - RDRAM_RANGE_1_START, &mut self.range1)),
            RDRAM_RANGE_2_START...RDRAM_RANGE_2_END => Ok(u32_to_u8_vector_by_loc(value, address - RDRAM_RANGE_2_START, &mut self.range2)),
            _ => Err(Error::new(ErrorKind::Other, "Unused rdram memory address.")),
        }
    }
}