use num::Unsigned;
use num::{NumCast, ToPrimitive};

use n64::exceptions::Exception;

pub struct CPU
{
    pub cpu_registers: CPURegisters,
}

impl CPU
{
    pub fn new() -> CPU
    {
        return CPU
        {
            cpu_registers: CPURegisters::new(),
        }
    }
}

pub struct CPURegisters
{
    pub register: Vec<CPUReg>,
}

impl CPURegisters
{
    pub fn new() -> CPURegisters
    {
        return CPURegisters
        {
            register: vec![CPUReg::default(); 0x20],
        }
    }

    pub fn register_by_name(&self, name: CPURegisterName) -> &CPUReg
    {
        let register_id = name.translate_name();
        &self.register[register_id]
    }

    pub fn register_by_name_mut(&mut self, name: CPURegisterName) -> &mut CPUReg
    {
        let register_id = name.translate_name();
        &mut self.register[register_id]
    }
}

#[derive(Copy, Clone)]
pub struct CPUReg 
{
    pub value: u64,
    pub u64_mode: bool,
}

impl CPUReg
{
    pub fn default() -> CPUReg
    {
        return CPUReg
        {
            value: 0,
            u64_mode: false,
        }
    }


    pub fn new(value: u64, u64_mode: bool) -> CPUReg
    {
        return CPUReg
        {
            value: value,
            u64_mode: u64_mode,
        }
    }

    pub fn get_value(self) -> u64
    {
        match self.u64_mode
        {
            true => self.value,
            false => self.value & 0x00000000FFFFFFFF,
        }
    }

    pub fn set_value<T: Unsigned + ToPrimitive>(&mut self, value: T)
    {
        let new_value: u64 = NumCast::from(value).unwrap();
        match self.u64_mode
        {
            true => self.value = new_value,
            false =>
            {
                if new_value > 0xFFFFFFFF
                {
                    panic!("Cannot set a 32 bit register with a 64 bit value")
                }
                else {
                    self.value = new_value;
                }
            },
        }
    }
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum CPURegisterName
{
    r0,
    at,
    v0,
    v1,
    a0,
    a1,
    a2,
    a3,
    t0,
    t1,
    t2,
    t3,
    t4,
    t5,
    t6,
    t7,
    s0,
    s1,
    s2,
    s3,
    s4,
    s5,
    s6,
    s7,
    t8,
    t9,
    k0,
    k1,
    gp,
    sp,
    s8,
    ra,
}

impl CPURegisterName
{
    pub fn translate_name(self) -> usize
    {
        match self
        {
            CPURegisterName::r0 => 0x00,
            CPURegisterName::at => 0x01,
            CPURegisterName::v0 => 0x02,
            CPURegisterName::v1 => 0x03,
            CPURegisterName::a0 => 0x04,
            CPURegisterName::a1 => 0x05,
            CPURegisterName::a2 => 0x06,
            CPURegisterName::a3 => 0x07,
            CPURegisterName::t0 => 0x08,
            CPURegisterName::t1 => 0x09,
            CPURegisterName::t2 => 0x0A,
            CPURegisterName::t3 => 0x0B,
            CPURegisterName::t4 => 0x0C,
            CPURegisterName::t5 => 0x0D,
            CPURegisterName::t6 => 0x0E,
            CPURegisterName::t7 => 0x0F,
            CPURegisterName::s0 => 0x10,
            CPURegisterName::s1 => 0x11,
            CPURegisterName::s2 => 0x12,
            CPURegisterName::s3 => 0x13,
            CPURegisterName::s4 => 0x14,
            CPURegisterName::s5 => 0x15,
            CPURegisterName::s6 => 0x16,
            CPURegisterName::s7 => 0x17,
            CPURegisterName::t8 => 0x18,
            CPURegisterName::t9 => 0x19,
            CPURegisterName::k0 => 0x1A,
            CPURegisterName::k1 => 0x1B,
            CPURegisterName::gp => 0x1C,
            CPURegisterName::sp => 0x1D,
            CPURegisterName::s8 => 0x1E,
            CPURegisterName::ra => 0x1F,
        }
    }

    pub fn from_u8(value: u8) -> Option<CPURegisterName>
    {
        match value
        {
            0x00 => Some(CPURegisterName::r0),
            0x01 => Some(CPURegisterName::at),
            0x02 => Some(CPURegisterName::v0),
            0x03 => Some(CPURegisterName::v1),
            0x04 => Some(CPURegisterName::a0),
            0x05 => Some(CPURegisterName::a1),
            0x06 => Some(CPURegisterName::a2),
            0x07 => Some(CPURegisterName::a3),
            0x08 => Some(CPURegisterName::t0),
            0x09 => Some(CPURegisterName::t1),
            0x0A => Some(CPURegisterName::t2),
            0x0B => Some(CPURegisterName::t3),
            0x0C => Some(CPURegisterName::t4),
            0x0D => Some(CPURegisterName::t5),
            0x0E => Some(CPURegisterName::t6),
            0x0F => Some(CPURegisterName::t7),
            0x10 => Some(CPURegisterName::s0),
            0x11 => Some(CPURegisterName::s1),
            0x12 => Some(CPURegisterName::s2),
            0x13 => Some(CPURegisterName::s3),
            0x14 => Some(CPURegisterName::s4),
            0x15 => Some(CPURegisterName::s5),
            0x16 => Some(CPURegisterName::s6),
            0x17 => Some(CPURegisterName::s7),
            0x18 => Some(CPURegisterName::t8),
            0x19 => Some(CPURegisterName::t9),
            0x1A => Some(CPURegisterName::k0),
            0x1B => Some(CPURegisterName::k1),
            0x1C => Some(CPURegisterName::gp),
            0x1D => Some(CPURegisterName::sp),
            0x1E => Some(CPURegisterName::s8),
            0x1F => Some(CPURegisterName::ra),
            _ => None,
        }
    }
}