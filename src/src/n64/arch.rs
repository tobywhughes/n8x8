use num::Unsigned;
use num::{NumCast, ToPrimitive};

#[derive(Copy, Clone)]
pub struct Reg 
{
    pub value: u64,
    pub u64_mode: bool,
}

impl Reg
{
    pub fn default() -> Reg
    {
        return Reg
        {
            value: 0,
            u64_mode: false,
        }
    }


    pub fn new(value: u64, u64_mode: bool) -> Reg
    {
        return Reg
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