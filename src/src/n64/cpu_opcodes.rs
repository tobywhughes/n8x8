use num::{NumCast, ToPrimitive, FromPrimitive};
use std::io::{Error, ErrorKind};
use n64::cpu::{CPU, CPURegisterName};
use n64::connector::Connector;
use std::fmt;
use binary_helpers::*;

pub struct Opcode
{
    pub opcode: u32,
    pub nuemonic: String,
    pub command: Command,
    pub rs: u8,
    pub rt: u8,
    pub rd: u8,
    pub sa: u8,
    pub fs: u8,
    pub ft: u8,
    pub fd: u8,
    pub base: u8,
    pub imm: u16,
    pub offset: u16,
    pub target: u32,
}

impl Opcode
{
    pub fn new(opcode: u32) -> Opcode
    {
        return Opcode
        {
            opcode: opcode,
            nuemonic: "".to_string(),
            command: Command::from_opcode(opcode),
            rs: ((opcode >> 21) & 0x1F) as u8,
            rt: ((opcode >> 16) & 0x1F) as u8,
            rd: ((opcode >> 11) & 0x1F) as u8,
            sa: ((opcode >> 6) & 0x1F) as u8,
            fs: ((opcode >> 11) & 0x1F) as u8,
            ft: ((opcode >> 16) & 0x1F) as u8,
            fd: ((opcode >> 6) & 0x1F) as u8,
            base: ((opcode >> 21) & 0x1F) as u8,
            imm: (opcode & 0x0000FFFF) as u16,
            offset: (opcode & 0x0000FFFF) as u16,
            target: opcode & 0x03FFFFFF,
        }
    }

    pub fn Debug(&self)
    {
        println!("OPCODE DEBUG - 0x{:08x}", self.opcode);
        println!("{:04b} {:04b} {:04b} {:04b} {:04b} {:04b} {:04b} {:04b}", (self.opcode >> 28) & 0xF, (self.opcode >> 24) & 0xF, (self.opcode >> 20) & 0xF, (self.opcode >> 16) & 0xF, (self.opcode >> 12) & 0xF, (self.opcode >> 8) & 0xF, (self.opcode >> 4) & 0xF, self.opcode & 0xF);
        println!("COMMAND - {}\t{}", self.command, self.nuemonic);
        println!("rs: 0x{:02x}\trt: 0x{:02x}\trd: 0x{:02x}\tsa: 0x{:02x}", self.rs, self.rt, self.rd, self.sa);
        println!("fs: 0x{:02x}\tft: 0x{:02x}\tfd: 0x{:02x}\tbase: 0x{:02x}", self.fs, self.ft, self.fd, self.base);
        println!("imm: 0x{:04x}\toffset: 0x{:04x}\ttarget: 0x{:08x}", self.imm, self.offset, self.target);
    }

    pub fn execute(&self, cpu: &mut CPU, connector: &mut Connector) -> Result<(), Error>
    {
        self.command.parse(self, cpu, connector)?;
        Ok(())
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(FromPrimitive)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
pub enum Command
{
    LB,
    LBU,
    LD,
    LDL,
    LDR,
    LH,
    LHU,
    LL,
    LLD,
    LW,
    LWL,
    LWR,
    LWU,
    SB,
    SC,
    SCD,
    SD,
    SDL,
    SDR,
    SH,
    SW,
    SWL,
    SWR,
    SYNC,
    ADD,
    ADDI,
    ADDIU,
    ADDU,
    AND,
    ANDI,
    DADD,
    DADDI,
    DADDIU,
    DADDU,
    DDIV,
    DDIVU,
    DIV,
    DIVU,
    DMULT,
    DMULTU,
    DSLL,
    DSLL32,
    DSLLV,
    DSRA,
    DSRA32,
    DSRAV,
    DSRL,
    DSRL32,
    DSRLV,
    DSUB ,
    DSUBU,
    LUI ,
    MFHI,
    MFLO,
    MTHI,
    MTLO,
    MULT,
    MULTU,
    NOR,
    OR,
    ORI,
    SLL,
    SLLV,
    SLT,
    SLTI,
    SLTIU,
    SLTU,
    SRA,
    SRAV,
    SRL,
    SRLV,     
    SUB,     
    SUBU,      
    XOR,     
    XORI, 
    BEQ,
    BEQL,
    BGEZ,
    BGEZAL,
    BGEZALL,
    BGEZL,
    BGTZ,
    BGTZL,
    BLEZ,
    BLEZL,
    BLTZ,
    BLTZAL,
    BLTZALL,
    BLTZL,
    BNE,
    BNEL,
    J,
    JAL,
    JALR,
    JR,
    BREAK,
    SYSCALL,
    TEQ,
    TEQI,
    TGE,
    TGEI,
    TGEIU,
    TGEU,
    TLT,
    TLTI,
    TLTIU,
    TLTU,
    TNE,
    TNEI,
    CACHE,
    ERET,
    MFC0,
    MTC0,
    TLBP,
    TLBR,
    TLBWI,
    TLBWR,
    ABS_fmt,
    ADD_fmt,
    BC1F,
    BC1FL,
    BC1T,
    BC1TL,
    C_cond_fmt,
    CEIL_L_fmt,
    CEIL_W_fmt,
    CFC1,
    CTC1,
    CVT_D_fmt,
    CVT_L_fmt,
    CVT_S_fmt,
    CVT_W_fmt,
    DIV_fmt,
    DMFC1,
    DMTC1,
    FLOOR_L_fmt,
    FLOOR_W_fmt,
    LDC1,
    LWC1,
    MFC1,
    MOV_fmt,
    MTC1,
    MUL_fmt,
    NEG_fmt,
    ROUND_L_fmt,
    ROUND_W_fmt,
    SDC1,
    SQRT_fmt,
    SUB_fmt,
    SWC1,
    TRUNC_L_fmt,
    TRUNC_W_fmt,
    NOP,
    MOVE, 
    NEG,
    NEGU, 
    BNEZ,
    BNEZL,
    BEQZ,
    BEQZL,
    B,
    BAL,  
    LI, 
    S_S, 
    L_S,
    UNIMPLEMENTED, 
}

impl Command
{
    pub fn from_opcode(opcode: u32) -> Command
    {
        let command_value: u8 = (opcode >> 26) as u8;
        let command2_value: u8 = ((opcode >> 21) & 0x0000001F) as u8;
        let secondary_value: u8 = (opcode & 0x0000003F) as u8;
        match command_value
        {
            0b000000 => 
            {
                match secondary_value
                {
                    0b000000 => Command::SLL,
                    0b100101 => Command::OR,
                    _ => Command::UNIMPLEMENTED,
                }
            },
            0b000011 => Command::JAL,
            0b000100 => Command::BEQ,
            0b000101 => Command::BNE,
            0b001000 => Command::ADDI,
            0b001001 => Command::ADDIU,
            0b001101 => Command::ORI,
            0b001111 => Command::LUI,
            0b010000 => 
            {
                match command2_value
                {
                    0b00100 => Command::MTC0,
                    _ => Command::UNIMPLEMENTED,
                }
            },
            0b100011 => Command::LW,
            0b101011 => Command::SW,
            _ => Command::UNIMPLEMENTED,
        }
    }



    pub fn parse(self, opcode: &Opcode, cpu: &mut CPU, connector: &mut Connector) -> Result<(), Error>
    {
        match self
        {
            Command::ADDI => execute_ADDI(opcode, cpu)?,
            Command::ADDIU => execute_ADDIU(opcode, cpu),
            Command::BEQ => execute_BEQ(opcode, cpu),
            Command::BNE => execute_BNE(opcode, cpu),
            Command::JAL => execute_JAL(opcode, cpu),
            Command::LUI => execute_LUI(opcode, cpu),
            Command::LW => execute_LW(opcode, cpu, connector)?,
            Command::MTC0 => execute_MTC0(opcode, cpu),
            Command::OR => execute_OR(opcode, cpu),
            Command::ORI => execute_ORI(opcode, cpu),
            Command::SLL => execute_SLL(opcode, cpu),
            Command::SW => execute_SW(opcode, cpu, connector)?,
            _ => return Err(Error::new(ErrorKind::Other, "Opcode not implemented.")),
        };
        Ok(())
    }
}

impl fmt::Display for Command
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
} 

fn execute_ADDI(opcode: &Opcode, cpu: &mut CPU) -> Result<(), Error>
{
    let new_value = add_u16_to_u32_as_i16_trap(cpu.cpu_registers.register[opcode.rs as usize].get_value() as u32, opcode.imm)?;
    cpu.cpu_registers.register[opcode.rt as usize].set_value(new_value);
    Ok(())
}

fn execute_ADDIU(opcode: &Opcode, cpu: &mut CPU)
{
    let new_value = add_u16_to_u32_as_i16_overflow(cpu.cpu_registers.register[opcode.rs as usize].get_value() as u32, opcode.imm);
    cpu.cpu_registers.register[opcode.rt as usize].set_value(new_value);
}

fn execute_BNE(opcode: &Opcode, cpu: &mut CPU)
{
    let l_value = cpu.cpu_registers.register[opcode.rs as usize].get_value() as u32;
    let r_value = cpu.cpu_registers.register[opcode.rt as usize].get_value() as u32;
    if l_value != r_value
    {
        let current_pc = cpu.program_counter.get_value() as i64;
        cpu.program_counter.set_value((current_pc + ((opcode.imm as i16 as i64) * 4)) as u32);
    }
}

fn execute_BEQ(opcode: &Opcode, cpu: &mut CPU)
{
    let l_value = cpu.cpu_registers.register[opcode.rs as usize].get_value() as u32;
    let r_value = cpu.cpu_registers.register[opcode.rt as usize].get_value() as u32;
    if l_value == r_value
    {
        let current_pc = cpu.program_counter.get_value() as i64;
        cpu.program_counter.set_value((current_pc + ((opcode.imm as i16 as i64) * 4)) as u32);
    }
}

fn execute_JAL(opcode: &Opcode, cpu: &mut CPU)
{
    cpu.cpu_registers.register[CPURegisterName::ra as usize].set_value(cpu.program_counter.get_value() as u32 + 4);
    let masked_pc: u32 = (cpu.program_counter.get_value() as u32) & 0xF0000000;
    cpu.program_counter.set_value(masked_pc | (opcode.target << 2));
}

fn execute_LUI(opcode: &Opcode, cpu: &mut CPU)
{
    let reg_value = cpu.cpu_registers.register[opcode.rt as usize].get_value() as u32;
    cpu.cpu_registers.register[opcode.rt as usize].set_value((reg_value & 0x0000FFFF) | ((opcode.imm as u32) << 16));
}

fn execute_LW(opcode: &Opcode, cpu: &mut CPU, connector: &Connector) -> Result<(), Error>
{
    let address = add_u16_to_u32_as_i16_overflow(cpu.cpu_registers.register[opcode.base as usize].get_value() as u32, opcode.imm);
    let new_value = connector.read_u32(address)?;
    cpu.cpu_registers.register[opcode.rt as usize].set_value(new_value);
    Ok(())
}


fn execute_MTC0(opcode: &Opcode, cpu: &mut CPU)
{
    let reg_value = cpu.cpu_registers.register[opcode.rt as usize].get_value() as u32;
    cpu.cop0_registers.register[opcode.fs as usize].set_value(reg_value);
}

fn execute_OR(opcode: &Opcode, cpu: &mut CPU)
{
    let l_value = cpu.cpu_registers.register[opcode.rs as usize].get_value() as u32;
    let r_value = cpu.cpu_registers.register[opcode.rt as usize].get_value() as u32;
    cpu.cpu_registers.register[opcode.rd as usize].set_value(l_value | r_value);
}

fn execute_ORI(opcode: &Opcode, cpu: &mut CPU)
{
    let new_value = cpu.cpu_registers.register[opcode.rs as usize].get_value() as u32;
    cpu.cpu_registers.register[opcode.rt as usize].set_value(new_value | (opcode.imm as u32));
}


fn execute_SLL(opcode: &Opcode, cpu: &mut CPU) 
{
    let new_value = cpu.cpu_registers.register[opcode.rt as usize].get_value() as u32;
    cpu.cpu_registers.register[opcode.rd as usize].set_value(new_value << (opcode.sa as u32));
}

fn execute_SW(opcode: &Opcode, cpu: &CPU, connector: &mut Connector) -> Result<(), Error>
{
    let new_value = cpu.cpu_registers.register[opcode.rt as usize].get_value() as u32;
    let address =  add_u16_to_u32_as_i16_overflow(cpu.cpu_registers.register[opcode.base as usize].get_value() as u32, opcode.offset);
    connector.store_u32(address, new_value)?;
    Ok(())
}



