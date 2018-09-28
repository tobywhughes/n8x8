use n64::{cpu, rom, mips_iface, memory,rsp, rdram_iface};
use std::io::{Error, ErrorKind};

pub struct Connector
{
    pub rom: rom::Rom,
    pub mips_interface: mips_iface::MipsInterface,
    pub rsp: rsp::RealitySignalProcessor,
    pub rdram_iface: rdram_iface::RDRAMInterface,
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
        }
    }

    pub fn read_u32(&self, address: u32) -> Result<u32, Error>
    {
        let mapping = memory::MemoryMapping::new(address);
        match mapping.sector
        {
            memory::Sector::SP_REG => Ok(self.rsp.read_u32_from_address(mapping.mapped_address as usize).unwrap()),
            memory::Sector::RI_REG => Ok(self.rdram_iface.read_u32_from_address(mapping.mapped_address as usize).unwrap()),
            _ => Err(Error::new(ErrorKind::Other, "Unimplemented Address.")),
        }
    }

    pub fn store_u32(&mut self, address:u32, value: u32) -> Result<(), Error>
    {
        let mapping = memory::MemoryMapping::new(address);
        match mapping.sector
        {
            memory::Sector::RI_REG => self.rdram_iface.load_u32_to_address(mapping.mapped_address as usize, value).unwrap(),
            memory::Sector::SP_REG => self.rsp.load_u32_to_address(mapping.mapped_address as usize, value).unwrap(),
            _ => return Err(Error::new(ErrorKind::Other, "Unimplemented Address.")),
        };
        Ok(())
    }
}