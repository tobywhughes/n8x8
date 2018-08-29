use n64::arch::Reg;

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
}