use n64::connector::Connector;
use n64::cpu_opcodes::Opcode;
use n64::cpu;


pub struct N64
{
    pub connector: Connector,
    pub cpu: cpu::CPU,
}

impl N64 {
    pub fn new(filename: &str) -> N64
    {
        return N64
        {
            connector: Connector::new(filename),
            cpu: cpu::CPU::new(),
        }
    }

    pub fn run_pif_rom(&mut self)
    {
        //Init CPU
        self.cpu.cpu_registers.set_pif_rom_values();
        self.cpu.cop0_registers.set_pif_rom_values();
        self.cpu.set_pif_rom_values();

        //Init MIPS Interface
        self.connector.mips_interface.set_pif_rom_values();

        //Copy ROM data
        let rom_data: Vec<u8> = self.connector.rom.rom_data[0..0x1000].to_vec();
        self.connector.rsp.copy_bytes_from_u8_vector(0x0000, rom_data, 0x1000);
    }

    pub fn register_debug(&self)
    {
        self.cpu.cpu_registers.Debug();
        self.cpu.cop0_registers.Debug();
    }

    pub fn run(&mut self)
    {
        let opcode = self.cpu.retrieve_opcode(&self.connector);
        opcode.Debug();
    }
}