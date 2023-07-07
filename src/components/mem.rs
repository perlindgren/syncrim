use crate::common::{Component, Input, Output, OutputType, Ports, Signal, Simulator};
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

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
        (addr % size != 0) as Signal
    }

    fn read(&self, addr: usize, size: usize, sign_extend: bool, big_endian: bool) -> Signal {
        let data: Vec<u8> = (0..size)
            .into_iter()
            .map(|i| *self.bytes.borrow().get(&(addr + i)).unwrap_or(&0))
            .collect();

        let data = data.as_slice();

        println!("{:x?}", data);
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
                        println!("read signed half word be");
                        let i_16 = i16::from_be_bytes(data.try_into().unwrap());
                        println!("i_16 {:x?}", i_16);
                        let i_32 = i_16 as i32;
                        println!("i_32 {:x?}", i_32);
                        i_32 as Signal
                    } else {
                        println!("read signed half word le");
                        let i_16 = i16::from_le_bytes(data.try_into().unwrap());
                        println!("i_16 {:x?}", i_16);
                        let i_32 = i_16 as i32;
                        println!("i_32 {:x?}", i_32);
                        i_32 as Signal
                    }
                } else {
                    if big_endian {
                        println!("read unsigned half word be");
                        let u_16 = u16::from_be_bytes(data.try_into().unwrap());
                        println!("u_16 {:x?}", u_16);
                        let u_32 = u_16 as u32;
                        println!("u_32 {:x?}", u_32);
                        u_32 as Signal
                    } else {
                        println!("read unsigned half word le");
                        let u_16 = u16::from_le_bytes(data.try_into().unwrap());
                        println!("u_16 {:x?}", u_16);
                        let u_32 = u_16 as u32;
                        println!("u_32 {:x?}", u_32);
                        u_32 as Signal
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
                println!("write byte");
                self.bytes.borrow_mut().insert(addr, data as u8);
            }
            2 => {
                if big_endian {
                    println!("write half word be");
                    (data as u16)
                        .to_be_bytes()
                        .iter()
                        .enumerate()
                        .for_each(|(i, bytes)| {
                            self.bytes.borrow_mut().insert(addr + i, *bytes);
                        })
                } else {
                    println!("write half word le");
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
                    println!("write word be");
                    (data as u32)
                        .to_be_bytes()
                        .iter()
                        .enumerate()
                        .for_each(|(i, bytes)| {
                            self.bytes.borrow_mut().insert(addr + i, *bytes);
                        })
                } else {
                    println!("write word le");
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
                simulator.set_id_index(&self.id, 0, value);
                let value = self.memory.align(addr, size);
                println!("align {}", value);
                simulator.set_id_index(&self.id, 1, value); // align
            }
            MemCtrl::Write => {
                println!("write addr {:?} size {:?}", addr, size);
                self.memory.write(addr, size, self.big_endian, data);
                let value = self.memory.align(addr, size);
                println!("align {}", value);
                simulator.set_id_index(&self.id, 1, value); // align
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
    fn test_mem_be() {
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
                    big_endian: true, // i.e., big endian

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

        simulator.set_id_index("data", 0, 0xf0);
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
        assert_eq!(simulator.get_input_val(out), 0xf0);
        assert_eq!(simulator.get_input_val(err), false as Signal);

        println!("<setup for read byte from addr 4>");
        simulator.set_id_index("size", 0, 1); // b
        simulator.set_id_index("sign_extend", 0, true as Signal);

        simulator.clock(&mut clock);
        assert_eq!(clock, 4);
        assert_eq!(simulator.get_input_val(out), 0xffff_fff0);
        assert_eq!(simulator.get_input_val(err), false as Signal);

        println!("<setup for read half-word from addr 4>");
        simulator.set_id_index("size", 0, 2); // b
        simulator.set_id_index("sign_extend", 0, true as Signal);

        simulator.clock(&mut clock);
        assert_eq!(clock, 5);
        assert_eq!(simulator.get_input_val(out), 0xffff_f000);
        assert_eq!(simulator.get_input_val(err), false as Signal);

        println!("<setup for read word from addr 4>");
        simulator.set_id_index("size", 0, 4); // b
        simulator.set_id_index("sign_extend", 0, true as Signal);

        simulator.clock(&mut clock);
        assert_eq!(clock, 6);
        assert_eq!(simulator.get_input_val(out), 0xf000_0000);
        assert_eq!(simulator.get_input_val(err), false as Signal);

        println!("<setup for read word from addr 5>");
        simulator.set_id_index("addr", 0, 5); // b

        simulator.clock(&mut clock);
        assert_eq!(clock, 7);
        assert_eq!(simulator.get_input_val(err), true as Signal);

        println!("<setup for read word from addr 6>");
        simulator.set_id_index("addr", 0, 6); // b

        simulator.clock(&mut clock);
        assert_eq!(clock, 8);
        assert_eq!(simulator.get_input_val(err), true as Signal);

        println!("<setup for read word from addr 7>");
        simulator.set_id_index("addr", 0, 7); // b

        simulator.clock(&mut clock);
        assert_eq!(clock, 9);
        assert_eq!(simulator.get_input_val(err), true as Signal);

        println!("<setup for read word from addr 8>");
        simulator.set_id_index("addr", 0, 8); // b

        simulator.clock(&mut clock);
        assert_eq!(clock, 10);
        assert_eq!(simulator.get_input_val(err), false as Signal);

        println!("<setup for read half-word from addr 9>");
        simulator.set_id_index("addr", 0, 9); // b
        simulator.set_id_index("size", 0, 2); // b
        simulator.clock(&mut clock);
        assert_eq!(clock, 11);
        assert_eq!(simulator.get_input_val(err), true as Signal);

        println!("<setup for read half-word from addr 10>");
        simulator.set_id_index("addr", 0, 10); // b

        simulator.clock(&mut clock);
        assert_eq!(clock, 12);
        assert_eq!(simulator.get_input_val(err), false as Signal);

        println!("<setup for write half-word at add 10>");
        simulator.set_id_index("addr", 0, 10); // b
        simulator.set_id_index("data", 0, 0x1234);
        simulator.set_id_index("ctrl", 0, MemCtrl::Write as Signal);
        simulator.clock(&mut clock);
        assert_eq!(clock, 13);
        assert_eq!(simulator.get_input_val(err), false as Signal);

        println!("<setup for read byte at add 10>");
        simulator.set_id_index("ctrl", 0, MemCtrl::Read as Signal);
        simulator.set_id_index("size", 0, 1);
        simulator.clock(&mut clock);
        assert_eq!(clock, 14);
        assert_eq!(simulator.get_input_val(out), 0x12 as Signal);

        println!("<setup for read byte at add 11>");
        simulator.set_id_index("addr", 0, 11);
        simulator.clock(&mut clock);
        assert_eq!(clock, 15);
        assert_eq!(simulator.get_input_val(out), 0x34 as Signal);

        println!("test done")
    }

    #[test]
    fn test_mem_le() {
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
                    big_endian: false, // i.e., little endian

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

        simulator.set_id_index("data", 0, 0xf0);
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
        assert_eq!(simulator.get_input_val(out), 0xf0);
        assert_eq!(simulator.get_input_val(err), false as Signal);

        println!("<setup for read byte from addr 4>");
        simulator.set_id_index("size", 0, 1); // b
        simulator.set_id_index("sign_extend", 0, true as Signal);

        simulator.clock(&mut clock);
        assert_eq!(clock, 4);
        assert_eq!(simulator.get_input_val(out), 0xffff_fff0);
        assert_eq!(simulator.get_input_val(err), false as Signal);

        println!("<setup for read half-word from addr 4>");
        simulator.set_id_index("size", 0, 2); // b
        simulator.set_id_index("sign_extend", 0, true as Signal);

        simulator.clock(&mut clock);
        assert_eq!(clock, 5);
        assert_eq!(simulator.get_input_val(out), 0x0000_00f0);
        assert_eq!(simulator.get_input_val(err), false as Signal);

        println!("<setup for read word from addr 4>");
        simulator.set_id_index("size", 0, 4); // b
        simulator.set_id_index("sign_extend", 0, true as Signal);
        simulator.clock(&mut clock);
        assert_eq!(clock, 6);
        assert_eq!(simulator.get_input_val(out), 0x0000_00f0);
        assert_eq!(simulator.get_input_val(err), false as Signal);

        println!("<setup for write half-word at add 10>");
        simulator.set_id_index("addr", 0, 10); // b
        simulator.set_id_index("data", 0, 0x1234);
        simulator.set_id_index("ctrl", 0, MemCtrl::Write as Signal);
        simulator.set_id_index("size", 0, 2);

        simulator.clock(&mut clock);
        assert_eq!(clock, 7);
        assert_eq!(simulator.get_input_val(err), false as Signal);

        println!("<setup for read byte at add 10>");
        simulator.set_id_index("ctrl", 0, MemCtrl::Read as Signal);
        simulator.set_id_index("size", 0, 1);
        simulator.clock(&mut clock);
        assert_eq!(clock, 8);
        assert_eq!(simulator.get_input_val(out), 0x34 as Signal);

        println!("<setup for read byte at add 11>");
        simulator.set_id_index("addr", 0, 11);
        simulator.clock(&mut clock);
        assert_eq!(clock, 9);
        assert_eq!(simulator.get_input_val(out), 0x12 as Signal);
    }
}
