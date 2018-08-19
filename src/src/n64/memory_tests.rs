#[cfg(test)]
mod memory_tests
{
    use MemoryMapping;
    use n64::memory::Sector;

    const RDRAM_MEM_START: u32 = 0x00000000;
    const RDRAM_MEM_END: u32 = 0x03EFFFFF;
    const RDRAM_REG_START: u32 = 0x03F00000;
    const RDRAM_REG_END: u32 = 0x03FFFFFF;
    const SP_REG_START: u32 = 0x04000000;
    const SP_REG_END: u32 = 0x040FFFFF;
    const DP_COMMAND_REG_START: u32 = 0x04100000;
    const DP_COMMAND_REG_END: u32 = 0x041FFFFF;
    const DP_SPAN_REG_START: u32 = 0x04200000;
    const DP_SPAN_REG_END: u32 = 0x042FFFFF;
    const MI_REG_START: u32 = 0x04300000;
    const MI_REG_END: u32 = 0x043FFFFF;
    const VI_REG_START: u32 = 0x04400000;
    const VI_REG_END: u32 = 0x044FFFFF;
    const AI_REG_START: u32 = 0x04500000;
    const AI_REG_END: u32 = 0x045FFFFF;
    const PI_REG_START: u32 = 0x04600000;
    const PI_REG_END: u32 = 0x046FFFFF;
    const RI_REG_START: u32 = 0x04700000;
    const RI_REG_END: u32 = 0x047FFFFF;
    const SI_REG_START: u32 = 0x04800000;
    const SI_REG_END: u32 = 0x048FFFFF;
    const UNUSED_START: u32 = 0x04900000;
    const UNUSED_END: u32 = 0x04FFFFFF;
    const CD_2_ADDR_1_START: u32 = 0x05000000;
    const CD_2_ADDR_1_END: u32 = 0x05FFFFFF;
    const CD_1_ADDR_1_START: u32 = 0x06000000;
    const CD_1_ADDR_1_END: u32 = 0x07FFFFFF;
    const CD_2_ADDR_2_START: u32 = 0x08000000;
    const CD_2_ADDR_2_END: u32 = 0x0FFFFFFF;
    const CD_1_ADDR_2_START: u32 = 0x10000000;
    const CD_1_ADDR_2_END: u32 = 0x1FBFFFFF;
    const PIF_BOOT_ROM_START: u32 = 0x1FC00000;
    const PIF_BOOT_ROM_END: u32 = 0x1FC007BF;
    const PIF_RAM_START: u32 = 0x1FC007C0;
    const PIF_RAM_END: u32 = 0x1FC007FF;
    const RESERVED_START: u32 = 0x1FC00800;
    const RESERVED_END: u32 = 0x1FCFFFFF; 
    const CD_1_ADDR_3_START: u32 = 0x1FD00000;
    const CD_1_ADDR_3_END: u32 = 0x7FFFFFFF;
    const EXT_SYSAD_DEV_START: u32 = 0x80000000;
    const EXT_SYSAD_DEV_END: u32 = 0xFFFFFFFF; 

    #[test]
    #[allow(overflowing_literals)]
    fn memory_mapping_gives_propper_sector()
    {
        for address in 0x00000000..0x100000000
        {
            let mapping = MemoryMapping::new(address);
            match address
            {
                RDRAM_MEM_START...RDRAM_MEM_END => assert_eq!(mapping.sector, Sector::RDRAM_MEM),
                RDRAM_REG_START...RDRAM_REG_END => assert_eq!(mapping.sector, Sector::RDRAM_REG),
                SP_REG_START...SP_REG_END => assert_eq!(mapping.sector, Sector::SP_REG),
                DP_COMMAND_REG_START...DP_COMMAND_REG_END => assert_eq!(mapping.sector, Sector::DP_COMMAND_REG),
                DP_SPAN_REG_START...DP_SPAN_REG_END => assert_eq!(mapping.sector, Sector::DP_SPAN_REG),
                MI_REG_START...MI_REG_END => assert_eq!(mapping.sector, Sector::MI_REG),
                VI_REG_START...VI_REG_END => assert_eq!(mapping.sector, Sector::VI_REG),
                AI_REG_START...AI_REG_END => assert_eq!(mapping.sector, Sector::AI_REG),
                PI_REG_START...PI_REG_END => assert_eq!(mapping.sector, Sector::PI_REG),
                RI_REG_START...RI_REG_END => assert_eq!(mapping.sector, Sector::RI_REG),
                SI_REG_START...SI_REG_END => assert_eq!(mapping.sector, Sector::SI_REG),
                UNUSED_START...UNUSED_END => assert_eq!(mapping.sector, Sector::UNUSED),
                CD_2_ADDR_1_START...CD_2_ADDR_1_END => assert_eq!(mapping.sector, Sector::CD_2_ADDR_1),
                CD_1_ADDR_1_START...CD_1_ADDR_1_END => assert_eq!(mapping.sector, Sector::CD_1_ADDR_1),
                CD_2_ADDR_2_START...CD_2_ADDR_2_END => assert_eq!(mapping.sector, Sector::CD_2_ADDR_2),
                CD_1_ADDR_2_START...CD_1_ADDR_2_END => assert_eq!(mapping.sector, Sector::CD_1_ADDR_2),
                PIF_BOOT_ROM_START...PIF_BOOT_ROM_END => assert_eq!(mapping.sector, Sector::PIF_BOOT_ROM),
                PIF_RAM_START...PIF_RAM_END => assert_eq!(mapping.sector, Sector::PIF_RAM),
                RESERVED_START...RESERVED_END => assert_eq!(mapping.sector, Sector::RESERVED),
                CD_1_ADDR_3_START...CD_1_ADDR_3_END => assert_eq!(mapping.sector, Sector::CD_1_ADDR_3),
                EXT_SYSAD_DEV_START...EXT_SYSAD_DEV_END => assert_eq!(mapping.sector, Sector::EXT_SYSAD_DEV),
                _ => (),
            }
        }
    }
}