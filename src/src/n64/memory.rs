use std::fmt;

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

const KSEG0_START: u32 = 0x80000000;
const KSEG0_END: u32 = 0x9FFFFFFF;
const KSEG1_START: u32 = 0xA0000000;
const KSEG1_END: u32 = 0xBFFFFFFF;
const KSSEG_START: u32 = 0xC0000000;
const KSSEG_END: u32 = 0xDFFFFFFF;
const KSEG3_START: u32 = 0xE0000000; 
const KSEG3_END: u32 = 0xFFFFFFFF; 

pub struct MemoryMapping
{
    pub address: u32,
    pub mapped_address: u32,
    pub sector: Sector,
}

impl MemoryMapping
{
    pub fn new(address: u32) -> MemoryMapping
    {
        let mut sector= identify_sector(address).unwrap();
        
        if(sector == Sector::EXT_SYSAD_DEV)
        {
            match address
            {
                KSEG0_START...KSEG0_END => sector = identify_sector(address - KSEG0_START).unwrap(),
                KSEG1_START...KSEG1_END => sector = identify_sector(address - KSEG1_START).unwrap(),
                KSSEG_START...KSSEG_END => panic!("Unimplemented sysad mapping!"),
                KSEG3_START...KSEG3_END => panic!("Unimplemented sysad mapping!"),
                _ => panic!("Illegal sysad address!")
            }
        }

        return MemoryMapping
        {
            address: address,
            mapped_address: address - sector.SectorInformation().sector_start,
            sector: sector,
        }
    }

}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Sector {
    RDRAM_MEM,
    RDRAM_REG,
    SP_REG,
    DP_COMMAND_REG,
    DP_SPAN_REG,
    MI_REG,
    VI_REG,
    AI_REG,
    PI_REG,
    RI_REG,
    SI_REG,
    UNUSED,
    CD_2_ADDR_1,
    CD_1_ADDR_1,
    CD_2_ADDR_2,
    CD_1_ADDR_2,
    PIF_BOOT_ROM,
    PIF_RAM,
    RESERVED,
    CD_1_ADDR_3,
    EXT_SYSAD_DEV,
}    

impl Sector
{
    #[allow(non_snake_case)]
    pub fn SectorInformation(self) -> SectorInformation
    {
        match self
        {
            Sector::RDRAM_MEM => SectorInformation::new(RDRAM_MEM_START, RDRAM_MEM_END),
            Sector::RDRAM_REG => SectorInformation::new(RDRAM_REG_START, RDRAM_REG_END),
            Sector::SP_REG => SectorInformation::new(SP_REG_START, SP_REG_END),
            Sector::DP_COMMAND_REG => SectorInformation::new(DP_COMMAND_REG_START, DP_COMMAND_REG_END),
            Sector::DP_SPAN_REG => SectorInformation::new(DP_SPAN_REG_START, DP_SPAN_REG_END),
            Sector::MI_REG => SectorInformation::new(MI_REG_START, MI_REG_END),
            Sector::VI_REG => SectorInformation::new(VI_REG_START, VI_REG_END),
            Sector::AI_REG => SectorInformation::new(AI_REG_START, AI_REG_END),
            Sector::PI_REG => SectorInformation::new(PI_REG_START, PI_REG_END),
            Sector::RI_REG => SectorInformation::new(RI_REG_START, RI_REG_END),
            Sector::SI_REG => SectorInformation::new(SI_REG_START, SI_REG_END),
            Sector::UNUSED => SectorInformation::new(UNUSED_START, UNUSED_END),
            Sector::CD_2_ADDR_1 => SectorInformation::new(CD_2_ADDR_1_START, CD_2_ADDR_1_END),
            Sector::CD_1_ADDR_1 => SectorInformation::new(CD_1_ADDR_1_START, CD_1_ADDR_1_END),
            Sector::CD_2_ADDR_2 => SectorInformation::new(CD_2_ADDR_2_START, CD_2_ADDR_2_END),
            Sector::CD_1_ADDR_2 => SectorInformation::new(CD_1_ADDR_2_START, CD_1_ADDR_2_END),
            Sector::PIF_BOOT_ROM => SectorInformation::new(PIF_BOOT_ROM_START, PIF_BOOT_ROM_END),
            Sector::PIF_RAM => SectorInformation::new(PIF_RAM_START, PIF_RAM_END),
            Sector::RESERVED => SectorInformation::new(RESERVED_START, RESERVED_END),
            Sector::CD_1_ADDR_3 => SectorInformation::new(CD_1_ADDR_3_START, CD_1_ADDR_3_END),
            Sector::EXT_SYSAD_DEV => SectorInformation::new(EXT_SYSAD_DEV_START, EXT_SYSAD_DEV_END),
        }
    }

}

impl fmt::Display for Sector
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
} 

#[derive(Debug)]
pub struct SectorInformation
{
    pub sector_start: u32,
    pub sector_end: u32,
}

impl SectorInformation
{
    pub fn new(sector_start: u32, sector_end: u32) -> SectorInformation
    {
        return SectorInformation
        {
            sector_start: sector_start,
            sector_end: sector_end,
        }
    }
}



pub fn identify_sector(address: u32) -> Option<Sector>
{
    match address
    {
        RDRAM_MEM_START...RDRAM_MEM_END => Some(Sector::RDRAM_MEM),
        RDRAM_REG_START...RDRAM_REG_END => Some(Sector::RDRAM_REG),
        SP_REG_START...SP_REG_END => Some(Sector::SP_REG),
        DP_COMMAND_REG_START...DP_COMMAND_REG_END => Some(Sector::DP_COMMAND_REG),
        DP_SPAN_REG_START...DP_SPAN_REG_END => Some(Sector::DP_SPAN_REG),
        MI_REG_START...MI_REG_END => Some(Sector::MI_REG),
        VI_REG_START...VI_REG_END => Some(Sector::VI_REG),
        AI_REG_START...AI_REG_END => Some(Sector::AI_REG),
        PI_REG_START...PI_REG_END => Some(Sector::PI_REG),
        RI_REG_START...RI_REG_END => Some(Sector::RI_REG),
        SI_REG_START...SI_REG_END => Some(Sector::SI_REG),
        UNUSED_START...UNUSED_END => Some(Sector::UNUSED),
        CD_2_ADDR_1_START...CD_2_ADDR_1_END => Some(Sector::CD_2_ADDR_1),
        CD_1_ADDR_1_START...CD_1_ADDR_1_END => Some(Sector::CD_1_ADDR_1),
        CD_2_ADDR_2_START...CD_2_ADDR_2_END => Some(Sector::CD_2_ADDR_2),
        CD_1_ADDR_2_START...CD_1_ADDR_2_END => Some(Sector::CD_1_ADDR_2),
        PIF_BOOT_ROM_START...PIF_BOOT_ROM_END => Some(Sector::PIF_BOOT_ROM),
        PIF_RAM_START...PIF_RAM_END => Some(Sector::PIF_RAM),
        RESERVED_START...RESERVED_END => Some(Sector::RESERVED),
        CD_1_ADDR_3_START...CD_1_ADDR_3_END => Some(Sector::CD_1_ADDR_3),
        EXT_SYSAD_DEV_START...EXT_SYSAD_DEV_END => Some(Sector::EXT_SYSAD_DEV),
        _ => None,
    }
}

