use log::*;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, Range};
use std::{cell::RefCell, rc::Rc};
#[cfg(feature = "gui-egui")]
use syncrim::common::EguiComponent;
use syncrim::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, SignalUnsigned, Simulator,
};
use syncrim::signal::SignalValue;
#[allow(non_camel_case_types)]
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, TryFromPrimitive, PartialEq)]
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

#[rustfmt::skip]
impl std::convert::TryFrom<u32> for Reg {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Reg::zero, // Hard-wired zero
            1 => Reg::ra,   // Return address
            2 => Reg::sp,   // Stack pointer
            3 => Reg::gp,   // Global pointer
            4 => Reg::tp,   // Thread pointer
            5 => Reg::t0,   // Temporaries
            6 => Reg::t1,   // Temporaries
            7 => Reg::t2,   // Temporaries
            8 => Reg::s0,   // Saved register/frame pointer
            9 => Reg::s1,   // Saved register
            10 =>Reg::a0,   // Function arguments/return values
            11 =>Reg::a1,   // Function arguments/return values
            12 =>Reg::a2,   // Function arguments
            13 =>Reg::a3,   // Function arguments
            14 =>Reg::a4,   // Function arguments
            15 =>Reg::a5,   // Function arguments
            16 =>Reg::a6,   // Function arguments
            17 =>Reg::a7,   // Function arguments
            18 =>Reg::s2,   // Saved registers
            19 =>Reg::s3,   // Saved registers
            20 =>Reg::s4,   // Saved registers
            21 =>Reg::s5,   // Saved registers
            22 =>Reg::s6,   // Saved registers
            23 =>Reg::s7,   // Saved registers
            24 =>Reg::s8,   // Saved registers
            25 =>Reg::s9,   // Saved registers
            26 =>Reg::s10,  // Saved registers
            27 =>Reg::s11,  // Saved registers
            28 =>Reg::t3,   // Temporaries
            29 =>Reg::t4,   // Temporaries
            30 =>Reg::t5,   // Temporaries
            31 =>Reg::t6,   // Temporaries
            _ => Err(())?,
        })
    }
}

//const REG_ZERO: u32 = Reg::zero as u32;
const REG_RA: u32 = Reg::ra as u32;

pub const REG_FILE_MAX_DEPTH: usize = 8;

pub const REG_FILE_STACK_DEPTH_ID: &str = "stack_depth";
//pub const REG_FILE_CLIC_MEPC_ID: &str = "clic_mepc";
pub const REG_FILE_CLIC_RA_WE_ID: &str = "clic_ra_we";

pub const REG_FILE_READ_ADDR1_ID: &str = "read_addr1";
pub const REG_FILE_READ_ADDR2_ID: &str = "read_addr2";
pub const REG_FILE_WRITE_DATA_ID: &str = "write_data";
pub const REG_FILE_WRITE_ADDR_ID: &str = "write_addr";
pub const REG_FILE_WRITE_ENABLE_ID: &str = "write_enable";

pub const REG_FILE_REG_A_OUT: &str = "reg_a";
pub const REG_FILE_REG_B_OUT: &str = "reg_b";
pub const REG_FILE_RA_OUT: &str = "ra";

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
    //pub clic_mepc: Input,
    pub clic_ra_we: Input,

    pub read_addr1: Input,
    pub read_addr2: Input,
    pub write_data: Input,
    pub write_addr: Input,
    pub write_enable: Input,

    // data
    #[serde(skip)]
    pub registers: RegStore,
    pub history: RegHistory,
    // this is purely for the graphical view
    // should be removed eventually with the gui
    // implementing tabs or something over the different
    // register sets
    #[serde(skip)]
    pub stack_depth_state: RefCell<u32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegOp {
    stack_depth: u8,
    read_addr1: u8,
    read_addr2: u8,
    write_addr2: Option<(u8, u32)>,
    old_data: Option<u8>,
    old_ra: Option<u32>,
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
    pub fn read_reg(&self, simulator: &Simulator, input: impl Into<SignalValue>) -> SignalValue {
        let input = input.into();
        let stack_depth: SignalUnsigned = simulator
            .get_input_value(&self.stack_depth)
            .try_into()
            .unwrap();
        let stack_depth = if stack_depth as i32 >= 0 {
            stack_depth
        } else {
            0
        };
        match input {
            SignalValue::Data(read_addr) => {
                trace!("read_addr {}", read_addr);
                let read_reg: Reg = read_addr.try_into().unwrap();
                let read_addr = read_reg as usize;
                match read_reg {
                    Reg::zero => {
                        // reg zero always reads 0
                        SignalValue::from(0)
                    }
                    Reg::sp => {
                        // reg sp shared among all stacks, we use stack_depth 0 for that
                        SignalValue::from(self.registers.borrow()[0][read_addr])
                    }
                    _ => {
                        // all other registers
                        SignalValue::from(self.registers.borrow()[stack_depth as usize][read_addr])
                    }
                }
            }
            _ => SignalValue::Unknown,
        }
    }

    fn write_reg(&self, simulator: &Simulator, input: impl Into<SignalValue>, data: SignalValue) {
        let input: SignalValue = input.into();
        let stack_depth: SignalUnsigned = simulator
            .get_input_value(&self.stack_depth)
            .try_into()
            .unwrap();
        let stack_depth = if stack_depth as i32 >= 0 {
            stack_depth
        } else {
            0
        };

        match input {
            SignalValue::Data(write_addr) => {
                trace!("write_addr {}", write_addr);
                let write_reg: Reg = write_addr.try_into().unwrap();
                match write_reg {
                    Reg::zero => {
                        // reg zero always reads 0
                        // do nothing on write
                    }
                    Reg::sp => {
                        // reg sp shared among all stacks, we use stack_depth 0 for that
                        self.registers.borrow_mut()[0][write_addr as usize] =
                            data.try_into().unwrap();
                    }
                    _ => {
                        // all other registers
                        self.registers.borrow_mut()[stack_depth as usize][write_addr as usize] =
                            data.try_into().unwrap();
                    }
                }
            }
            _ => {
                panic!("Unknown write address")
            }
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
            //clic_mepc: dummy.clone(),
            clic_ra_we: dummy.clone(),
            read_addr1: dummy.clone(),
            read_addr2: dummy.clone(),
            write_data: dummy.clone(),
            write_addr: dummy.clone(),
            write_enable: dummy.clone(),
            registers: RegStore::new(Rc::new(RefCell::new([[0; 32]; REG_FILE_MAX_DEPTH]))),
            history: RegHistory::new(),
            stack_depth_state: 0.into(),
        }
    }
}

#[typetag::serde()]
impl Component for RegFile {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

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
                        port_id: REG_FILE_STACK_DEPTH_ID.to_string(),
                        input: self.stack_depth.clone(),
                    },
                    // InputPort {
                    //     port_id: REG_FILE_CLIC_MEPC_ID.to_string(),
                    //     input: self.stack_depth.clone(),
                    //
                    //},
                    InputPort {
                        port_id: REG_FILE_CLIC_RA_WE_ID.to_string(),
                        input: self.stack_depth.clone(),
                    },
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
                outputs: vec![
                    REG_FILE_REG_A_OUT.into(),
                    REG_FILE_REG_B_OUT.into(),
                    REG_FILE_RA_OUT.into(),
                ],
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
            //clic_mepc: dummy_input.clone(),
            clic_ra_we: dummy_input.clone(),
            read_addr1: dummy_input.clone(),
            read_addr2: dummy_input.clone(),
            write_data: dummy_input.clone(),
            write_addr: dummy_input.clone(),
            write_enable: dummy_input.clone(),
            stack_depth_state: 0.into(),
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
            old_ra: None,
        };
        let stack_depth: SignalUnsigned = simulator
            .get_input_value(&self.stack_depth)
            .try_into()
            .unwrap();

        // special handling if clic_ra_we
        // THIS IS 2 WHEN
        let clic_ra_we =
            simulator.get_input_value(&self.clic_ra_we) == (true as SignalUnsigned).into();

        if clic_ra_we && (stack_depth as i32 >= 0) {
            // println!(
            //     "update ra register {:?}",
            //     simulator.get_input_value(&self.clic_mepc)
            // );
            let old_ra = self.read_reg(simulator, REG_RA);
            regop.old_ra = Some(old_ra.try_into().unwrap());
            // self.write_reg(
            //     simulator,
            //     REG_RA,
            //     simulator.get_input_value(&self.clic_mepc),
            // );
            // write magic number to RA
            self.write_reg(simulator, REG_RA, SignalValue::Data(0xFFFF_FFFF));
        }

        let stack_depth = stack_depth as usize;
        *self.stack_depth_state.borrow_mut() = stack_depth as u32;
        let read_addr1 = simulator.get_input_value(&self.read_addr1);
        let read_addr2 = simulator.get_input_value(&self.read_addr2);
        //*depth_state = stack_depth;

        if simulator.get_input_value(&self.write_enable) == (true as SignalUnsigned).into() {
            let data = simulator.get_input_value(&self.write_data);
            trace!(
                "write data {:?}",
                match data {
                    SignalValue::Data(data) => {
                        format!("{:x}", data)
                    }
                    _ => {
                        "unknown".to_string()
                    }
                }
            );
            let write_addr = simulator.get_input_value(&self.write_addr);

            regop.write_addr2 = Some((
                TryInto::<SignalUnsigned>::try_into(write_addr).unwrap() as u8,
                self.read_reg(simulator, write_addr).try_into().unwrap(), // read old value
            ));

            self.write_reg(&simulator, write_addr, data);
        }
        self.history.0.borrow_mut().push(regop);

        // read after write
        let reg_value_a = self.read_reg(simulator, read_addr1);
        trace!("reg_value_a {:?}", reg_value_a);
        simulator.set_out_value(&self.id, REG_FILE_REG_A_OUT, reg_value_a);

        let reg_value_b = self.read_reg(simulator, read_addr2);
        trace!("reg_value_b {:?}", reg_value_b);
        simulator.set_out_value(&self.id, REG_FILE_REG_B_OUT, reg_value_b);

        let reg_value_ra = self.read_reg(simulator, REG_RA);
        trace!("reg_value ra {:?}", reg_value_ra);
        simulator.set_out_value(&self.id, REG_FILE_RA_OUT, reg_value_ra);

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
                //Rc::new(ProbeOut::new("clic_mepc")),
                Rc::new(ProbeOut::new("clic_ra_we")),
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
                    //clic_mepc: Input::new("clic_mepc", "out"),
                    clic_ra_we: Input::new("clic_ra_we", "out"),

                    read_addr1: Input::new("read_reg_1", "out"),
                    read_addr2: Input::new("read_reg_2", "out"),
                    write_data: Input::new("write_data", "out"),
                    write_addr: Input::new("write_addr", "out"),
                    write_enable: Input::new("write_enable", "out"),

                    // data
                    registers: RegStore::default(),
                    history: RegHistory::new(),

                    stack_depth_state: 0.into(),
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

        // simulator.set_out_value("clic_write", "out", 1); // fatal test

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
