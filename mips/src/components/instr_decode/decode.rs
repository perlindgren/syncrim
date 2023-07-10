use crate::components::ctrl::{AinMux, AluOp, BinMux, ImmExtend, PcMux};
use num_enum::{IntoPrimitive, TryFromPrimitive};
// instruction encoding

#[derive(Debug)]
pub enum Instr {
    RType {
        op: Op,
        rs: Reg,
        rt: Reg,
        rd: Reg,
    },
    IType {
        op: Op,
        rs: Reg,
        rt: Reg,
        imm: Imm16,
    },
    JType {
        op: Op,
        imm: Imm26,
    },
}

type Imm16 = u16;
type Imm26 = u32;

#[allow(non_camel_case_types)]
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Reg {
    zero    = 0,    // Constant 0
    at      = 1,    // Reserved for assembler
    v0      = 2,    // Expression evaluation and results of function
    v1      = 3,    // Expression evaluation and results of function
    a0      = 4,    // Argument 1
    a1      = 5,    // Argument 2
    a2      = 6,    // Argument 3
    a3      = 7,    // Argument 4
    t0      = 8,    // Temporary (not preserved across calls)
    t1      = 9,    // Temporary (not preserved across calls)
    t2      = 10,   // Temporary (not preserved across calls)
    t3      = 11,   // Temporary (not preserved across calls)
    t4      = 12,   // Temporary (not preserved across calls)
    t5      = 13,   // Temporary (not preserved across calls)
    t6      = 14,   // Temporary (not preserved across calls)
    t7      = 15,   // Temporary (not preserved across calls)
    s0      = 16,   // Temporary (not preserved across calls)
    s1      = 17,   // Temporary (not preserved across calls)
    s2      = 18,   // Temporary (not preserved across calls)
    s3      = 19,   // Temporary (not preserved across calls)
    s4      = 20,   // Temporary (not preserved across calls)
    s5      = 21,   // Temporary (not preserved across calls)
    s6      = 22,   // Temporary (not preserved across calls)
    s7      = 23,   // Temporary (not preserved across calls)
    t8      = 24,   // Temporary (not preserved across calls)
    t9      = 25,   // Temporary (not preserved across calls)
    k0      = 26,   // Reserved for OS kernel
    k1      = 27,   // Reserved for OS kernel
    gp      = 28,   // Pointer to global area
    sp      = 29,   // Stack pointer
    fp      = 30,   // Frame pointer
    ra      = 31,   // Return address (used by function calls)
}

#[allow(non_camel_case_types)]
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Op {
    funct   = 0,
    rt      = 1,
    j       = 2,
    jal     = 3,
    beq     = 4,
    bne     = 5,
    blez    = 6,
    bgtz    = 7,
    addi    = 8,
    addiu   = 9,
    slti    = 10,
    sltiu   = 11,
    andi    = 12,
    ori     = 13,
    xori    = 14,
    lui     = 15,
    z0Rs    = 16,
    z1Rs    = 17,
    z2Rs    = 18,
    z4Rs    = 19,
    beql    = 20,
    bnel    = 21,
    blezl   = 22,
    bgtzl   = 23,
    lb      = 32,
    lh      = 33,
    lwl     = 34,
    lw      = 35,
    lbu     = 36,
    lhu     = 37,
    lwr     = 38,
    sb      = 40,
    sh      = 41,
    swl     = 42,
    sw      = 43,
    swr     = 46,
    cache   = 47,
    ll      = 48,
    lwc1    = 49,
    lwc2    = 50,
    pref    = 51,
    lcd1    = 53,
    ldc2    = 54,
    sc      = 56,
    swc1    = 57,
    swc2    = 58,
    scd1    = 61,
    scd2    = 62,
}

pub enum Type {
    R,
    I,
    J,
}

enum AluBMux {}
impl Op {
    fn ctrl(&self) -> (AluOp, PcMux) {
        let mut alu_op = AluOp::Nop;
        // AluOp
        match self {
            // branches
            Op::beq | Op::bne | Op::blez | Op::bgtz |
            Op::beql | Op::bnel | Op::blezl | Op::bgtzl |
            // immediate arithmetic
            Op::addi | Op::addiu |
            // memory operations
            Op::lb | Op::lh | Op::lwl | Op::lw |
            Op::lbu | Op::lhu | Op::lwr | Op::sb |
            Op::sh | Op::swl | Op::sw | Op::swr => alu_op = AluOp::Add,
            // comparisons
            Op::slti | Op::sltiu => alu_op = AluOp::Sub,
            // immediate logic
            Op::andi => alu_op = AluOp::And,
            Op::ori => alu_op = AluOp::Or,
            Op::xori => alu_op = AluOp::Xor,
            // lui
            Op::lui => alu_op = AluOp::High16,
            _ => panic!(),
        };

        // ImmExt
        let mut imm_ext = ImmExtend::Sign;
        match self {
            Op::andi | Op::ori | Op::xori | Op::lui => imm_ext = ImmExtend::Zero,
            _ => panic!(),
        };

        let mut pc_mux = PcMux::Pc4;
        // PcMux
        match self {
            Op::j | Op::jal => pc_mux = PcMux::Jump,
            // branches
            Op::beq | Op::bne | Op::blez | Op::bgtz |
            // branches likely
            Op::beql | Op::bnel | Op::blezl | Op::bgtzl => pc_mux = PcMux::Branch,
            _ => panic!(),
        };
        (alu_op, pc_mux)
    }
}

#[allow(non_camel_case_types)]
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum RsDecode {
    mfcz    = 0,
    cfcz    = 2,
    mtcz    = 4,
    ctcz    = 6,
    bcz     = 8,
    copz16  = 16,
    copz17  = 17,
}

#[allow(non_camel_case_types)]
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum RtDecode {
    bltz    = 0,
    bgez    = 1,
    bltzl   = 2,
    bgezl   = 3,
    tgei    = 8,
    tgeiu   = 9,
    tlti    = 10,
    tltiu   = 11,
    tegi    = 12,
    tnei    = 14,
    bltzal  = 16,
    bgezal  = 17,
    bltzall = 18,
    bgczall = 19,
}

#[allow(non_camel_case_types)]
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Funct {
    sll,
    srl,
    sra,
    sllv,
    srlv,
    srav,
    jr,
    jalr,
    movz,
    movn,
    syscall,
    _break,
    sync,
    mfhi,
    mthi,
    mflo,
    mtlo,
    mult,
    multu,
    div,
    divu,
    add,
    addu,
    sub,
    subu,
    and,
    or,
    xor,
    nor,
    slt,
    sltu,
    tge,
    tgeu,
    tlt,
    tltu,
    teq,
    tne,
}

pub fn decode(binary: u32) {
    if let Ok(op) = Op::try_from((binary >> 26) as u8) {
        println!("op {:?}", op);
    } else {
        println!("illegal op")
    }
}

#[cfg(test)]
mod test_decode {
    use super::*;

    #[test]
    fn test_lui() {
        let binary = 0x3c08_0000; // lui	t0,0x0
        let _x = decode(binary);
    }
    RType(Op, Rs, Rt, Rd, Sa, Function),
    IType(Op, Rs, Rt, Imm16),
    JType(Op, Imm26),
}

#[rustfmt::skip]
#[derive(Debug)]
pub enum Op {
    Funct   = 0,
    Rt      = 1,
    J       = 2,
    Jal     = 3,
    Beq     = 4,
    Bne     = 5,
    Blez    = 6,
    Bgtz    = 7,
    Addi    = 8,
    Addiu   = 9,
    Slti    = 10,
    Sltiu   = 11,
    Andi    = 12,
    Ori     = 13,
    Xori    = 14,
    Lui     = 15,
    Z0Rs    = 16,
    Z1Rs    = 17,
    Z2Rs    = 18,
    Z4Rs    = 19,
    Beql    = 20,
    Bnel    = 21,
    Blezl   = 22,
    Bgtzl   = 23,
    Lb      = 32,
    Lh      = 33,
    Lwl     = 34,
    Lw      = 35,
    Lbu     = 36,
    Lhu     = 37,
    Lwr     = 38,
    Sb      = 40,
    Sh      = 41,
    Swl     = 42,
    Sw      = 43,
    Swr     = 46,
    Cache   = 47,
    Ll      = 48,
    Lwc1    = 49,
    Lwc2    = 50,
    Pref    = 51,
    Lcd1    = 53,
    Ldc2    = 54,
    Sc      = 56,
    Swc1    = 57,
    Swc2    = 58,
    Scd1    = 61,
    Scd2    = 62,
}

#[rustfmt::skip]
#[derive(Debug)]
pub enum RsDecode {
    Mfcz    = 0,
    Cfcz    = 2,
    Mtcz    = 4,
    Ctcz    = 6,
    Bcz     = 8,
    Copz16  = 16,
    Copz17  = 17,
}

#[rustfmt::skip]
#[derive(Debug)]
pub enum RtDecode {
    Bltz    = 0,
    Bgez    = 1,
    Bltzl   = 2,
    Bgezl   = 3,
    Tgei    = 8,
    Tgeiu   = 9,
    Tlti    = 10,
    Tltiu   = 11,
    Tegi    = 12,
    Tnei    = 14,
    Bltzal  = 16,
    Bgezal  = 17,
    Bltzall = 18,
    Bgczall = 19,
}

#[rustfmt::skip]
#[derive(Debug)]
pub enum Funct {
    Sll,
    Srl,
    Sra,
    Sllv,
    Srlv,
    Srav,
    Jr,
    Jalr,
    Movz,
    Movn,
    Syscall,
    Break,
    Sync,
    Mfhi,
    Mthi,
    Mflo,
    Mtlo,
    Mult,
    Multu,
    Div,
    Divu,
    Add,
    Addu,
    Sub,
    Subu,
    And,
    Or,
    Xor,
    Nor,
    Slt,
    Sltu,
    Tge,
    Tgeu,
    Tlt,
    Tltu,
    Teq,
    Tne,
}
