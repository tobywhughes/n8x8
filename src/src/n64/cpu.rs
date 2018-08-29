use num::Unsigned;
use num::{NumCast, ToPrimitive, FromPrimitive};
use std::fmt;

use n64::exceptions::Exception;
use n64::arch::Reg;

pub struct CPU
{
    pub cpu_registers: CPURegisters,
    pub cop0_registers: COP0Registers,
}

impl CPU
{
    pub fn new() -> CPU
    {
        return CPU
        {
            cpu_registers: CPURegisters::new(),
            cop0_registers: COP0Registers::new(),
        }
    }
}

pub struct CPURegisters
{
    pub register: Vec<Reg>,
}

impl CPURegisters
{
    pub fn new() -> CPURegisters
    {
        return CPURegisters
        {
            register: vec![Reg::default(); 0x20],
        }
    }


    pub fn set_pif_rom_values(&mut self)
    {
        // Referenced: http://www.emulation64.com/ultra64/bootn64.html
        self.register[CPURegisterName::s4 as usize].set_value(0x00000001_u32);
        self.register[CPURegisterName::s6 as usize].set_value(0x0000003F_u32);
        self.register[CPURegisterName::sp as usize].set_value(0xA4001FF0_u32);
    }

    pub fn Debug(self)
    {
        println!("CPU Register Dump:");
        for reg in 0..0x20
        {
            print!("{}: 0x{:08x}\t", CPURegisterName::from_u8(reg).unwrap(), self.register[reg as usize].get_value());
            if (reg + 1) % 8 == 0
            {
                print!("\n");
            }
        }
    }
}

pub struct COP0Registers
{
    pub register: Vec<Reg>,
}

impl COP0Registers
{
    pub fn new() -> COP0Registers
    {
        return COP0Registers
        {
            register: vec![Reg::default(); 0x20],
        }
    }


    pub fn set_pif_rom_values(&mut self)
    {
        // Referenced: http://www.emulation64.com/ultra64/bootn64.html
        self.register[COP0RegisterName::Random as usize].set_value(0x0000001F_u32);
        self.register[COP0RegisterName::Status as usize].set_value(0x70400004_u32);
        self.register[COP0RegisterName::PRevID as usize].set_value(0x00000B00_u32);
        self.register[COP0RegisterName::Config as usize].set_value(0x0006E463_u32);
    }

    pub fn Debug(self)
    {
        println!("COP0 Register Dump:");
        for reg in 0..0x20
        {
            print!("{}: 0x{:08x}  \t", COP0RegisterName::from_u8(reg).unwrap(), self.register[reg as usize].get_value());
            if (reg + 1) % 8 == 0
            {
                print!("\n");
            }
        }
    }
}


#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(FromPrimitive)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum CPURegisterName
{
    r0 = 0x00,
    at = 0x01,
    v0 = 0x02,
    v1 = 0x03,
    a0 = 0x04,
    a1 = 0x05,
    a2 = 0x06,
    a3 = 0x07,
    t0 = 0x08,
    t1 = 0x09,
    t2 = 0x0A,
    t3 = 0x0B,
    t4 = 0x0C,
    t5 = 0x0D,
    t6 = 0x0E,
    t7 = 0x0F,
    s0 = 0x10,
    s1 = 0x11,
    s2 = 0x12,
    s3 = 0x13,
    s4 = 0x14,
    s5 = 0x15,
    s6 = 0x16,
    s7 = 0x17,
    t8 = 0x18,
    t9 = 0x19,
    k0 = 0x1A,
    k1 = 0x1B,
    gp = 0x1C,
    sp = 0x1D,
    s8 = 0x1E,
    ra = 0x1F,
}

impl CPURegisterName
{
    pub fn from_u8(value: u8) -> Option<CPURegisterName>
    {
        match value
        {
            0...0x1F => Some(FromPrimitive::from_u8(value).unwrap()),
            _ => None,
        }
    }
}

impl fmt::Display for CPURegisterName
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
} 

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(FromPrimitive)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum COP0RegisterName
{
    Index = 0x00,
    Random = 0x01,
    EntryLo0 = 0x02,
    EntryLo1 = 0x03,
    Context = 0x04,
    PageMask = 0x05,
    Wired = 0x06,
    BadVAddr = 0x08,
    Count = 0x09,
    EntryHi = 0x0A,
    Compare = 0x0B,
    Status = 0x0C,
    Cause = 0x0D,
    EPC = 0x0E,
    PRevID = 0x0F,
    Config = 0x10,
    LLAddr = 0x11,
    WatchLo = 0x12,
    WatchHi = 0x13,
    XContext = 0x14,
    PErr = 0x1A,
    CacheErr = 0x1B,
    TagLo = 0x1C,
    TagHi = 0x1D,
    ErrorEPC = 0x1E,
    RESERVED,
}

impl COP0RegisterName
{
    pub fn from_u8(value: u8) -> Option<COP0RegisterName>
    {
        match value
        {
            0...0x06 | 0x08...0x14 | 0x1A...0x1E=> Some(FromPrimitive::from_u8(value).unwrap()),
            0...0x07 | 0x15...0x19 | 0x1F=> Some(COP0RegisterName::RESERVED),
            _ => None,
        }
    }
}

impl fmt::Display for COP0RegisterName
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
} 