#[cfg(test)]
mod cpu_tests
{
    use n64::cpu::*;

    #[test]
    fn setting_legal_cpu_reg_values() 
    {
        let mut cpu_reg_64 = CPUReg::new(0, true);
        let mut cpu_reg_32 = CPUReg::default();

        //64-bit Register Tests
        for val in (0..0xFFFFFFFFFFFFFFFF).step_by(0x10000000000000)
        {
            cpu_reg_64.set_value(val);
            assert_eq!(val, cpu_reg_64.get_value())
        }

        //32-bit Register Tests
        for val in (0..0xFFFFFFFF).step_by(0x100000)
        {
            cpu_reg_32.set_value(val);
            assert_eq!(val, cpu_reg_32.get_value())
        }
    }

    #[test]
    #[should_panic]
    fn illegal_cpu_reg_values() 
    {
        let mut cpu_reg_32 = CPUReg::default();
        cpu_reg_32.set_value(0x100000000_u64);
    }

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
}