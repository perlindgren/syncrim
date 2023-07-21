use log::*;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, Range};
use std::{cell::RefCell, rc::Rc};
use syncrim::common::{Component, Input, OutputType, Ports, Signal, SignalUnsigned, Simulator};

#[allow(non_camel_case_types)]
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, TryFromPrimitive)]
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

#[derive(Serialize, Deserialize)]
pub struct RegFile {
    pub id: String,
    pub pos: (f32, f32),
    pub width: f32,
    pub height: f32,

    // ports
    pub read_addr1: Input,
    pub read_addr2: Input,
    pub write_data: Input,
    pub write_addr: Input,
    pub write_enable: Input,

    // data
    pub registers: RegStore,
    pub history: RegHistory,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegOp {
    read_addr1: u8,
    read_addr2: u8,
    write_addr2: Option<(u8, u32)>,
    old_data: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegHistory(RefCell<Vec<RegOp>>);

impl RegHistory {
    pub fn new() -> Self {
        RegHistory(RefCell::new(Vec::new()))
    }
}

impl Default for RegHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegStore(pub Rc<RefCell<[u32; 32]>>);

impl RegStore {
    pub fn new() -> Self {
        RegStore(Rc::new(RefCell::new([0; 32])))
    }

    pub fn full_range() -> Range<u8> {
        Range { start: 0, end: 32 }
    }

    pub fn lo_range() -> Range<u8> {
        Range { start: 0, end: 16 }
    }

    pub fn hi_range() -> Range<u8> {
        Range { start: 16, end: 32 }
    }
}

impl Default for RegStore {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for RegStore {
    type Target = RefCell<[u32; 32]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RegFile {
    fn read_reg(&self, simulator: &Simulator, input: &Input) -> u32 {
        let read_addr = simulator.get_input_val(input) as usize;
        trace!("read_addr {}", read_addr);

        // mips always reads 0;
        if read_addr > 0 {
            self.registers.borrow()[read_addr]
        } else {
            0
        }
    }
}

#[typetag::serde()]
impl Component for RegFile {
    fn to_(&self) {
        trace!("RegFile");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.read_addr1.clone(), self.read_addr2.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec!["reg_a".into(), "reg_b".into()],
            },
        )
    }

    fn clock(&self, simulator: &mut Simulator) {
        if simulator.get_input_val(&self.write_enable) == (true as SignalUnsigned).into() {
            let data = simulator.get_input_val(&self.write_data);
            trace!("data {:?}", data);
            let write_addr: usize = simulator
                .get_input_val(&self.write_addr)
                .try_into()
                .unwrap();
            trace!("write_addr {}", write_addr);
            self.registers.borrow_mut()[write_addr] = data;
        }

        // read after write
        let reg_value_a = self.read_reg(simulator, &self.read_addr1);
        trace!("reg_value {}", reg_value_a);
        simulator.set_out_val(&self.id, "reg_a", reg_value_a);

        let reg_value_b = self.read_reg(simulator, &self.read_addr2);
        trace!("reg_value {}", reg_value_b);
        simulator.set_out_val(&self.id, "reg_b", reg_value_b);
    }
}
