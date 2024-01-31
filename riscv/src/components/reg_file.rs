use log::*;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, Range};
use std::{cell::RefCell, rc::Rc};
#[cfg(feature = "gui-egui")]
use syncrim::common::EguiComponent;
use syncrim::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, Signal, SignalUnsigned,
    Simulator,
};
use syncrim::signal::SignalValue;
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
    s10     = 26,   // Saved registers
    s11     = 27,   // Saved registers
    t3      = 28,   // Temporaries
    t4      = 29,   // Temporaries
    t5      = 30,   // Temporaries
    t6      = 31,   // Temporaries
}

pub const REG_FILE_MAX_DEPTH: usize = 4;

pub const REG_FILE_STACK_DEPTH_ID: &str = "stack_depth";
pub const REG_FILE_READ_ADDR1_ID: &str = "read_addr1";
pub const REG_FILE_READ_ADDR2_ID: &str = "read_addr2";
pub const REG_FILE_WRITE_DATA_ID: &str = "write_data";
pub const REG_FILE_WRITE_ADDR_ID: &str = "write_addr";
pub const REG_FILE_WRITE_ENABLE_ID: &str = "write_enable";

pub const REG_FILE_REG_A_OUT: &str = "reg_a";
pub const REG_FILE_REG_B_OUT: &str = "reg_b";

pub const REG_FILE_WIDTH: f32 = 250.0;
pub const REG_FILE_HEIGHT: f32 = 500.0;

#[derive(Serialize, Deserialize, Clone)]
pub struct RegFile {
    pub id: String,
    pub pos: (f32, f32),
    pub width: f32,
    pub height: f32,

    // ports
    pub stack_depth: Input,
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
    stack_depth: u8,
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

// The regfile is [u32; 32]; REG_FILE_MAX_DEPTH]
type RegStack = [[u32; 32]; REG_FILE_MAX_DEPTH];

#[derive(Serialize, Deserialize, Clone)]
pub struct RegStore(pub Rc<RefCell<RegStack>>);

impl RegStore {
    pub fn new(regs: Rc<RefCell<RegStack>>) -> Self {
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
        Self::new(Rc::new(RefCell::new([[0; 32]; REG_FILE_MAX_DEPTH])))
    }
}

impl Deref for RegStore {
    type Target = RefCell<[[u32; 32]; REG_FILE_MAX_DEPTH]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RegFile {
    fn read_reg(&self, simulator: &Simulator, stack_depth: usize, input: &Input) -> SignalValue {
        match simulator.get_input_value(input) {
            SignalValue::Data(read_addr) => {
                if read_addr > 0 {
                    trace!("read_addr {}", read_addr);
                    SignalValue::from(self.registers.borrow()[stack_depth][read_addr as usize])
                } else {
                    trace!("read_addr {}", read_addr);
                    SignalValue::from(0)
                }
            }
            _ => SignalValue::Unknown,
        }
    }

    pub fn dummy() -> RegFile {
        let dummy = Input::new("id", "field");
        RegFile {
            id: "dummy_reg_file".into(),
            pos: (0.0, 0.0),
            width: REG_FILE_WIDTH,
            height: REG_FILE_HEIGHT,
            stack_depth: dummy.clone(),
            read_addr1: dummy.clone(),
            read_addr2: dummy.clone(),
            write_data: dummy.clone(),
            write_addr: dummy.clone(),
            write_enable: dummy.clone(),
            registers: RegStore::new(Rc::new(RefCell::new([[0; 32]; REG_FILE_MAX_DEPTH]))),
            history: RegHistory::new(),
        }
    }
}

#[typetag::serde()]
impl Component for RegFile {
    fn to_(&self) {
        trace!("RegFile");
    }

    fn reset(&self) {
        self.registers
            .borrow_mut()
            .swap_with_slice(&mut [[0; 32]; REG_FILE_MAX_DEPTH]);
        self.history.0.swap(&RefCell::new(vec![]));
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![
                    InputPort {
                        port_id: REG_FILE_READ_ADDR1_ID.to_string(),
                        input: self.read_addr1.clone(),
                    },
                    InputPort {
                        port_id: REG_FILE_READ_ADDR2_ID.to_string(),
                        input: self.read_addr2.clone(),
                    },
                    InputPort {
                        port_id: REG_FILE_WRITE_DATA_ID.to_string(),
                        input: self.write_data.clone(),
                    },
                    InputPort {
                        port_id: REG_FILE_WRITE_ADDR_ID.to_string(),
                        input: self.write_addr.clone(),
                    },
                    InputPort {
                        port_id: REG_FILE_WRITE_ENABLE_ID.to_string(),
                        input: self.write_enable.clone(),
                    },
                ],
                out_type: OutputType::Combinatorial,
                outputs: vec!["reg_a".into(), "reg_b".into()],
            },
        )
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(RegFile {
            width: REG_FILE_WIDTH,
            height: REG_FILE_HEIGHT,
            id: id.to_string(),
            pos: (pos.0, pos.1),
            registers: RegStore::new(Rc::new(RefCell::new([[0; 32]; REG_FILE_MAX_DEPTH]))),
            history: RegHistory::new(),
            stack_depth: dummy_input.clone(),
            read_addr1: dummy_input.clone(),
            read_addr2: dummy_input.clone(),
            write_data: dummy_input.clone(),
            write_addr: dummy_input.clone(),
            write_enable: dummy_input.clone(),
        }))
    }
    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            REG_FILE_STACK_DEPTH_ID => self.stack_depth = new_input,
            REG_FILE_READ_ADDR1_ID => self.read_addr1 = new_input,
            REG_FILE_READ_ADDR2_ID => self.read_addr2 = new_input,
            REG_FILE_WRITE_DATA_ID => self.write_data = new_input,
            REG_FILE_WRITE_ADDR_ID => self.write_addr = new_input,
            REG_FILE_WRITE_ENABLE_ID => self.write_enable = new_input,
            _ => (),
        }
    }
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        let mut regop = RegOp {
            stack_depth: 0,
            read_addr1: 0,
            read_addr2: 0,
            write_addr2: None,
            old_data: None,
        };
        let stack_depth: SignalValue = simulator.get_input_value(&self.stack_depth);

        let stack_depth: SignalUnsigned = if let SignalValue::Data(v) = stack_depth {
            v
        } else {
            0
        };
        let stack_depth: usize = stack_depth as usize;

        if simulator.get_input_value(&self.write_enable) == (true as SignalUnsigned).into() {
            let data = simulator.get_input_value(&self.write_data);
            trace!("write data {:?}", data);

            let write_addr: SignalUnsigned = simulator
                .get_input_value(&self.write_addr)
                .try_into()
                .unwrap();

            trace!("write_addr {}", write_addr);

            regop.write_addr2 = Some((
                write_addr as u8,
                self.registers.borrow()[stack_depth as usize][write_addr as usize],
            ));

            self.registers.borrow_mut()[stack_depth as usize][write_addr as usize] =
                data.try_into().unwrap();
        }
        self.history.0.borrow_mut().push(regop);
        // read after write
        let reg_value_a = self.read_reg(simulator, stack_depth, &self.read_addr1);
        //regop.read_addr1 = simulator.get_input_value(&self.read_addr1).try_into().unwrap();
        trace!("reg_value_a {:?}", reg_value_a);
        simulator.set_out_value(&self.id, "reg_a", reg_value_a);

        let reg_value_b = self.read_reg(simulator, stack_depth, &self.read_addr2);
        trace!("reg_value_b {:?}", reg_value_b);
        simulator.set_out_value(&self.id, "reg_b", reg_value_b);
        Ok(())
    }

    fn un_clock(&self) {
        //println!("unclock");
        let regop = self.history.0.borrow_mut().pop().unwrap();
        let mut regstore = self.registers.borrow_mut();
        if let Some(w) = regop.write_addr2 {
            regstore[regop.stack_depth as usize][w.0 as usize] = w.1
        }
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
                Rc::new(ProbeOut::new("stack_depth")),
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
                    stack_depth: Input::new("stack_depth", "out"),
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

        let mut simulator = Simulator::new(cs).unwrap();

        assert_eq!(simulator.cycle, 1);

        // outputs
        let out_reg_1 = &Input::new("reg_file", "reg_a");
        let out_reg_2 = &Input::new("reg_file", "reg_b");

        // reset
        assert_eq!(simulator.get_input_value(out_reg_1), 0.into());
        assert_eq!(simulator.get_input_value(out_reg_2), 0.into());

        println!("<setup for clock 2>");
        simulator.set_out_value("stack_depth", "out", 0);
        simulator.set_out_value("read_reg_1", "out", 0);
        simulator.set_out_value("read_reg_2", "out", 1);
        simulator.set_out_value("write_data", "out", 1337);
        simulator.set_out_value("write_addr", "out", 1);
        simulator.set_out_value("write_enable", "out", true as SignalUnsigned);

        // test write and read to reg # 1 in same cycle
        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 2);
        assert_eq!(simulator.get_input_value(out_reg_1), 0.into());
        assert_eq!(simulator.get_input_value(out_reg_2), 1337.into());

        // test write and read to reg # 0 in same cycle (red #0 should always read 0)
        println!("<setup for clock 3>");
        simulator.set_out_value("read_reg_1", "out", 0);
        simulator.set_out_value("read_reg_2", "out", 1);
        simulator.set_out_value("write_data", "out", 42);
        simulator.set_out_value("write_addr", "out", 0);
        simulator.set_out_value("write_enable", "out", true as SignalUnsigned);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 3);
        assert_eq!(simulator.get_input_value(out_reg_1), 0.into());
        assert_eq!(simulator.get_input_value(out_reg_2), 1337.into());

        println!("<setup for clock 4>");
        simulator.set_out_value("stack_depth", "out", 1);
        simulator.set_out_value("read_reg_1", "out", 31);
        simulator.set_out_value("read_reg_2", "out", 1);
        simulator.set_out_value("write_data", "out", 1234);
        simulator.set_out_value("write_addr", "out", 31);
        simulator.set_out_value("write_enable", "out", true as SignalUnsigned);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 4);
        assert_eq!(simulator.get_input_value(out_reg_1), 1234.into());
        assert_eq!(simulator.get_input_value(out_reg_2), 0.into());

        println!("<un clock>");
        simulator.un_clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 3);
        assert_eq!(simulator.get_input_value(out_reg_1), 0.into());
        assert_eq!(simulator.get_input_value(out_reg_2), 1337.into());

        println!("<un clock>");
        simulator.un_clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 2);
        assert_eq!(simulator.get_input_value(out_reg_1), 0.into());
        assert_eq!(simulator.get_input_value(out_reg_2), 1337.into());

        println!("<un clock>");
        simulator.un_clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 1);
        assert_eq!(simulator.get_input_value(out_reg_1), 0.into());
        assert_eq!(simulator.get_input_value(out_reg_2), 0.into())
    }
}
