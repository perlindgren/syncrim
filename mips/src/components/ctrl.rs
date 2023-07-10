#[allow(dead_code)]
pub enum AluOp {
    Nop = 0,
    Add = 1,
    Sub = 2,
    And = 3,
    Or = 4,
    Xor = 5,
    High16 = 6,
}

#[allow(dead_code)]
pub enum AinMux {
    Shamt = 0,
    PcOffset = 1,
    RegOutA = 2,
}

#[allow(dead_code)]
pub enum BinMux {
    RegOutA = 0,
    Pc = 1,
    ImmExt = 2,
}

#[allow(dead_code)]
pub enum PcMux {
    Pc4 = 0,
    Jump = 1,
    Branch = 2,
}

#[allow(dead_code)]
pub enum ImmExtend {
    Zero,
    Sign,
}

#[allow(dead_code)]
enum WbDataMux {}

#[allow(dead_code)]
enum WbRegMux {
    Rt,
    Rd,
    Ra,
}
