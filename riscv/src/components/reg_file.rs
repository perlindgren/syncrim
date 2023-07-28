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
    zero    = 0,    // Hard-wired zero
    ra      = 1,    // Return address
    sp      = 2,    // Stack pointer
    gp      = 3,    // Global pointer
    tp      = 4,    // Thread pointer
    t0      = 5,    // Temporaries
    t1      = 6,    // Temporaries
    t2      = 7,    // Temporaries
    s0      = 8,    // Saved register/frame pointer
    s1      = 9,    // Saved register
    a0      = 10,   // Function arguments/return values
    a1      = 11,   // Function arguments/return values
    a2      = 12,   // Function arguments
    a3      = 13,   // Function arguments
    a4      = 14,   // Function arguments
    a5      = 15,   // Function arguments
    a6      = 16,   // Function arguments
    a7      = 17,   // Function arguments
    s2      = 18,   // Saved registers
    s3      = 19,   // Saved registers
    s4      = 20,   // Saved registers
    s5      = 21,   // Saved registers
    s6      = 22,   // Saved registers
    s7      = 23,   // Saved registers
    s8      = 24,   // Saved registers
    s9      = 25,   // Saved registers
    s10      = 26,   // Saved registers
    s11      = 27,   // Saved registers
    t3      = 28,   // Temporaries
    t4      = 29,   // Temporaries
    t5      = 30,   // Temporaries
    t6      = 31,   // Temporaries
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
// TODO: Perhaps we want registers to be of Signal type (containing potentially Signal::Unknown)

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
    pub fn new(regs: Rc<RefCell<[u32; 32]>>) -> Self {
        RegStore(regs)
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
        Self::new(Rc::new(RefCell::new([0; 32])))
    }
}

impl Deref for RegStore {
    type Target = RefCell<[u32; 32]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RegFile {
    fn read_reg(&self, simulator: &Simulator, input: &Input) -> Signal {
        match simulator.get_input_val(input) {
            Signal::DontCare | Signal::Unknown | Signal::Uninitialized => return Signal::Unknown,
            Signal::Data(read_addr) => {
                if read_addr > 0 {
                    trace!("read_addr {}", read_addr);
                    return Signal::from(self.registers.borrow()[read_addr as usize]);
                } else {
                    trace!("read_addr {}", read_addr);
                    return Signal::from(0);
                }
            }
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
            trace!("write data {:?}", data);
            let write_addr: SignalUnsigned = simulator
                .get_input_val(&self.write_addr)
                .try_into()
                .unwrap();
            trace!("write_addr {}", write_addr);
            self.registers.borrow_mut()[write_addr as usize] = data.try_into().unwrap();
        }

        // read after write
        let reg_value_a = self.read_reg(simulator, &self.read_addr1);
        trace!("reg_value_a {:?}", reg_value_a);
        simulator.set_out_val(&self.id, "reg_a", reg_value_a);

        let reg_value_b = self.read_reg(simulator, &self.read_addr2);
        trace!("reg_value_b {:?}", reg_value_b);
        simulator.set_out_val(&self.id, "reg_b", reg_value_b);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::rc::Rc;
    use syncrim::{
        common::{ComponentStore, Input, Simulator},
        components::ProbeOut,
    };

    // an example of integration test for a mips specific component
    #[test]
    fn test_reg_file() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("read_reg_1")),
                Rc::new(ProbeOut::new("read_reg_2")),
                Rc::new(ProbeOut::new("write_data")),
                Rc::new(ProbeOut::new("write_addr")),
                Rc::new(ProbeOut::new("write_enable")),
                // regfile
                Rc::new(RegFile {
                    id: "reg_file".to_string(),
                    pos: (200.0, 150.0),
                    width: 100.0,
                    height: 150.0,

                    // ports
                    read_addr1: Input::new("read_reg_1", "out"),
                    read_addr2: Input::new("read_reg_2", "out"),
                    write_data: Input::new("write_data", "out"),
                    write_addr: Input::new("write_addr", "out"),
                    write_enable: Input::new("write_enable", "out"),

                    // data
                    registers: RegStore::default(),
                    history: RegHistory::new(),
                }),
            ],
        };

        let mut simulator = Simulator::new(&cs);

        assert_eq!(simulator.cycle, 1);

        // outputs
        let out_reg_1 = &Input::new("reg_file", "reg_a");
        let out_reg_2 = &Input::new("reg_file", "reg_b");

        // reset
        assert_eq!(simulator.get_input_val(out_reg_1), 0.into());
        assert_eq!(simulator.get_input_val(out_reg_2), 0.into());

        println!("<setup for clock 2>");
        simulator.set_out_val("read_reg_1", "out", 0);
        simulator.set_out_val("read_reg_2", "out", 1);
        simulator.set_out_val("write_data", "out", 1337);
        simulator.set_out_val("write_addr", "out", 1);
        simulator.set_out_val("write_enable", "out", true as SignalUnsigned);

        // test write and read to reg # 1 in same cycle
        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 2);
        assert_eq!(simulator.get_input_val(out_reg_1), 0.into());
        assert_eq!(simulator.get_input_val(out_reg_2), 1337.into());

        // test write and read to reg # 0 in same cycle (red #0 should always read 0)
        println!("<setup for clock 3>");
        simulator.set_out_val("read_reg_1", "out", 0);
        simulator.set_out_val("read_reg_2", "out", 1);
        simulator.set_out_val("write_data", "out", 42);
        simulator.set_out_val("write_addr", "out", 0);
        simulator.set_out_val("write_enable", "out", true as SignalUnsigned);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 3);
        assert_eq!(simulator.get_input_val(out_reg_1), 0.into());
        assert_eq!(simulator.get_input_val(out_reg_2), 1337.into());
    }
}
