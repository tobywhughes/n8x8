#[cfg(test)]
mod cpu_tests
{
    use n64::cpu::*;

    #[test]
    fn can_access_and_modify_all_cpu_regs()
    {
        let mut cpu_regs = CPURegisters::new();
        for reg in 0..0x20
        {
            cpu_regs.register[reg].set_value(1_u32);
            assert_eq!(1, cpu_regs.register[reg].get_value());
        }
    }

    #[test]
    fn can_access_and_modify_cpu_regs_with_name() 
    {
        let mut cpu_regs = CPURegisters::new();
        let mut reg_names: Vec<CPURegisterName> = Vec::new();
        for reg_name_id in 0..0x20
        {
            reg_names.push(CPURegisterName::from_u8(reg_name_id).unwrap());
        }

        for reg in 0..0x20
        {
            cpu_regs.register[reg_names[reg] as usize].set_value(1_u32);
            assert_eq!(1, cpu_regs.register[reg].get_value());
            assert_eq!(1, cpu_regs.register[reg_names[reg] as usize].get_value());
        }
    }

    #[test]
    fn can_access_and_modify_all_cop0_regs()
    {
        let mut cop0_regs = COP0Registers::new();
        for reg in 0..0x20
        {
            cop0_regs.register[reg].set_value(1_u32);
            assert_eq!(1, cop0_regs.register[reg].get_value());
        }
    }

    #[test]
    fn can_access_and_modify_cop0_regs_with_name() 
    {
        let mut cop0_regs = COP0Registers::new();
        let mut reg_names: Vec<COP0RegisterName> = Vec::new();
        for reg_name_id in 0..0x20
        {
            reg_names.push(COP0RegisterName::from_u8(reg_name_id).unwrap());
        }

        for reg in 0..0x20
        {
            if(reg_names[reg] != COP0RegisterName::RESERVED)
            {
                cop0_regs.register[reg_names[reg] as usize].set_value(1_u32);
                assert_eq!(1, cop0_regs.register[reg].get_value());
                assert_eq!(1, cop0_regs.register[reg_names[reg] as usize].get_value());
            }
        }
    }

    #[test]
    fn test_tlb_entry_fill() 
    {
        let mut cpu = CPU::new();
        cpu.cop0_registers.register[COP0RegisterName::PageMask as usize].set_value(0xFFFFFFFF_u32);
        cpu.cop0_registers.register[COP0RegisterName::EntryHi as usize].set_value(0xFFFFFFFF_u32);
        cpu.cop0_registers.register[COP0RegisterName::EntryLo0 as usize].set_value(0xFFFFFFFF_u32);
        cpu.cop0_registers.register[COP0RegisterName::EntryLo1 as usize].set_value(0xFFFFFFFF_u32);
        cpu.tlb.entries[0].fill_entry_from_cop0_regs(&cpu.cop0_registers);
        //data 0
        assert_eq!(cpu.tlb.entries[0].data[0], 0b00000001111111111110000000000000_u32);
        assert_eq!(cpu.tlb.entries[0].mask, 0x0FFF_u16);

        //data 1
        assert_eq!(cpu.tlb.entries[0].data[1], 0b11111111111111111111000011111111_u32);
        assert_eq!(cpu.tlb.entries[0].virtual_page_number, 0x0007FFFF_u32);
        assert_eq!(cpu.tlb.entries[0].global, true);
        assert_eq!(cpu.tlb.entries[0].address_space_id, 0xFF_u8);

        //data 2
        assert_eq!(cpu.tlb.entries[0].data[2], 0b00000011111111111111111111111110_u32);
        assert_eq!(cpu.tlb.entries[0].physical_frame_num_even, 0x000FFFFF_u32);
        assert_eq!(cpu.tlb.entries[0].dirty_even, true);
        assert_eq!(cpu.tlb.entries[0].valid_even, true);

        //data 3
        assert_eq!(cpu.tlb.entries[0].data[3], 0b00000011111111111111111111111110_u32);
        assert_eq!(cpu.tlb.entries[0].physical_frame_num_odd, 0x000FFFFF_u32);
        assert_eq!(cpu.tlb.entries[0].dirty_odd, true);
        assert_eq!(cpu.tlb.entries[0].valid_odd, true);
    }

    #[test]
    fn test_virtual_to_physical_conversion() 
    {
        let mut cpu = CPU::new();
        //No valid bits set
        assert!(cpu.compute_physical_address(0x00000000_u32).is_err());
        //No Match
        assert!(cpu.compute_physical_address(0xFFFFFFFF_u32).is_err());

        cpu.cop0_registers.register[COP0RegisterName::PageMask as usize].set_value(0xFFFFFFFF_u32);
        cpu.cop0_registers.register[COP0RegisterName::EntryHi as usize].set_value(0xFFFFFFFF_u32);
        cpu.cop0_registers.register[COP0RegisterName::EntryLo0 as usize].set_value(0xFFFFFFFF_u32);
        cpu.cop0_registers.register[COP0RegisterName::EntryLo1 as usize].set_value(0xFFFFFFFF_u32);
        cpu.tlb.entries[0x1F].fill_entry_from_cop0_regs(&cpu.cop0_registers);
        assert!(cpu.compute_physical_address(0xFFFFFFFF_u32).is_ok());
        assert_eq!(cpu.compute_physical_address(0xFFFFFFFF_u32).unwrap(), 0xFFFFFFFF_u32);
    }
}