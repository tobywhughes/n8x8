use n64::{cpu, rom, mips_iface, memory,rsp, rdram_iface};

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

    pub fn read_u32(&self, address: u32) -> u32
    {
        let mapping = memory::MemoryMapping::new(address);
        match mapping.sector
        {
            memory::Sector::SP_REG => self.rsp.read_u32_from_address(mapping.mapped_address as usize).unwrap(),
            memory::Sector::RI_REG => self.rdram_iface.read_u32_from_address(mapping.mapped_address as usize).unwrap(),
            _ => panic!("Unimplemented Address"),
        }
    }
}