use std::fmt;

#[derive(PartialEq)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum Exception {
    INTEGER_OVERFLOW,
    ADDRESS_ERROR,
    TLB_MISS(u32, u8),


    UNIMPLEMENTED_OPCODE,
    UNIMPLEMENTED_ADDRESS,
    OTHER(String)
}   

impl fmt::Display for Exception
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self
        {
            Exception::TLB_MISS(virtual_address, asid) => write!(f, "TLB Miss at 0x{:08X} and ASID 0x{:02X}", virtual_address, asid),
            Exception::OTHER(s) => write!(f, "Other Error: {}",s),
            _ => write!(f, "{:?}", self),
        }
    }
} 