const RDRAM_CONFIG_REG_START: usize = 0x00000000;
const RDRAM_CONFIG_REG_END: usize = 0x00000003;
const RDRAM_DEVICE_ID_REG_START: usize = 0x00000004;
const RDRAM_DEVICE_ID_REG_END: usize = 0x00000007;
const RDRAM_DELAY_REG_START: usize = 0x00000008;
const RDRAM_DELAY_REG_END: usize = 0x0000000B;
const RDRAM_MODE_REG_START: usize = 0x0000000C;
const RDRAM_MODE_REG_END: usize = 0x0000000F;
const RDRAM_REF_INTERVAL_REG_START: usize = 0x00000010;
const RDRAM_REF_INTERVAL_REG_END: usize = 0x00000013;
const RDRAM_REF_ROW_REG_START: usize = 0x00000014;
const RDRAM_REF_ROW_REG_END: usize = 0x00000017;
const RDRAM_RAS_INTERVAL_REG_START: usize = 0x00000018;
const RDRAM_RAS_INTERVAL_REG_END: usize = 0x0000001B;
const RDRAM_MIN_INTERVAL_REG_START: usize = 0x0000001C;
const RDRAM_MIN_INTERVAL_REG_END: usize = 0x0000001F;
const RDRAM_ADDR_SELECT_REG_START: usize = 0x00000020;
const RDRAM_ADDR_SELECT_REG_END: usize = 0x00000023;
const RDRAM_DEVICE_MANUF_REG_START: usize = 0x00000024;
const RDRAM_DEVICE_MANUF_REG_END: usize = 0x00000027;

use n64::arch::Reg;
use std::io::{Error, ErrorKind};

pub struct RDRAMRegisters
{
    pub config: Reg,
    pub device_id: Reg,
    pub delay: Reg,
    pub mode: Reg,
    pub ref_interval: Reg,
    pub ref_row: Reg,
    pub ras_interval: Reg,
    pub min_interval: Reg,
    pub address_select: Reg,
    pub device_manufacturer: Reg,
}

impl RDRAMRegisters
{
    pub fn new() -> RDRAMRegisters
    {
        return RDRAMRegisters
        {
            config: Reg::default(),
            device_id: Reg::default(),
            delay: Reg::default(),
            mode: Reg::default(),
            ref_interval: Reg::default(),
            ref_row: Reg::default(),
            ras_interval: Reg::default(),
            min_interval: Reg::default(),
            address_select: Reg::default(),
            device_manufacturer: Reg::default()
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
            RDRAM_CONFIG_REG_START...RDRAM_CONFIG_REG_END => Ok(self.config.get_value() as u32),
            RDRAM_DEVICE_ID_REG_START...RDRAM_DEVICE_ID_REG_END => Ok(self.device_id.get_value() as u32),
            RDRAM_DELAY_REG_START...RDRAM_DELAY_REG_END => Ok(self.delay.get_value() as u32),
            RDRAM_MODE_REG_START...RDRAM_MODE_REG_END => Ok(self.mode.get_value() as u32),
            RDRAM_REF_INTERVAL_REG_START...RDRAM_REF_INTERVAL_REG_END => Ok(self.ref_interval.get_value() as u32),
            RDRAM_REF_ROW_REG_START...RDRAM_REF_ROW_REG_END => Ok(self.ref_row.get_value() as u32),
            RDRAM_RAS_INTERVAL_REG_START...RDRAM_RAS_INTERVAL_REG_END => Ok(self.ras_interval.get_value() as u32),
            RDRAM_MIN_INTERVAL_REG_START...RDRAM_MIN_INTERVAL_REG_END => Ok(self.min_interval.get_value() as u32),
            RDRAM_ADDR_SELECT_REG_START...RDRAM_ADDR_SELECT_REG_END => Ok(self.address_select.get_value() as u32),
            RDRAM_DEVICE_MANUF_REG_START...RDRAM_DEVICE_MANUF_REG_END => Ok(self.device_manufacturer.get_value() as u32),
            _ => Err(Error::new(ErrorKind::Other, "Unused rdram register address.")),
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
            RDRAM_CONFIG_REG_START...RDRAM_CONFIG_REG_END => Ok(self.config.set_value(value)),
            RDRAM_DEVICE_ID_REG_START...RDRAM_DEVICE_ID_REG_END => Ok(self.device_id.set_value(value)),
            RDRAM_DELAY_REG_START...RDRAM_DELAY_REG_END => Ok(self.delay.set_value(value)),
            RDRAM_MODE_REG_START...RDRAM_MODE_REG_END => Ok(self.mode.set_value(value)),
            RDRAM_REF_INTERVAL_REG_START...RDRAM_REF_INTERVAL_REG_END => Ok(self.ref_interval.set_value(value)),
            RDRAM_REF_ROW_REG_START...RDRAM_REF_ROW_REG_END => Ok(self.ref_row.set_value(value)),
            RDRAM_RAS_INTERVAL_REG_START...RDRAM_RAS_INTERVAL_REG_END => Ok(self.ras_interval.set_value(value)),
            RDRAM_MIN_INTERVAL_REG_START...RDRAM_MIN_INTERVAL_REG_END => Ok(self.min_interval.set_value(value)),
            RDRAM_ADDR_SELECT_REG_START...RDRAM_ADDR_SELECT_REG_END => Ok(self.address_select.set_value(value)),
            RDRAM_DEVICE_MANUF_REG_START...RDRAM_DEVICE_MANUF_REG_END => Ok(self.device_manufacturer.set_value(value)),
            0x00004004 | 0x00008004 | 0x00080004 | 0x00080008 | 0x0008000C | 0x00080014 => Ok(()),
            _ => Err(Error::new(ErrorKind::Other, "Unused rdram register address.")),
        }
    }
}