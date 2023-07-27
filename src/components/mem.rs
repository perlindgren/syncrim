use crate::common::{
    Component, Id, Input, InputPort, OutputType, Ports, Signal, SignalSigned, SignalUnsigned,
    Simulator,
};
use log::*;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, convert::TryFrom};

pub const MEM_DATA_ID: &str = "data";
pub const MEM_ADDR_ID: &str = "addr";
pub const MEM_CTRL_ID: &str = "ctrl";
pub const MEM_SIGN_ID: &str = "sign";
pub const MEM_SIZE_ID: &str = "size";

pub const MEM_DATA_OUT_ID: &str = "data";
pub const MEM_ERR_OUT_ID: &str = "err";

#[derive(Serialize, Deserialize)]
pub struct Mem {
    pub id: Id,
    pub pos: (f32, f32),
    pub width: f32,
    pub height: f32,

    // configuration
    pub big_endian: bool,

    // ports
    pub data: Input,
    pub addr: Input,
    pub ctrl: Input,
    pub sign: Input,
    pub size: Input,

    // memory
    pub memory: Memory,
    // later history... tbd
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Memory {
    bytes: RefCell<HashMap<usize, u8>>,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            bytes: RefCell::new(HashMap::new()),
        }
    }

    fn align(&self, addr: usize, size: usize) -> Signal {
        Signal::Data((addr % size != 0) as SignalUnsigned)
    }

    fn read(&self, addr: usize, size: usize, sign: bool, big_endian: bool) -> Signal {
        let data: Vec<u8> = (0..size)
            .map(|i| *self.bytes.borrow().get(&(addr + i)).unwrap_or(&0))
            .collect();

        let data = data.as_slice();

        trace!("{:x?}", data);

        Signal::Data(match size {
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
        })
    }

    fn write(&self, addr: usize, size: usize, big_endian: bool, data: Signal) {
        let data: SignalUnsigned = data.try_into().unwrap();
        match size {
            1 => {
                trace!("write byte");
                self.bytes.borrow_mut().insert(addr, data as u8);
            }
            2 => {
                if big_endian {
                    trace!("write half word be");
                    (data as u16)
                        .to_be_bytes()
                        .iter()
                        .enumerate()
                        .for_each(|(i, bytes)| {
                            self.bytes.borrow_mut().insert(addr + i, *bytes);
                        })
                } else {
                    trace!("write half word le");
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
                    trace!("write word be");
                    data.to_be_bytes()
                        .iter()
                        .enumerate()
                        .for_each(|(i, bytes)| {
                            self.bytes.borrow_mut().insert(addr + i, *bytes);
                        })
                } else {
                    trace!("write word le");
                    data.to_le_bytes()
                        .iter()
                        .enumerate()
                        .for_each(|(i, bytes)| {
                            self.bytes.borrow_mut().insert(addr + i, *bytes);
                        })
                }
            }
            _ => {
                panic!("illegal sized memory operation, size = {}", size)
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
        trace!("Mem");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: MEM_DATA_ID.to_string(),
                        input: self.data.clone(),
                    },
                    &InputPort {
                        port_id: MEM_ADDR_ID.to_string(),
                        input: self.addr.clone(),
                    },
                    &InputPort {
                        port_id: MEM_CTRL_ID.to_string(),
                        input: self.ctrl.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![MEM_DATA_OUT_ID, MEM_ERR_OUT_ID],
            ),
        )
    }

    fn clock(&self, simulator: &mut Simulator) {
        let data = simulator.get_input_val(&self.data);
        let addr: SignalUnsigned = simulator.get_input_val(&self.addr).try_into().unwrap();
        let addr = addr as usize;
        let ctrl: SignalUnsigned = simulator.get_input_val(&self.ctrl).try_into().unwrap();
        let ctrl = MemCtrl::try_from(ctrl as u8).unwrap();
        let size: SignalUnsigned = simulator.get_input_val(&self.size).try_into().unwrap();
        let size = size as usize;
        let sign: SignalUnsigned = simulator.get_input_val(&self.sign).try_into().unwrap();
        let sign = sign != 0;

        match ctrl {
            MemCtrl::Read => {
                trace!("read addr {:?} size {:?}", addr, size);
                let value = self.memory.read(addr, size, sign, self.big_endian);
                simulator.set_out_val(&self.id, "data", value);
                let value = self.memory.align(addr, size);
                trace!("align {:?}", value);
                simulator.set_out_val(&self.id, "err", value); // align
            }
            MemCtrl::Write => {
                trace!("write addr {:?} size {:?}", addr, size);
                self.memory.write(addr, size, self.big_endian, data);
                let value = self.memory.align(addr, size);
                trace!("align {:?}", value);
                simulator.set_out_val(&self.id, "err", value); // align
            }
            MemCtrl::None => {
                trace!("no read/write");
            }
        }

        trace!("memory {:?}", self.memory);
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            MEM_DATA_ID => self.data = new_input,
            MEM_ADDR_ID => self.addr = new_input,
            MEM_CTRL_ID => self.ctrl = new_input,
            MEM_SIGN_ID => self.sign = new_input,
            MEM_SIZE_ID => self.size = new_input,
            _ => (),
        }
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
                Rc::new(ProbeOut::new("sign")),
                Rc::new(Mem {
                    id: "mem".into(),
                    pos: (0.0, 0.0),
                    width: 0.0,
                    height: 0.0,

                    // configuration
                    big_endian: true, // i.e., big endian

                    // ports
                    data: Input::new("data", "out"),
                    addr: Input::new("addr", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    size: Input::new("size", "out"),
                    sign: Input::new("sign", "out"),

                    // memory
                    memory: Memory {
                        bytes: RefCell::new(HashMap::new()),
                    },
                }),
            ],
        };

        let mut simulator = Simulator::new(cs);

        assert_eq!(simulator.cycle, 1);

        // outputs
        let out = &Input::new("mem", "data");
        let err = &Input::new("mem", "err");

        // reset
        assert_eq!(simulator.get_input_val(out), 0.into());
        assert_eq!(
            simulator.get_input_val(err),
            (false as SignalUnsigned).into()
        );

        println!("<setup for write 42 to addr 4>");

        simulator.set_out_val("data", "out", 0xf0);
        simulator.set_out_val("addr", "out", 4);
        simulator.set_out_val("ctrl", "out", MemCtrl::Write as SignalUnsigned);
        simulator.set_out_val("size", "out", 1);
        println!("sim_state {:?}", simulator.sim_state);

        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);

        assert_eq!(simulator.cycle, 2);
        assert_eq!(simulator.get_input_val(out), 0.into());
        assert_eq!(
            simulator.get_input_val(err),
            (false as SignalUnsigned).into()
        );

        println!("<setup for read byte from addr 4>");

        simulator.set_out_val("ctrl", "out", MemCtrl::Read as SignalUnsigned);
        simulator.set_out_val("size", "out", 1);

        simulator.clock();

        assert_eq!(simulator.cycle, 3);
        assert_eq!(simulator.get_input_val(out), 0xf0.into());
        assert_eq!(
            simulator.get_input_val(err),
            (false as SignalUnsigned).into()
        );

        println!("<setup for read byte from addr 4>");
        simulator.set_out_val("size", "out", 1);
        simulator.set_out_val("sign", "out", true);

        simulator.clock();
        assert_eq!(simulator.cycle, 4);
        assert_eq!(simulator.get_input_val(out), 0xffff_fff0.into());
        assert_eq!(
            simulator.get_input_val(err),
            (false as SignalUnsigned).into()
        );

        println!("<setup for read half-word from addr 4>");
        simulator.set_out_val("size", "out", 2);
        simulator.set_out_val("sign", "out", true as SignalUnsigned);

        simulator.clock();
        assert_eq!(simulator.cycle, 5);
        assert_eq!(simulator.get_input_val(out), 0xffff_f000.into());
        assert_eq!(
            simulator.get_input_val(err),
            (false as SignalUnsigned).into()
        );

        println!("<setup for read word from addr 4>");
        simulator.set_out_val("size", "out", 4);
        simulator.set_out_val("sign", "out", true);

        simulator.clock();
        assert_eq!(simulator.cycle, 6);
        assert_eq!(simulator.get_input_val(out), 0xf000_0000.into());
        assert_eq!(simulator.get_input_val(err), false.into());

        println!("<setup for read word from addr 5>");
        simulator.set_out_val("addr", "out", 5);

        simulator.clock();
        assert_eq!(simulator.cycle, 7);
        assert_eq!(simulator.get_input_val(err), true.into());

        println!("<setup for read word from addr 6>");
        simulator.set_out_val("addr", "out", 6);

        simulator.clock();
        assert_eq!(simulator.cycle, 8);
        assert_eq!(simulator.get_input_val(err), true.into());

        println!("<setup for read word from addr 7>");
        simulator.set_out_val("addr", "out", 7);

        simulator.clock();
        assert_eq!(simulator.cycle, 9);
        assert_eq!(simulator.get_input_val(err), true.into());

        println!("<setup for read word from addr 8>");
        simulator.set_out_val("addr", "out", 8);

        simulator.clock();
        assert_eq!(simulator.cycle, 10);
        assert_eq!(simulator.get_input_val(err), false.into());

        println!("<setup for read half-word from addr 9>");
        simulator.set_out_val("addr", "out", 9);
        simulator.set_out_val("size", "out", 2);
        simulator.clock();
        assert_eq!(simulator.cycle, 11);
        assert_eq!(simulator.get_input_val(err), true.into());

        println!("<setup for read half-word from addr 10>");
        simulator.set_out_val("addr", "out", 10);

        simulator.clock();
        assert_eq!(simulator.cycle, 12);
        assert_eq!(simulator.get_input_val(err), false.into());

        println!("<setup for write half-word at add 10>");
        simulator.set_out_val("addr", "out", 10);
        simulator.set_out_val("data", "out", 0x1234);
        simulator.set_out_val("ctrl", "out", MemCtrl::Write as SignalUnsigned);
        simulator.clock();
        assert_eq!(simulator.cycle, 13);
        assert_eq!(simulator.get_input_val(err), false.into());

        println!("<setup for read byte at add 10>");
        simulator.set_out_val("ctrl", "out", MemCtrl::Read as SignalUnsigned);
        simulator.set_out_val("size", "out", 1);
        simulator.clock();
        assert_eq!(simulator.cycle, 14);
        assert_eq!(simulator.get_input_val(out), 0x12.into());

        println!("<setup for read byte at add 11>");
        simulator.set_out_val("addr", "out", 11);
        simulator.clock();
        assert_eq!(simulator.cycle, 15);
        assert_eq!(simulator.get_input_val(out), 0x34.into());

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
                Rc::new(ProbeOut::new("sign")),
                Rc::new(Mem {
                    id: "mem".into(),
                    pos: (0.0, 0.0),
                    width: 0.0,
                    height: 0.0,

                    // configuration
                    big_endian: false, // i.e., little endian

                    // ports
                    data: Input::new("data", "out"),
                    addr: Input::new("addr", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    size: Input::new("size", "out"),
                    sign: Input::new("sign", "out"),

                    // memory
                    memory: Memory {
                        bytes: RefCell::new(HashMap::new()),
                    },
                    // later history... tbd
                }),
            ],
        };

        let mut simulator = Simulator::new(cs);

        assert_eq!(simulator.cycle, 1);

        // outputs
        let out = &Input::new("mem", "data");
        let err = &Input::new("mem", "err");

        // reset
        assert_eq!(simulator.get_input_val(out), 0.into());
        assert_eq!(simulator.get_input_val(err), false.into());

        // println!("<setup for write 42 to addr 4>");

        simulator.set_out_val("data", "out", 0xf0);
        simulator.set_out_val("addr", "out", 4);
        simulator.set_out_val("ctrl", "out", MemCtrl::Write as SignalUnsigned);
        simulator.set_out_val("size", "out", 1); // byte

        println!("sim_state {:?}", simulator.sim_state);

        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);

        assert_eq!(simulator.cycle, 2);
        assert_eq!(simulator.get_input_val(out), 0.into());
        assert_eq!(simulator.get_input_val(err), false.into());

        println!("<setup for read byte from addr 4>");

        simulator.set_out_val("ctrl", "out", MemCtrl::Read as SignalUnsigned);
        simulator.set_out_val("size", "out", 1);

        simulator.clock();

        assert_eq!(simulator.cycle, 3);
        assert_eq!(simulator.get_input_val(out), 0xf0.into());
        assert_eq!(simulator.get_input_val(err), false.into());

        println!("<setup for read byte from addr 4>");
        simulator.set_out_val("size", "out", 1);
        simulator.set_out_val("sign", "out", true);

        simulator.clock();
        assert_eq!(simulator.cycle, 4);
        assert_eq!(simulator.get_input_val(out), 0xffff_fff0.into());
        assert_eq!(simulator.get_input_val(err), false.into());

        println!("<setup for read half-word from addr 4>");
        simulator.set_out_val("size", "out", 2);
        simulator.set_out_val("sign", "out", true);

        simulator.clock();
        assert_eq!(simulator.cycle, 5);
        assert_eq!(simulator.get_input_val(out), 0x0000_00f0.into());
        assert_eq!(simulator.get_input_val(err), false.into());

        println!("<setup for read word from addr 4>");
        simulator.set_out_val("size", "out", 4);
        simulator.set_out_val("sign", "out", true);
        simulator.clock();
        assert_eq!(simulator.cycle, 6);
        assert_eq!(simulator.get_input_val(out), 0x0000_00f0.into());
        assert_eq!(simulator.get_input_val(err), false.into());

        println!("<setup for write half-word at add 10>");
        simulator.set_out_val("addr", "out", 10); // b
        simulator.set_out_val("data", "out", 0x1234);
        simulator.set_out_val("ctrl", "out", MemCtrl::Write as SignalUnsigned);
        simulator.set_out_val("size", "out", 2);

        simulator.clock();
        assert_eq!(simulator.cycle, 7);
        assert_eq!(simulator.get_input_val(err), false.into());

        println!("<setup for read byte at add 10>");
        simulator.set_out_val("ctrl", "out", MemCtrl::Read as SignalUnsigned);
        simulator.set_out_val("size", "out", 1);
        simulator.clock();
        assert_eq!(simulator.cycle, 8);
        assert_eq!(simulator.get_input_val(out), 0x34.into());

        println!("<setup for read byte at add 11>");
        simulator.set_out_val("addr", "out", 11);
        simulator.clock();
        assert_eq!(simulator.cycle, 9);
        assert_eq!(simulator.get_input_val(out), 0x12.into());
    }
}
