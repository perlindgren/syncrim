// use std::fmt::Alignment;
#[cfg(feature = "gui-egui")]
use crate::common::EguiComponent;
use crate::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, SignalSigned, SignalUnsigned,
    SignalValue, Simulator,
};
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::ops::Deref;
use std::rc::Rc;

pub const REG_FILE_A1_IN_ID: &str = "reg_file_a1_in";
pub const REG_FILE_A2_IN_ID: &str = "reg_file_a2_in";
pub const REG_FILE_A3_IN_ID: &str = "reg_file_a3_in";
pub const REG_FILE_WD3_IN_ID: &str = "reg_file_wd3_in";
pub const REG_FILE_WE3_IN_ID: &str = "reg_file_we3_in";

pub const REG_FILE_RD1_OUT_ID: &str = "rd1_out";
pub const REG_FILE_RD2_OUT_ID: &str = "rd2_out";

#[derive(Serialize, Deserialize, Clone)]
pub struct RegFile {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) a1_in: Input,
    pub(crate) a2_in: Input,
    pub(crate) a3_in: Input,
    pub(crate) wd3_in: Input,
    pub(crate) we3_in: Input,

    pub big_endian: bool,

    #[serde(skip)]
    pub memory: Memory,
    history: RefCell<Vec<MemOp>>,
    #[serde(skip)]
    pub init_state: BTreeMap<usize, u8>,
}

#[derive(Serialize, Deserialize, Clone)]
struct MemOp {
    pub data: Option<usize>,
    pub addr: usize,
    pub size: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Memory(pub Rc<RefCell<BTreeMap<usize, u8>>>);

impl Default for Memory {
    fn default() -> Self {
        Self::new(BTreeMap::new())
    }
}

impl Memory {
    pub fn new(data: BTreeMap<usize, u8>) -> Self {
        Memory(Rc::new(RefCell::new(data)))
    }

    fn align(&self, addr: usize, size: usize) -> SignalValue {
        ((addr % size != 0) as SignalUnsigned).into()
    }

    pub fn read(&self, addr: usize, size: usize, sign: bool, big_endian: bool) -> SignalValue {
        let data: Vec<u8> = (0..size)
            .map(|i| *self.0.borrow().get(&(addr + i)).unwrap_or(&0))
            .collect();

        let data = data.as_slice();

        //trace!("{:x?}", data);

        match size {
            1 => {
                if sign {
                    data[0] as i8 as SignalSigned as SignalUnsigned
                } else {
                    data[0] as SignalUnsigned
                }
            }
            2 => {
                if sign {
                    if big_endian {
                        trace!("read signed half word be");
                        let i_16 = i16::from_be_bytes(data.try_into().unwrap());
                        trace!("i_16 {:x?}", i_16);
                        let i_32 = i_16 as i32;
                        trace!("i_32 {:x?}", i_32);
                        i_32 as SignalUnsigned
                    } else {
                        trace!("read signed half word le");
                        let i_16 = i16::from_le_bytes(data.try_into().unwrap());
                        trace!("i_16 {:x?}", i_16);
                        let i_32 = i_16 as i32;
                        trace!("i_32 {:x?}", i_32);
                        i_32 as SignalUnsigned
                    }
                } else if big_endian {
                    trace!("read unsigned half word be");
                    let u_16 = u16::from_be_bytes(data.try_into().unwrap());
                    trace!("u_16 {:x?}", u_16);
                    let u_32 = u_16 as u32;
                    trace!("u_32 {:x?}", u_32);
                    u_32 as SignalUnsigned
                } else {
                    trace!("read unsigned half word le");
                    let u_16 = u16::from_le_bytes(data.try_into().unwrap());
                    trace!("u_16 {:x?}", u_16);
                    let u_32 = u_16 as u32;
                    trace!("u_32 {:x?}", u_32);
                    u_32 as SignalUnsigned
                }
            }
            4 => {
                if sign {
                    if big_endian {
                        i32::from_be_bytes(data.try_into().unwrap()) as SignalUnsigned
                    } else {
                        i32::from_le_bytes(data.try_into().unwrap()) as SignalUnsigned
                    }
                } else if big_endian {
                    u32::from_be_bytes(data.try_into().unwrap()) as SignalUnsigned
                } else {
                    u32::from_le_bytes(data.try_into().unwrap()) as SignalUnsigned
                }
            }
            _ => panic!("illegal sized memory operation"),
        }
        .into()
    }

    pub fn write(&self, addr: usize, size: usize, big_endian: bool, data: SignalValue) {
        let data: SignalUnsigned = data.try_into().unwrap();
        trace!("we = 1, now writing {:?} at addr {:?}", data, addr);

        match size {
            1 => {
                trace!("write byte");
                self.0.borrow_mut().insert(addr, data as u8);
            }
            2 => {
                if big_endian {
                    trace!("write half word be");
                    (data as u16)
                        .to_be_bytes()
                        .iter()
                        .enumerate()
                        .for_each(|(i, bytes)| {
                            self.0.borrow_mut().insert(addr + i, *bytes);
                        })
                } else {
                    trace!("write half word le");
                    (data as u16)
                        .to_le_bytes()
                        .iter()
                        .enumerate()
                        .for_each(|(i, bytes)| {
                            self.0.borrow_mut().insert(addr + i, *bytes);
                        })
                }
            }

            4 => {
                if big_endian {
                    trace!("write word be");
                    data.to_be_bytes()
                        .iter()
                        .enumerate()
                        .for_each(|(i, bytes)| {
                            self.0.borrow_mut().insert(addr + i, *bytes);
                        })
                } else {
                    trace!("write word le");
                    data.to_le_bytes()
                        .iter()
                        .enumerate()
                        .for_each(|(i, bytes)| {
                            self.0.borrow_mut().insert(addr + i, *bytes);
                        })
                }
            }
            _ => {
                panic!("illegal sized memory operation, size = {}", size)
            }
        };
    }
}

#[typetag::serde]
impl Component for RegFile {
    fn to_(&self) {
        trace!("reg_file");
    }
    #[cfg(feature = "gui-egui")]
    // fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
    //     let dummy_input = Input::new("dummy", "out");
    //     Box::new(Rc::new(RegFile {
    //         id: "dummy".to_string(),
    //         pos: (0.0, 0.0),
    //         a1_in: dummy_input.clone(),
    //         a2_in: dummy_input.clone(),
    //         a3_in: dummy_input.clone(),
    //         wd3_in: dummy_input.clone(),
    //         we3_in: dummy_input.clone(),
    //     }))
    // }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: REG_FILE_A1_IN_ID.to_string(),
                        input: self.a1_in.clone(),
                    },
                    &InputPort {
                        port_id: REG_FILE_A2_IN_ID.to_string(),
                        input: self.a2_in.clone(),
                    },
                    &InputPort {
                        port_id: REG_FILE_A3_IN_ID.to_string(),
                        input: self.a3_in.clone(),
                    },
                    &InputPort {
                        port_id: REG_FILE_WD3_IN_ID.to_string(),
                        input: self.wd3_in.clone(),
                    },
                    &InputPort {
                        port_id: REG_FILE_WE3_IN_ID.to_string(),
                        input: self.we3_in.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![REG_FILE_RD1_OUT_ID, REG_FILE_RD2_OUT_ID],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            REG_FILE_A1_IN_ID => self.a1_in = new_input,
            REG_FILE_A2_IN_ID => self.a2_in = new_input,
            REG_FILE_A3_IN_ID => self.a3_in = new_input,
            REG_FILE_WD3_IN_ID => self.wd3_in = new_input,
            REG_FILE_WE3_IN_ID => self.we3_in = new_input,
            _ => {}
        }
    }

    // propagate sign extension to output
    // TODO: always extend to Signal size? (it should not matter and should be slightly cheaper)
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input values
        let mut history_entry = MemOp {
            data: None,
            addr: 0,
            size: 0,
        };

        let a1: u32 = simulator.get_input_value(&self.a1_in).try_into().unwrap();
        let a2: u32 = simulator.get_input_value(&self.a2_in).try_into().unwrap();
        let a3: u32 = simulator.get_input_value(&self.a3_in).try_into().unwrap();
        let wd3: SignalValue = simulator.get_input_value(&self.wd3_in);
        let we3: u32 = simulator.get_input_value(&self.we3_in).try_into().unwrap();

        let size = 4;
        let sign: bool = false; // in the mips, always read as unsigned

        let a1_addr = a1; // rs
        let a2_addr = a2; // rt
        let a3_addr = a3; // rt or rd depending on mux output, operation type

        // since the addr is only 5 bits, it cant be out of bounds, 2^5 = 32

        // read RD1 and RD2
        trace!("read addr {:?} size {:?}", a1_addr, size);
        let value1 = self
            .memory
            .read(a1_addr as usize, size as usize, sign, self.big_endian)
            .try_into()
            .unwrap();

        trace!("read addr {:?} size {:?}", a2_addr, size);
        let value2 = self
            .memory
            .read(a2_addr as usize, size as usize, sign, self.big_endian)
            .try_into()
            .unwrap();

        // if we, write to reg
        if we3 == 1 {
            let size = 4;
            history_entry = MemOp {
                data: match self.memory.read(
                    a3_addr as usize,
                    size as usize,
                    false,
                    self.big_endian,
                ) {
                    SignalValue::Data(d) => Some(d as usize),
                    _ => None,
                },

                addr: a3_addr as usize,
                size: size as u8,
            };
            trace!("write addr {:?} size {:?}", a3_addr, size);

            if a3_addr != 0 {
                self.memory
                    .write(a3_addr as usize, size as usize, self.big_endian, wd3);
            } else {
                // does nothing and reg remains 0
            }

            let value = self.memory.align(a3_addr as usize, size as usize);
            trace!("align {:?}", value);
        }

        simulator.set_out_value(&self.id, REG_FILE_RD1_OUT_ID, SignalValue::Data(value1));
        simulator.set_out_value(&self.id, REG_FILE_RD2_OUT_ID, SignalValue::Data(value2));

        self.history.borrow_mut().push(history_entry);
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Deref for Memory {
    type Target = RefCell<BTreeMap<usize, u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RegFile {
    pub fn new(
        id: &str,
        pos: (f32, f32),
        a1_in: Input,
        a2_in: Input,
        a3_in: Input,
        wd3_in: Input,
        we3_in: Input,
        big_endian: bool,
        memory: BTreeMap<usize, u8>,
    ) -> Self {
        RegFile {
            id: id.to_string(),
            pos,
            a1_in,
            a2_in,
            a3_in,
            wd3_in,
            we3_in,
            big_endian,
            memory: Memory::new(memory.clone()),
            history: RefCell::new(vec![]),
            init_state: memory.clone(),
        }
    }

    pub fn rc_new(
        id: &str,
        pos: (f32, f32),
        a1_in: Input,
        a2_in: Input,
        a3_in: Input,
        wd3_in: Input,
        we3_in: Input,
        big_endian: bool,
    ) -> Rc<Self> {
        let mut mem = BTreeMap::new();
        //fill the defined memory range with zeroes
        for i in 0..32.clone() {
            mem.insert(i as usize, 0u8);
        }
        Rc::new(RegFile::new(
            id, pos, a1_in, a2_in, a3_in, wd3_in, we3_in, big_endian, mem,
        ))
    }
}
