use n64::{cpu, rom, mips_iface, memory,rsp};

pub struct Connector
{
    pub cpu: cpu::CPU,
    pub rom: rom::Rom,
    pub mips_interface: mips_iface::MipsInterface,
    pub rsp: rsp::RealitySignalProcessor,
}

impl Connector
{
    pub fn new(filename: &str) -> Connector
    {
        return Connector
        {
            cpu: cpu::CPU::new(),
            rom: rom::Rom::new(filename),
            mips_interface: mips_iface::MipsInterface::new(),
            rsp: rsp::RealitySignalProcessor::new(),
        }
    }
}