#[cfg(test)]
mod cpu_tests
{
    use n64::arch::*;

    #[test]
    fn setting_legal_cpu_reg_values() 
    {
        let mut reg_64 = Reg::new(0, true);
        let mut reg_32 = Reg::default();

        //64-bit Register Tests
        for val in (0..0xFFFFFFFFFFFFFFFF).step_by(0x10000000000000)
        {
            reg_64.set_value(val);
            assert_eq!(val, reg_64.get_value())
        }

        //32-bit Register Tests
        for val in (0..0xFFFFFFFF).step_by(0x100000)
        {
            reg_32.set_value(val);
            assert_eq!(val, reg_32.get_value())
        }
    }

    #[test]
    #[should_panic]
    fn illegal_cpu_reg_values() 
    {
        let mut reg_32 = Reg::default();
        reg_32.set_value(0x100000000_u64);
    }
}