#[cfg(test)]
mod cpu_opcodes_tests
{
    use n64::cpu::CPU;
    use n64::connector::Connector;
    use n64::cpu_opcodes::*;

    #[test]
    fn test_mtc0() 
    {
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        let opcode = Opcode::new(0b01000000100000010000100000000000_u32);
        cpu.cpu_registers.register[0x01].set_value(0xFFFFFFFF_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cop0_registers.register[0x01].get_value() as u32, 0xFFFFFFFF_u32);
    }

    #[test]
    fn test_lui() {
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        let opcode = Opcode::new(0b00111100000000011111111111111111_u32);
        cpu.cpu_registers.register[0x01].set_value(0x0000FFFF_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cpu_registers.register[0x01].get_value() as u32, 0xFFFFFFFF_u32);
    }

    #[test]
    fn test_addiu() {
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        let opcode = Opcode::new(0b00100100001000010000000000000001_u32);
        cpu.cpu_registers.register[0x01].set_value(0x00000001_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cpu_registers.register[0x01].get_value() as u32, 0x00000002_u32);
    }


    #[test]
    fn test_lw() {
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        cpu.cpu_registers.register[0x01].set_value(0x04700000_u32);
        connector.rdram_iface.load_u32_to_address(0x00000004, 0xFFFFFFFF_u32).unwrap();
        let opcode = Opcode::new(0b10001100001000010000000000000100_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cpu_registers.register[0x01].get_value() as u32, 0xFFFFFFFF_u32);
    }

    #[test]
    fn test_bne() {
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        cpu.cpu_registers.register[0x01].set_value(0x00000001u32);
        let opcode = Opcode::new(0b00010100010000010000000000000001_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.program_counter.get_value() as u32, 0x00000004)
    }

    #[test]
    fn test_sll() {
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        cpu.cpu_registers.register[0x01].set_value(0x00000001_u32);
        let opcode = Opcode::new(0b00000000000000010000100001000000_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cpu_registers.register[0x01].get_value() as u32, 0x00000002_u32)
    }
}

