use num::{NumCast, ToPrimitive, FromPrimitive};
use std::fmt;

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

    pub fn Debug(self)
    {
        println!("OPCODE DEBUG - 0x{:08x}", self.opcode);
        println!("COMMAND - {}\t {}", self.command, self.nuemonic);
        println!("rs: 0x{:02x}\trt: 0x{:02x}\trd: 0x{:02x}\tsa: 0x{:02x}", self.rs, self.rt, self.rd, self.sa);
        println!("fs: 0x{:02x}\tft: 0x{:02x}\tfd: 0x{:02x}\tbase: 0x{:02x}", self.fs, self.ft, self.fd, self.base);
        println!("imm: 0x{:04x}\toffset: 0x{:04x}\ttarget: 0x{:08x}", self.imm, self.offset, self.target);
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
            0b010000 => 
            {
                match command2_value
                {
                    0b00100 => Command::MTC0,
                    _ => Command::UNIMPLEMENTED,
                }
            },
            _ => Command::UNIMPLEMENTED,
        }
    }
}

impl fmt::Display for Command
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
} 