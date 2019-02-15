use num::Unsigned;
use num::{NumCast, ToPrimitive, FromPrimitive};
use std::fmt;


use n64::exceptions::Exception;
use n64::arch::Reg;
use n64::connector::Connector;
use n64::cpu_opcodes::Opcode;

pub struct CPU
{
    pub cpu_registers: CPURegisters,
    pub cop0_registers: COP0Registers,
    pub tlb: TLB,
    pub program_counter: Reg,
    pub lo: Reg,
    pub hi: Reg,
    pub pc_save: u32,
    pub pc_save_count: u8,
}

impl CPU
{
    pub fn new() -> CPU
    {
        return CPU
        {
            cpu_registers: CPURegisters::new(),
            cop0_registers: COP0Registers::new(),
            program_counter: Reg::default(),
            tlb: TLB::new(),
            lo: Reg::default(),
            hi: Reg::default(),
            pc_save: 0,
            pc_save_count: 0,
        }
    }

    pub fn set_pif_rom_values(&mut self)
    {
        // Referenced: http://www.emulation64.com/ultra64/bootn64.html
        self.program_counter.set_value(0xA4000040_u32);
    }

    pub fn retrieve_opcode(&mut self, connector: &Connector) -> Opcode
    {
        let pc: u32 = self.program_counter.get_value() as u32;
        let value: u32 = connector.read_u32(pc).unwrap();
        self.program_counter.set_value(pc + 4);
        Opcode::new(value)
    }

    pub fn execute_opcode(&mut self, opcode: &Opcode, connector: &mut Connector) -> Result<(), Exception>
    {
        opcode.execute(self, connector)?;
        if self.pc_save_count > 0
        {
            if self.pc_save_count == 1
            {
                self.program_counter.set_value(self.pc_save);
            }

            self.pc_save_count -= 1;
        }
        Ok(())
    }

    pub fn compute_physical_address(&mut self, virtual_address: u32) -> Result<u32, Exception>
    {
        let asid: u8 = ((self.cop0_registers.register[COP0RegisterName::EntryHi as usize].get_value() as u32) & 0x000000FF) as u8;
        for tlb_index in 0..0x20
        {
            let mask_offset = ((self.tlb.entries[tlb_index].mask as f32) + 1.0).log2() as u8;
            let vaddr_vpn: u32 = virtual_address >> (12 + mask_offset);
            if (vaddr_vpn >> 1) == (self.tlb.entries[tlb_index].virtual_page_number >> mask_offset)
            {
                if asid == self.tlb.entries[tlb_index].address_space_id || self.tlb.entries[tlb_index].global
                {
                    if vaddr_vpn % 2 == 0
                    {
                        if self.tlb.entries[tlb_index].valid_even && self.tlb.entries[tlb_index].dirty_even
                        {
                            return Ok(((self.tlb.entries[tlb_index].physical_frame_num_even & (!((1 << (mask_offset as u32)) -1))) << 12) | (virtual_address & ((1 << (12 + mask_offset as u32)) -1)))
                        }
                    }
                    else
                    {
                        if self.tlb.entries[tlb_index].valid_odd && self.tlb.entries[tlb_index].dirty_odd
                        {
                            return Ok(((self.tlb.entries[tlb_index].physical_frame_num_odd & (!((1 << (mask_offset as u32)) -1))) << 12) | (virtual_address & ((1 << (12 + mask_offset as u32)) -1)))

                        }
                    }
                }
            }
        }
        Err(Exception::TLB_MISS(virtual_address, asid))
    }
}

pub struct CPURegisters
{
    pub register: Vec<Reg>,
}

impl CPURegisters
{
    pub fn new() -> CPURegisters
    {
        return CPURegisters
        {
            register: vec![Reg::default(); 0x20],
        }
    }


    pub fn set_pif_rom_values(&mut self)
    {
        // Referenced: http://www.emulation64.com/ultra64/bootn64.html
        self.register[CPURegisterName::s4 as usize].set_value(0x00000001_u32);
        self.register[CPURegisterName::s6 as usize].set_value(0x0000003F_u32);
        self.register[CPURegisterName::sp as usize].set_value(0xA4001FF0_u32);
    }



    pub fn Debug(&self)
    {
        println!("CPU Register Dump:");
        for reg in 0..0x20
        {
            print!("{}: 0x{:08x}\t", CPURegisterName::from_u8(reg).unwrap(), self.register[reg as usize].get_value());
            if (reg + 1) % 8 == 0
            {
                print!("\n");
            }
        }
    }
}

pub struct TLB
{
    pub entries: Vec<TLBEntry>,
}

impl TLB
{
    pub fn new() -> TLB
    {
        let mut TLBEntryVector: Vec<TLBEntry> = Vec::new();
        for i in 0..0x20
        {
            TLBEntryVector.push(TLBEntry::new());
        }
        return TLB
        {
            entries: TLBEntryVector,
        }
    }
}

pub struct TLBEntry
{
    pub data: Vec<u32>,
    pub mask: u16,
    pub virtual_page_number: u32,
    pub global: bool,
    pub address_space_id: u8,
    pub physical_frame_num_even: u32,
    pub physical_frame_num_odd: u32,
    pub cache_algorithm_even: u8,
    pub cache_algorithm_odd: u8,
    pub dirty_even: bool,
    pub dirty_odd: bool,
    pub valid_even: bool,
    pub valid_odd: bool,
}

impl TLBEntry
{
    pub fn new() -> TLBEntry
    {
        return TLBEntry
        {
            data: vec![0_u32;4],
            mask: 0,
            virtual_page_number: 0,
            global: false,
            address_space_id: 0,
            physical_frame_num_even: 0,
            physical_frame_num_odd: 0,
            cache_algorithm_even: 0,
            cache_algorithm_odd: 0,
            dirty_even: false,
            dirty_odd: false,
            valid_even: false,
            valid_odd: false,
        }
    }

    pub fn fill_entry_from_cop0_regs(&mut self, cop0_registers: &COP0Registers)
    {
        let page_mask_ = cop0_registers.register[COP0RegisterName::PageMask as usize].get_value() as u32;
        let entry_hi_ = cop0_registers.register[COP0RegisterName::EntryHi as usize].get_value() as u32;
        let entry_lo0_ = cop0_registers.register[COP0RegisterName::EntryLo0 as usize].get_value() as u32;
        let entry_lo1_ = cop0_registers.register[COP0RegisterName::EntryLo1 as usize].get_value() as u32;

        //Page Mask
        self.data[0] =  page_mask_ & 0x01FFE000;
        self.mask = ((page_mask_ >> 13) & 0x00000FFF) as u16;

        //VPN divided by 2 and asid
        self.data[1] =  entry_hi_ & 0xFFFFE0FF;
        self.virtual_page_number = (entry_hi_ >> 13) & 0x0007FFFF;
        self.address_space_id =  (entry_hi_ & 0x000000FF) as u8;

        //global
        let global_ = (entry_lo0_ & 0x00000001) & (entry_lo1_ & 0x00000001);
        self.global = if global_ != 0 {true} else {false};
        self.data[1] |= global_ << 12;         

        //Entry los to pfns, etc.
        self.data[2] = entry_lo0_ & 0x03FFFFFE;
        self.data[3] = entry_lo1_ & 0x03FFFFFE;
        self.physical_frame_num_even =  (entry_lo0_ >> 6) & 0x000FFFFF;
        self.physical_frame_num_odd =  (entry_lo1_ >> 6) & 0x000FFFFF;
        self.cache_algorithm_even = ((entry_lo0_ >> 3) & 0x00000007) as u8;
        self.cache_algorithm_odd = ((entry_lo1_ >> 3) & 0x00000007) as u8;
        self.dirty_even = if ((entry_lo0_  >> 2) & 0x00000001) == 1 {true} else {false};
        self.dirty_odd = if ((entry_lo1_  >> 2) & 0x00000001) == 1 {true} else {false};
        self.valid_even = if ((entry_lo0_  >> 1) & 0x00000001) == 1 {true} else {false};
        self.valid_odd = if ((entry_lo1_  >> 1) & 0x00000001) == 1 {true} else {false};
    }

    pub fn Debug(&self)
    {
        println!("{:08X} {:08X} {:08X} {:08X}", self.data[0], self.data[1], self.data[2], self.data[3]);
        println!("Mask: {:032b}", self.mask);
        println!("VPN: {:032b}", self.virtual_page_number);
        println!("ASID: {:032b}",  self.address_space_id);
        println!("PFN even/odd {:032b} {:032b}", self.physical_frame_num_even, self.physical_frame_num_odd);
        println!("Cache Algorithm even/odd {:032b} {:032b}", self.cache_algorithm_even, self.cache_algorithm_odd);
        println!("Global {}", self.global);
        println!("Dirty even/odd {} {}", self.dirty_even, self.dirty_odd);
        println!("valideven/odd {} {}", self.valid_even, self.valid_odd);
    }
}

pub struct COP0Registers
{
    pub register: Vec<Reg>,
}

impl COP0Registers
{
    pub fn new() -> COP0Registers
    {
        return COP0Registers
        {
            register: vec![Reg::default(); 0x20],
        }
    }


    pub fn set_pif_rom_values(&mut self)
    {
        // Referenced: http://www.emulation64.com/ultra64/bootn64.html
        self.register[COP0RegisterName::Random as usize].set_value(0x0000001F_u32);
        self.register[COP0RegisterName::Status as usize].set_value(0x70400004_u32);
        self.register[COP0RegisterName::PRevID as usize].set_value(0x00000B00_u32);
        self.register[COP0RegisterName::Config as usize].set_value(0x0006E463_u32);
    }



    pub fn Debug(&self)
    {
        println!("COP0 Register Dump:");
        for reg in 0..0x20
        {
            print!("{}: 0x{:08x}  \t", COP0RegisterName::from_u8(reg).unwrap(), self.register[reg as usize].get_value());
            if (reg + 1) % 8 == 0
            {
                print!("\n");
            }
        }
    }
}


#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(FromPrimitive)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum CPURegisterName
{
    r0 = 0x00,
    at = 0x01,
    v0 = 0x02,
    v1 = 0x03,
    a0 = 0x04,
    a1 = 0x05,
    a2 = 0x06,
    a3 = 0x07,
    t0 = 0x08,
    t1 = 0x09,
    t2 = 0x0A,
    t3 = 0x0B,
    t4 = 0x0C,
    t5 = 0x0D,
    t6 = 0x0E,
    t7 = 0x0F,
    s0 = 0x10,
    s1 = 0x11,
    s2 = 0x12,
    s3 = 0x13,
    s4 = 0x14,
    s5 = 0x15,
    s6 = 0x16,
    s7 = 0x17,
    t8 = 0x18,
    t9 = 0x19,
    k0 = 0x1A,
    k1 = 0x1B,
    gp = 0x1C,
    sp = 0x1D,
    s8 = 0x1E,
    ra = 0x1F,
}

impl CPURegisterName
{
    pub fn from_u8(value: u8) -> Option<CPURegisterName>
    {
        match value
        {
            0...0x1F => Some(FromPrimitive::from_u8(value).unwrap()),
            _ => None,
        }
    }
}

impl fmt::Display for CPURegisterName
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
} 

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(FromPrimitive)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum COP0RegisterName
{
    Index = 0x00,
    Random = 0x01,
    EntryLo0 = 0x02,
    EntryLo1 = 0x03,
    Context = 0x04,
    PageMask = 0x05,
    Wired = 0x06,
    BadVAddr = 0x08,
    Count = 0x09,
    EntryHi = 0x0A,
    Compare = 0x0B,
    Status = 0x0C,
    Cause = 0x0D,
    EPC = 0x0E,
    PRevID = 0x0F,
    Config = 0x10,
    LLAddr = 0x11,
    WatchLo = 0x12,
    WatchHi = 0x13,
    XContext = 0x14,
    PErr = 0x1A,
    CacheErr = 0x1B,
    TagLo = 0x1C,
    TagHi = 0x1D,
    ErrorEPC = 0x1E,
    RESERVED,
}

impl COP0RegisterName
{
    pub fn from_u8(value: u8) -> Option<COP0RegisterName>
    {
        match value
        {
            0...0x06 | 0x08...0x14 | 0x1A...0x1E=> Some(FromPrimitive::from_u8(value).unwrap()),
            0...0x07 | 0x15...0x19 | 0x1F=> Some(COP0RegisterName::RESERVED),
            _ => None,
        }
    }
}

impl fmt::Display for COP0RegisterName
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
} 