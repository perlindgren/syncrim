use crate::common::{Component, Input, Output, OutputType, Ports, Signal, Simulator};
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

use std::borrow::BorrowMut;
use std::{cell::RefCell, collections::HashMap, convert::TryFrom};

#[derive(Serialize, Deserialize)]
pub struct Mem {
    pub id: String,
    pub pos: (f32, f32),
    pub width: f32,
    pub height: f32,

    // configuration
    pub big_endian: bool,

    // ports
    pub data: Input,
    pub addr: Input,
    pub ctrl: Input,
    pub sign_extend: Input,
    pub size: Input,

    // memory
    pub memory: Memory,
    // later history... tbd
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Memory {
    bytes: RefCell<HashMap<usize, u8>>,
}

impl Memory {
    fn align(&self, addr: usize, size: usize) -> Signal {
        (addr % size == 0) as Signal
    }

    fn read(&self, addr: usize, size: usize, sign_extend: bool, big_endian: bool) -> Signal {
        let data: Vec<u8> = (0..size)
            .into_iter()
            .map(|i| *self.bytes.borrow().get(&(addr + i)).unwrap_or(&0))
            .collect();

        let data = data.as_slice();

        match size {
            1 => {
                if sign_extend {
                    data[0] as i8 as Signal
                } else {
                    data[0] as u8 as Signal
                }
            }
            2 => {
                if sign_extend {
                    if big_endian {
                        i16::from_be_bytes(data.try_into().unwrap()) as Signal
                    } else {
                        i16::from_le_bytes(data.try_into().unwrap()) as Signal
                    }
                } else {
                    if big_endian {
                        u16::from_be_bytes(data.try_into().unwrap()) as Signal
                    } else {
                        u16::from_le_bytes(data.try_into().unwrap()) as Signal
                    }
                }
            }
            4 => {
                if sign_extend {
                    if big_endian {
                        i32::from_be_bytes(data.try_into().unwrap()) as Signal
                    } else {
                        i32::from_le_bytes(data.try_into().unwrap()) as Signal
                    }
                } else {
                    if big_endian {
                        u32::from_be_bytes(data.try_into().unwrap()) as Signal
                    } else {
                        u32::from_le_bytes(data.try_into().unwrap()) as Signal
                    }
                }
            }
            _ => panic!("illegal sized memory operation"),
        }
    }

    fn write(&self, addr: usize, size: usize, big_endian: bool, data: Signal) {
        match size {
            1 => {
                self.bytes.borrow_mut().insert(addr, data as u8);
            }
            2 => {
                if big_endian {
                    (data as u16)
                        .to_be_bytes()
                        .iter()
                        .enumerate()
                        .for_each(|(i, bytes)| {
                            self.bytes.borrow_mut().insert(addr + i, *bytes);
                        })
                } else {
                    (data as u16)
                        .to_le_bytes()
                        .iter()
                        .enumerate()
                        .for_each(|(i, bytes)| {
                            self.bytes.borrow_mut().insert(addr + i, *bytes);
                        })
                }
            }

            4 => {
                if big_endian {
                    (data as u32)
                        .to_be_bytes()
                        .iter()
                        .enumerate()
                        .for_each(|(i, bytes)| {
                            self.bytes.borrow_mut().insert(addr + i, *bytes);
                        })
                } else {
                    (data as u32)
                        .to_le_bytes()
                        .iter()
                        .enumerate()
                        .for_each(|(i, bytes)| {
                            self.bytes.borrow_mut().insert(addr + i, *bytes);
                        })
                }
            }
            _ => {
                panic!("illegal sized memory operation")
            }
        };
    }
}

#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)] // Unfortunately Rust does not allow Signal here, we need to cast manually
pub enum MemCtrl {
    None,
    Read,
    Write,
}

#[typetag::serde()]
impl Component for Mem {
    fn to_(&self) {
        println!("Mem");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.data.clone(), self.addr.clone(), self.ctrl.clone()],
                out_type: OutputType::Combinatorial,
                // out 0 data
                // out 1 alignment error
                outputs: vec![Output::Function; 2],
            },
        )
    }

    fn evaluate(&self, simulator: &mut Simulator) {
        let data = simulator.get_input_val(&self.data);
        let addr = simulator.get_input_val(&self.addr) as usize;
        let ctrl = MemCtrl::try_from(simulator.get_input_val(&self.ctrl) as u8).unwrap();
        let size = simulator.get_input_val(&self.size) as usize;
        let sign_extend = simulator.get_input_val(&self.sign_extend) != 0;

        match ctrl {
            MemCtrl::Read => {
                println!("read addr {:?} size {:?}", addr, size);
                let value = self.memory.read(addr, size, sign_extend, self.big_endian);
                simulator.set_id_index(&self.id, 0, value)
            }
            MemCtrl::Write => {
                println!("write addr {:?} size {:?}", addr, size);
                self.memory.write(addr, size, self.big_endian, data)
            }
            MemCtrl::None => {
                println!("no read/write");
            }
        }
        println!("memory {:?}", self.memory);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::ComponentStore;
    use crate::components::ProbeOut;
    use std::rc::Rc;

    #[test]
    fn test_mem() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("data")),
                Rc::new(ProbeOut::new("addr")),
                Rc::new(ProbeOut::new("ctrl")),
                Rc::new(ProbeOut::new("size")),
                Rc::new(ProbeOut::new("sign_extend")),
                Rc::new(Mem {
                    id: "mem".to_string(),
                    pos: (0.0, 0.0),
                    width: 0.0,
                    height: 0.0,

                    // configuration
                    big_endian: false,

                    // ports
                    data: Input::new("data", 0),
                    addr: Input::new("addr", 0),
                    ctrl: Input::new("ctrl", 0),
                    size: Input::new("size", 0),
                    sign_extend: Input::new("sign_extend", 0),

                    // memory
                    memory: Memory {
                        bytes: RefCell::new(HashMap::new()),
                    },
                    // later history... tbd
                }),
            ],
        };

        let mut clock = 0;
        let mut simulator = Simulator::new(&cs, &mut clock);

        assert_eq!(clock, 1);

        // outputs
        let out = &Input::new("mem", 0);
        let err = &Input::new("mem", 1);

        // reset
        assert_eq!(simulator.get_input_val(out), 0);
        assert_eq!(simulator.get_input_val(err), false as Signal);

        println!("<setup for write 42 to addr 4>");

        simulator.set_id_index("data", 0, 42);
        simulator.set_id_index("addr", 0, 4);
        simulator.set_id_index("ctrl", 0, MemCtrl::Write as Signal);
        simulator.set_id_index("size", 0, 1); // byte

        println!("sim_state {:?}", simulator.sim_state);

        println!("<clock>");
        simulator.clock(&mut clock);
        println!("sim_state {:?}", simulator.sim_state);

        assert_eq!(clock, 2);
        assert_eq!(simulator.get_input_val(out), 0);
        assert_eq!(simulator.get_input_val(err), false as Signal);

        println!("<setup for read byte from addr 4>");

        simulator.set_id_index("ctrl", 0, MemCtrl::Read as Signal);
        simulator.set_id_index("size", 0, 1);

        simulator.clock(&mut clock);

        assert_eq!(clock, 3);
        assert_eq!(simulator.get_input_val(out), 42);
        assert_eq!(simulator.get_input_val(err), false as Signal);
    }
}
