use n64::connector::Connector;

pub struct N64
{
    pub connector: Connector,
}

impl N64 {
    pub fn new(filename: &str) -> N64
    {
        return N64
        {
            connector: Connector::new(filename),
        }
    }

    pub fn run_pif_rom(&mut self)
    {
        self.connector.cpu.cpu_registers.set_pif_rom_values();
        self.connector.cpu.cop0_registers.set_pif_rom_values();
        self.connector.mips_interface.set_pif_rom_values();
    }

    pub fn register_debug(self)
    {
        self.connector.cpu.cpu_registers.Debug();
        self.connector.cpu.cop0_registers.Debug();
    }
}