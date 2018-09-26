use n64::arch::Reg;

const RI_MODE_REG_START: usize = 0x00000000;
const RI_MODE_REG_END: usize = 0x00000003;
const RI_CONFIG_REG_START: usize = 0x00000004;
const RI_CONFIG_REG_END: usize = 0x00000007;
const RI_CURRENT_LOAD_REG_START: usize = 0x00000008;
const RI_CURRENT_LOAD_REG_END: usize = 0x0000000B;
const RI_SELECT_REG_START: usize = 0x0000000C;
const RI_SELECT_REG_END: usize = 0x0000000F;
const RI_REFRESH_REG_START: usize = 0x00000010;
const RI_REFRESH_REG_END: usize = 0x00000013;
const RI_LATENCY_REG_START: usize = 0x00000014;
const RI_LATENCY_REG_END: usize = 0x00000017;
const RI_RERROR_REG_START: usize = 0x00000018;
const RI_RERROR_REG_END: usize = 0x0000001B;
const RI_WERROR_REG_START: usize = 0x0000001C;
const RI_WERROR_REG_END: usize = 0x0000001F;

pub struct RDRAMInterface
{
    pub mode: Reg,
    pub config: Reg,
    pub current_load: Reg,
    pub select: Reg,
    pub refresh: Reg,
    pub latency: Reg,
    pub read_error: Reg,
    pub write_error: Reg,
}


impl RDRAMInterface
{
    pub fn new() -> RDRAMInterface
    {
        return RDRAMInterface
        {
            mode: Reg::default(),
            config: Reg::default(),
            current_load: Reg::default(),
            select: Reg::default(),
            refresh: Reg::default(),
            latency: Reg::default(),
            read_error: Reg::default(),
            write_error: Reg::default(),
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
            RI_MODE_REG_START...RI_MODE_REG_END => Some(self.mode.get_value() as u32),
            RI_CONFIG_REG_START...RI_CONFIG_REG_END => Some(self.config.get_value() as u32),
            RI_CURRENT_LOAD_REG_START...RI_CURRENT_LOAD_REG_END => Some(self.current_load.get_value() as u32),
            RI_SELECT_REG_START...RI_SELECT_REG_END => Some(self.select.get_value() as u32),
            RI_REFRESH_REG_START...RI_REFRESH_REG_END => Some(self.refresh.get_value() as u32),
            RI_LATENCY_REG_START...RI_LATENCY_REG_END => Some(self.latency.get_value() as u32),
            RI_RERROR_REG_START...RI_RERROR_REG_END => Some(self.read_error.get_value() as u32),
            RI_WERROR_REG_START...RI_WERROR_REG_END => Some(self.write_error.get_value() as u32),
            _ => None,
        }
    }

    pub fn load_u32_to_address(&mut self, address: usize, value: u32) -> Result<(), usize>
    {
        //Only allow alligned addresses (unaligned handled exterior to function)
        if address % 4 != 0
        {
            return Err(address);
        }

        match address
        {
            RI_MODE_REG_START...RI_MODE_REG_END => Ok(self.mode.set_value(value)),
            RI_CONFIG_REG_START...RI_CONFIG_REG_END => Ok(self.config.set_value(value)),
            RI_CURRENT_LOAD_REG_START...RI_CURRENT_LOAD_REG_END => Ok(self.current_load.set_value(value)),
            RI_SELECT_REG_START...RI_SELECT_REG_END => Ok(self.select.set_value(value)),
            RI_REFRESH_REG_START...RI_REFRESH_REG_END => Ok(self.refresh.set_value(value)),
            RI_LATENCY_REG_START...RI_LATENCY_REG_END => Ok(self.latency.set_value(value)),
            RI_RERROR_REG_START...RI_RERROR_REG_END => Ok(self.read_error.set_value(value)),
            RI_WERROR_REG_START...RI_WERROR_REG_END => Ok(self.write_error.set_value(value)),
            _ => Err(address),
        }
    }
}