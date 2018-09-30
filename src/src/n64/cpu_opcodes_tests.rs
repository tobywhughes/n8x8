#[cfg(test)]
mod cpu_opcodes_tests
{
    use n64::cpu::{CPU, CPURegisterName};
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
    fn test_lui() 
    {
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        let opcode = Opcode::new(0b00111100000000011111111111111111_u32);
        cpu.cpu_registers.register[0x01].set_value(0x0000FFFF_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cpu_registers.register[0x01].get_value() as u32, 0xFFFF0000_u32);
    }

    #[test]
    fn test_addiu() 
    {
        //Regular
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        let opcode = Opcode::new(0b00100100001000010000000000000001_u32);
        cpu.cpu_registers.register[0x01].set_value(0x00000001_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cpu_registers.register[0x01].get_value() as u32, 0x00000002_u32);

        //Negative :/
        let opcode = Opcode::new(0b00100100001000011111111111111111_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cpu_registers.register[0x01].get_value() as u32, 0x00000001_u32);
    }


    #[test]
    fn test_lw() 
    {
        //Regular
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        cpu.cpu_registers.register[0x01].set_value(0x04700000_u32);
        connector.rdram_iface.load_u32_to_address(0x00000004, 0xFFFFFFFF_u32).unwrap();
        let opcode = Opcode::new(0b10001100001000010000000000000100_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cpu_registers.register[0x01].get_value() as u32, 0xFFFFFFFF_u32);

        //Negative Quirk
        cpu.cpu_registers.register[0x01].set_value(0x04700005_u32);
        connector.rdram_iface.load_u32_to_address(0x00000004, 0xFFFFFFFF_u32).unwrap();
        let opcode = Opcode::new(0b10001100001000011111111111111111_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cpu_registers.register[0x01].get_value() as u32, 0xFFFFFFFF_u32);
    }

    #[test]
    fn test_bne() 
    {
        //Regular
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        cpu.cpu_registers.register[0x01].set_value(0x00000001u32);
        let opcode = Opcode::new(0b00010100010000010000000000000001_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.program_counter.get_value() as u32, 0x00000004);

        //Negative
        let opcode = Opcode::new(0b00010100010000011111111111111111_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.program_counter.get_value() as u32, 0x00000000);
    }

    #[test]
    fn test_sll() 
    {
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        cpu.cpu_registers.register[0x01].set_value(0x00000001_u32);
        let opcode = Opcode::new(0b00000000000000010000100001000000_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cpu_registers.register[0x01].get_value() as u32, 0x00000002_u32)
    }

    #[test]
    fn test_sw() 
    {

        //Regular
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        cpu.cpu_registers.register[0x01].set_value(0x04001FFB_u32);
        cpu.cpu_registers.register[0x02].set_value(0xFFFFFFFF_u32);
        let opcode = Opcode::new(0b10101100001000100000000000000001_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(connector.read_u32(0x04001FFC_u32).unwrap(), 0xFFFFFFFF_u32);


        //Negative
        cpu.cpu_registers.register[0x01].set_value(0x04001FF9_u32);
        let opcode = Opcode::new(0b10101100001000101111111111111111_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(connector.read_u32(0x04001FF8_u32).unwrap(), 0xFFFFFFFF_u32);

    }

    #[test]
    fn test_ori() 
    {
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        cpu.cpu_registers.register[0x01].set_value(0xFFFF0000_u32);
        let opcode = Opcode::new(0b00110100001000011111111111111111_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cpu_registers.register[0x01].get_value() as u32, 0xFFFFFFFF_u32);
    }


    #[test]
    fn test_addi() 
    {
        //Positive
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        cpu.cpu_registers.register[0x01].set_value(0x00000000_u32);
        let opcode = Opcode::new(0b00100000001000010000000000000001_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cpu_registers.register[0x01].get_value() as u32, 0x00000001_u32);
        
        //Negative
        let opcode = Opcode::new(0b00100000001000011111111111111111_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cpu_registers.register[0x01].get_value() as u32, 0x00000000_u32);
   }

    #[test]
    fn test_addi_trap() 
    {
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        cpu.cpu_registers.register[0x01].set_value(0xFFFFFFFF_u32);
        let opcode = Opcode::new(0b00100000001000010111111111111111_u32);
        assert!(opcode.execute(&mut cpu, &mut connector).is_err());
    }

    #[test]
    fn test_or() 
    {
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        cpu.cpu_registers.register[0x01].set_value(0xFFFF0000_u32);
        cpu.cpu_registers.register[0x02].set_value(0x0000FFFF_u32);
        let opcode = Opcode::new(0b00000000001000100000100000100101_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cpu_registers.register[0x01].get_value() as u32, 0xFFFFFFFF_u32);
    }

    #[test]
    fn test_beq() 
    {
        //Regular
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        cpu.cpu_registers.register[0x01].set_value(0x00000001u32);
        let opcode = Opcode::new(0b00010000001000010000000000000001_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.program_counter.get_value() as u32, 0x00000004);

        //Negative
        let opcode = Opcode::new(0b00010000001000011111111111111111_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.program_counter.get_value() as u32, 0x00000000);
    }

    #[test]
    fn test_jal() 
    {
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        let opcode = Opcode::new(0b00001111111111111111111111111111_u32);
        cpu.program_counter.set_value(0x00000004_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.program_counter.get_value() as u32, 0x0FFFFFFC_u32);
        assert_eq!(cpu.cpu_registers.register[CPURegisterName::ra as usize].get_value() as u32, 0x00000008);
    }

    #[test]
    fn test_slti() 
    {
        //True
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        let opcode = Opcode::new(0b00101000001000011111111111111111_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cpu_registers.register[0x01].get_value() as u32, 0x00000001);
        
        //False
        let opcode = Opcode::new(0b00101000001000010000000000000000_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cpu_registers.register[0x01].get_value() as u32, 0x00000000);
    }

    #[test]
    fn test_beql() 
    {
        //Branch
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        let opcode = Opcode::new(0b01010000001000010000000000000100_u32);
        cpu.execute_opcode(&opcode, &mut connector);
        assert_eq!(cpu.program_counter.get_value() as u32, 0x00000000_u32);
        let opcode = Opcode::new(0b00000000000000000000000000000000_u32);
        cpu.execute_opcode(&opcode, &mut connector);
        assert_eq!(cpu.program_counter.get_value() as u32, 0x00000010_u32);

        //No Branch
        cpu.cpu_registers.register[0x02].set_value(1_u8);
        let opcode = Opcode::new(0b01010000001000100000000000000100_u32);
        cpu.execute_opcode(&opcode, &mut connector);
        assert_eq!(cpu.program_counter.get_value() as u32, 0x00000014_u32);
    }


    #[test]
    fn test_andi() 
    {
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        cpu.cpu_registers.register[0x01].set_value(0xFFFFFFFF_u32);
        let opcode = Opcode::new(0b00110000001000011111111111111111_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cpu_registers.register[0x01].get_value() as u32, 0x0000FFFF_u32);
    }


    #[test]
    fn test_xori() 
    {
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        cpu.cpu_registers.register[0x01].set_value(0xFFFFFFFF_u32);
        let opcode = Opcode::new(0b00111000001000011111111111111111_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cpu_registers.register[0x01].get_value() as u32, 0xFFFF0000_u32);
    }

    #[test]
    fn test_jr() {
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        cpu.cpu_registers.register[0x01].set_value(0xFFFFFFFF_u32);
        let opcode = Opcode::new(0b00000000001000000000000000001000_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.program_counter.get_value() as u32, 0xFFFFFFFF_u32);
    }

    #[test]
    fn test_srl() {
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        cpu.cpu_registers.register[0x01].set_value(0x0000010_u32);
        let opcode = Opcode::new(0b00000000000000010000100100000010_u32);
        opcode.execute(&mut cpu, &mut connector);
        assert_eq!(cpu.cpu_registers.register[0x01].get_value() as u32, 0x00000001_u32);
    }

    #[test]
    fn test_bnel() {
        //Branch
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        cpu.cpu_registers.register[0x02].set_value(1_u8);
        let opcode = Opcode::new(0b01010100001000100000000000000100_u32);
        cpu.execute_opcode(&opcode, &mut connector);
        assert_eq!(cpu.program_counter.get_value() as u32, 0x00000000_u32);
        let opcode = Opcode::new(0b00000000000000000000000000000000_u32);
        cpu.execute_opcode(&opcode, &mut connector);
        assert_eq!(cpu.program_counter.get_value() as u32, 0x00000010_u32);

        //No Branch
        cpu.cpu_registers.register[0x01].set_value(1_u8);
        let opcode = Opcode::new(0b01010100001000100000000000000100_u32);
        cpu.execute_opcode(&opcode, &mut connector);
        assert_eq!(cpu.program_counter.get_value() as u32, 0x00000014_u32);
    }

    #[test]
    fn test_blezl() {
        //Branch
        let mut cpu = CPU::new();
        let mut connector = Connector::test();
        cpu.cpu_registers.register[0x01].set_value(0xFFFFFFFF_u32);
        let opcode = Opcode::new(0b01011000001000000000000000000100_u32);
        cpu.execute_opcode(&opcode, &mut connector);
        assert_eq!(cpu.program_counter.get_value() as u32, 0x00000000_u32);
        let opcode = Opcode::new(0b00000000000000000000000000000000_u32);
        cpu.execute_opcode(&opcode, &mut connector);
        assert_eq!(cpu.program_counter.get_value() as u32, 0x00000010_u32);

        //No Branch
        cpu.cpu_registers.register[0x01].set_value(1_u8);
        let opcode = Opcode::new(0b01011000001000000000000000000100_u32);
        cpu.execute_opcode(&opcode, &mut connector);
        assert_eq!(cpu.program_counter.get_value() as u32, 0x00000014_u32);
    }
}

