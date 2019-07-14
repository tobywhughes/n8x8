use n64::{cpu, rom, mips_iface, memory,rsp, rdram_iface, rdram_registers, rdram, icache};
use n64::exceptions::Exception;

pub struct Connector
{
    pub rom: rom::Rom,
    pub mips_interface: mips_iface::MipsInterface,
    pub rsp: rsp::RealitySignalProcessor,
    pub rdram_iface: rdram_iface::RDRAMInterface,
    pub rdram_registers: rdram_registers::RDRAMRegisters, 
    pub rdram: rdram::RDRAM,
    pub icache: icache::ICache,
}

impl Connector
{
    pub fn new(filename: &str) -> Connector
    {
        return Connector
        {
            rom: rom::Rom::new(filename),
            mips_interface: mips_iface::MipsInterface::new(),
            rsp: rsp::RealitySignalProcessor::new(),
            rdram_iface: rdram_iface::RDRAMInterface::new(),
            rdram_registers: rdram_registers::RDRAMRegisters::new(),
            rdram: rdram::RDRAM::new(),
            icache: icache::ICache::new()
        }
    }

    pub fn test() -> Connector
    {
        return Connector
        {
            rom: rom::Rom::test(),
            mips_interface: mips_iface::MipsInterface::new(),
            rsp: rsp::RealitySignalProcessor::new(),
            rdram_iface: rdram_iface::RDRAMInterface::new(),
            rdram_registers: rdram_registers::RDRAMRegisters::new(),
            rdram: rdram::RDRAM::new(),
            icache: icache::ICache::new()
        }
    }

    pub fn read_u32(&self, address: u32) -> Result<u32, Exception>
    {
        let mapping = memory::MemoryMapping::new(address);
        match mapping.sector
        {
            memory::Sector::SP_REG => Ok(self.rsp.read_u32_from_address(mapping.mapped_address as usize).unwrap()),
            memory::Sector::RI_REG => Ok(self.rdram_iface.read_u32_from_address(mapping.mapped_address as usize).unwrap()),
            memory::Sector::MI_REG => Ok(self.mips_interface.read_u32_from_address(mapping.mapped_address as usize)?),
            memory::Sector::RDRAM_REG => Ok(self.rdram_registers.read_u32_from_address(mapping.mapped_address as usize)?),
            memory::Sector::RDRAM_MEM => Ok(self.rdram.read_u32_from_address(mapping.mapped_address as usize)?),
            _ => Err(Exception::UNIMPLEMENTED_ADDRESS),
        }
    }

    pub fn read_u8(&self, mut address: u32) -> Result<u8, Exception>
    {
        let offset = address % 4;
        address -= offset;
        let mut u32_value: u32 = self.read_u32(address)?;
        Ok(((u32_value >> ((3 - offset) * 8) & 0x000000FF) as u8))
    }

    pub fn store_u32(&mut self, address:u32, value: u32) -> Result<(), Exception>
    {
        let mapping = memory::MemoryMapping::new(address);
        match mapping.sector
        {
            memory::Sector::RI_REG => self.rdram_iface.load_u32_to_address(mapping.mapped_address as usize, value).unwrap(),
            memory::Sector::SP_REG => self.rsp.load_u32_to_address(mapping.mapped_address as usize, value).unwrap(),
            memory::Sector::MI_REG => self.mips_interface.load_u32_to_address(mapping.mapped_address as usize, value)?,
            memory::Sector::RDRAM_REG => self.rdram_registers.load_u32_to_address(mapping.mapped_address as usize, value)?,
            memory::Sector::RDRAM_MEM => self.rdram.load_u32_to_address(mapping.mapped_address as usize, value)?,
            _ => return Err(Exception::UNIMPLEMENTED_ADDRESS),
        };
        Ok(())
    }

    pub fn store_u8(&mut self, mut address: u32, value: u8) -> Result<(), Exception>
    {
        let offset = address % 4;
        address -= offset;
        let mut u32_value: u32 = self.read_u32(address)?;
        self.store_u32(address, (u32_value & (!(0xFF << (8 * (3 - offset))))) | ((value as u32) << (8 * (3 - offset))));
        Ok(())
    }
}