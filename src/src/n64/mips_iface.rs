const MI_INIT_MODE_REG_START: usize = 0x00000000;
const MI_INIT_MODE_REG_END: usize = 0x00000003;
const MI_VERSION_REG_START: usize = 0x00000004;
const MI_VERSION_REG_END: usize = 0x00000007;
const MI_INTR_REG_START: usize = 0x00000008;
const MI_INTR_REG_END: usize = 0x0000000B;
const MI_INTR_MASK_REG_START: usize = 0x0000000C;
const MI_INTR_MASK_REG_END: usize = 0x0000000F;


use n64::arch::Reg;
use n64::exceptions::Exception;

pub struct MipsInterface
{
    pub init_mod: Reg,
    pub version: Reg,
    pub interrupt: Reg,
    pub interrupt_mask: Reg,
}

impl MipsInterface
{
    pub fn new() -> MipsInterface
    {
        return MipsInterface
        {
            init_mod: Reg::default(),
            version: Reg::default(),
            interrupt: Reg::default(),
            interrupt_mask: Reg::default(),
        }
    }

    pub fn set_pif_rom_values(&mut self)
    {
        self.version.set_value(0x01010101_u32);
    }

    pub fn read_u32_from_address(&self, address: usize) -> Result<u32, Exception>
    {
        //Only allow alligned addresses (unaligned handled exterior to function)
        if address % 4 != 0
        {
            return Err(Exception::ADDRESS_ERROR);
        }

        match address
        {
            MI_INIT_MODE_REG_START...MI_INIT_MODE_REG_END => Ok(self.init_mod.get_value() as u32),
            MI_VERSION_REG_START...MI_VERSION_REG_END => Ok(self.version.get_value() as u32),
            MI_INTR_REG_START...MI_INTR_REG_END => Ok(self.interrupt.get_value() as u32),
            MI_INTR_MASK_REG_START...MI_INTR_MASK_REG_END => Ok(self.interrupt_mask.get_value() as u32),
            _ => Err(Exception::UNIMPLEMENTED_ADDRESS),
        }
    }

    pub fn load_u32_to_address(&mut self, address: usize, value: u32) -> Result<(), Exception>
    {
        //Only allow alligned addresses (unaligned handled exterior to function)
        if address % 4 != 0
        {
            return Err(Exception::ADDRESS_ERROR);
        }

        match address
        {
            MI_INIT_MODE_REG_START...MI_INIT_MODE_REG_END => Ok(self.init_mod.set_value(value)),
            MI_VERSION_REG_START...MI_VERSION_REG_END => Ok(self.version.set_value(value)),
            MI_INTR_REG_START...MI_INTR_REG_END => Ok(self.interrupt.set_value(value)),
            MI_INTR_MASK_REG_START...MI_INTR_MASK_REG_END => Ok(self.interrupt_mask.set_value(value)),
            _ => Err(Exception::UNIMPLEMENTED_ADDRESS),
        }
    }
}