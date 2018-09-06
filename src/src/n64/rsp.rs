const SP_DMEM_START: usize =  0x00000000;
const SP_DMEM_END: usize = 0x00000FFF;
const SP_IMEM_START: usize = 0x00001000;
const SP_IMEM_END: usize = 0x00001FFF;
const SP_MEM_ADDR_REG_START: usize = 0x00040000;
const SP_MEM_ADDR_REG_END: usize = 0x00040003;
const SP_DRAM_ADDR_REG_START: usize = 0x00040004;
const SP_DRAM_ADDR_REG_END: usize = 0x00040007;
const SP_RD_LEN_REG_START: usize = 0x00040008;
const SP_RD_LEN_REG_END: usize = 0x0004000B;
const SP_WR_LEN_REG_START: usize = 0x0004000C;
const SP_WR_LEN_REG_END: usize = 0x0004000F;
const SP_STATUS_REG_START: usize = 0x00040010;
const SP_STATUS_REG_END: usize = 0x00040013;
const SP_DMA_FULL_REG_START: usize = 0x00040014;
const SP_DMA_FULL_REG_END: usize = 0x00040017;
const SP_DMA_BUSY_REG_START: usize = 0x00040018;
const SP_DMA_BUSY_REG_END: usize = 0x0004001B;
const SP_SEMAPHORE_REG_START: usize = 0x0004001C;
const SP_SEMAPHORE_REG_END: usize = 0x0004001F;
const SP_PC_REG_START: usize = 0x00080000;
const SP_PC_REG_END: usize =  0x00080003;
const SP_IBIST_REG_START: usize = 0x00080004;
const SP_IBIST_REG_END: usize = 0x00080007;

use n64::arch::Reg;
use binary_helpers::*;


pub struct RealitySignalProcessor
{
    pub dynamic_memory: Vec<u8>,
    pub instruction_memory: Vec<u8>,
    pub memory_address: Reg, 
    pub dram_dam_address: Reg,
    pub read_dma_length: Reg,
    pub write_dma_length: Reg,
    pub status: Reg,
    pub dma_full: Reg,
    pub dma_busy: Reg,
    pub sempahore: Reg,
    pub program_counter: Reg,
    pub instruction_memory_self_test: Reg,
}

impl RealitySignalProcessor
{
    pub fn new() -> RealitySignalProcessor
    {
        return RealitySignalProcessor
        {
            dynamic_memory: vec![0; 0x1000],
            instruction_memory: vec![0; 0x1000],
            memory_address: Reg::default(), 
            dram_dam_address: Reg::default(),
            read_dma_length: Reg::default(),
            write_dma_length: Reg::default(),
            status: Reg::default(),
            dma_full: Reg::default(),
            dma_busy: Reg::default(),
            sempahore: Reg::default(),
            program_counter: Reg::default(),
            instruction_memory_self_test: Reg::default(),
        }
    }

    pub fn read_u32_from_address(&self, address: usize) -> Option<u32>
    {
        //Only allow alligned addresses (unaligned handled exterior to function)
        if address % 4 != 0
        {
            return None;
        }

        match address
        {
            SP_DMEM_START...SP_DMEM_END => Some(u8_slice_to_u32(self.dynamic_memory[(address - SP_DMEM_START)..((address - SP_DMEM_START) + 4)].to_vec())),
            SP_IMEM_START...SP_IMEM_END => Some(u8_slice_to_u32(self.instruction_memory[(address - SP_IMEM_START)..((address - SP_IMEM_START) + 4)].to_vec())),
            SP_MEM_ADDR_REG_START...SP_MEM_ADDR_REG_END => Some(self.memory_address.get_value() as u32),
            SP_DRAM_ADDR_REG_START...SP_DRAM_ADDR_REG_END => Some(self.dram_dam_address.get_value() as u32),
            SP_RD_LEN_REG_START...SP_RD_LEN_REG_END => Some(self.read_dma_length.get_value() as u32),
            SP_WR_LEN_REG_START...SP_WR_LEN_REG_END => Some(self.write_dma_length.get_value() as u32),
            SP_STATUS_REG_START...SP_STATUS_REG_END => Some(self.status.get_value() as u32),
            SP_DMA_FULL_REG_START...SP_DMA_FULL_REG_END => Some(self.dma_full.get_value() as u32),
            SP_DMA_BUSY_REG_START...SP_DMA_BUSY_REG_END => Some(self.dma_busy.get_value() as u32),
            SP_SEMAPHORE_REG_START...SP_SEMAPHORE_REG_END => Some(self.sempahore.get_value() as u32),
            SP_PC_REG_START...SP_PC_REG_END => Some(self.program_counter.get_value() as u32),
            SP_IBIST_REG_START...SP_IBIST_REG_END => Some(self.instruction_memory_self_test.get_value() as u32),
            _ => None,
        }
    }

    pub fn read_u16_from_address(&self, address: usize) -> Option<u16>
    {
        //Only allow alligned addresses (unaligned handled exterior to function)
        if address % 2 != 0
        {
            return None;
        }

        match address
        {
            SP_DMEM_START...SP_DMEM_END => Some(u8_slice_to_u16(self.dynamic_memory[(address - SP_DMEM_START)..((address - SP_DMEM_START) + 2)].to_vec())),
            SP_IMEM_START...SP_IMEM_END => Some(u8_slice_to_u16(self.instruction_memory[(address - SP_IMEM_START)..((address - SP_IMEM_START) + 2)].to_vec())),
            _ => None,
        }
    }

    pub fn read_u8_from_address(&self, address: usize) -> Option<u8>
    {
        match address
        {
            SP_DMEM_START...SP_DMEM_END => Some(self.dynamic_memory[address - SP_DMEM_START]),
            SP_IMEM_START...SP_IMEM_END => Some(self.instruction_memory[address - SP_IMEM_START]),
            _ => None,
        }
    }

    pub fn load_u8_to_address(&mut self, address: usize, value: u8) -> Result<(), usize>
    {
        match address
        {
            SP_DMEM_START...SP_DMEM_END => Ok(self.dynamic_memory[address  - SP_DMEM_START] = value),
            SP_IMEM_START...SP_IMEM_END => Ok(self.instruction_memory[address - SP_IMEM_START] = value),
            _ => Err(address),
        }
    }

    pub fn copy_bytes_from_u8_vector(&mut self, start_address: usize, source_vector: Vec<u8>, bytes: usize) -> Result<(), usize>
    {
        for offset in 0..bytes
        {
            match self.load_u8_to_address(start_address + offset, source_vector[offset])
            {
                Err(error_address) => return Err(error_address),
                _ => (),
            }
        }
        return Ok(())
    }
}