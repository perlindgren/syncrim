#[cfg(feature = "gui-egui")]
use crate::common::EguiComponent;
use crate::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, SignalSigned, SignalUnsigned,
    SignalValue, Simulator,
};
use log::*;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::ops::Deref;
use std::ops::Range;
use std::{cell::RefCell, collections::BTreeMap, convert::TryFrom, rc::Rc};

pub const MEM_DATA_ID: &str = "data";
pub const MEM_ADDR_ID: &str = "addr";
pub const MEM_CTRL_ID: &str = "ctrl";
pub const MEM_SEXT_ID: &str = "sext";
pub const MEM_SIZE_ID: &str = "size";

pub const MEM_DATA_OUT_ID: &str = "data_o";
pub const MEM_ERR_OUT_ID: &str = "err";

#[derive(Serialize, Deserialize, Clone)]
pub struct Mem {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) width: f32,
    pub(crate) height: f32,

    // configuration
    pub big_endian: bool,

    // ports
    pub(crate) data: Input,
    pub(crate) addr: Input,
    pub(crate) ctrl: Input,
    pub(crate) sext: Input,
    pub(crate) size: Input,

    // memory
    pub(crate) memory: Memory,
    pub(crate) range: Range<u32>,
    // later history... tbd
}

impl Mem {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &str,
        pos: (f32, f32),
        width: f32,
        height: f32,
        big_endian: bool,
        data: Input,
        addr: Input,
        ctrl: Input,
        sext: Input,
        size: Input,
        memory: BTreeMap<usize, u8>,
        range: Range<u32>,
    ) -> Self {
        Mem {
            id: id.to_string(),
            pos,
            width,
            height,
            big_endian,
            data,
            addr,
            ctrl,
            sext,
            size,
            memory: Memory::new(memory),
            range,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn rc_new(
        id: &str,
        pos: (f32, f32),
        width: f32,
        height: f32,
        big_endian: bool,
        data: Input,
        addr: Input,
        ctrl: Input,
        sext: Input,
        size: Input,
        range: Range<u32>,
    ) -> Rc<Self> {
        let mut mem = BTreeMap::new();
        //fill the defined memory range with zeroes
        for i in range.clone() {
            mem.insert(i as usize, 0u8);
        }
        Rc::new(Mem::new(
            id, pos, width, height, big_endian, data, addr, ctrl, sext, size, mem, range,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn rc_new_from_bytes(
        id: &str,
        pos: (f32, f32),
        width: f32,
        height: f32,
        big_endian: bool,
        data: Input,
        addr: Input,
        ctrl: Input,
        sext: Input,
        size: Input,
        memory: BTreeMap<usize, u8>,
        range: Range<u32>,
    ) -> Rc<Self> {
        Rc::new(Mem::new(
            id, pos, width, height, big_endian, data, addr, ctrl, sext, size, memory, range,
        ))
    }
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

    /// is one if addres is unaligend, and zero if it is.
    /// # Example
    /// ```
    /// use syncrim::components::Memory;
    /// use syncrim::signal::SignalValue;
    ///
    /// let mem = Memory::default(); // creates a memory with only zeros
    ///
    /// let align_adrs = mem.align(0xa3f5, 4);
    /// assert_eq!(SignalValue::from(1), align_adrs)
    /// ```
    pub fn align(&self, addr: usize, size: usize) -> SignalValue {
        ((addr % size != 0) as SignalUnsigned).into()
    }

    /// This function reads 1, 2, 4 bytes from the memory,
    /// and converts them to a SignalValue using the a big_endian and sign arguments
    ///
    /// # Panics
    /// This function panics if the argument size is NOT 1, 2 or 4
    /// # TODO
    /// - maybe make size an enum?
    pub fn read(&self, addr: usize, size: usize, sign: bool, big_endian: bool) -> SignalValue {
        // Read amount of bytes determined by size, and add them them a vector
        let data: Vec<u8> = (0..size)
            .map(|i| *self.0.borrow().get(&(addr + i)).unwrap_or(&0))
            .collect();

        // why convert to a slice?
        let data = data.as_slice();

        trace!("{:x?}", data);

        match size {
            1 => {
                // Loading one byte
                if sign {
                    data[0] as i8 as SignalSigned as SignalUnsigned
                } else {
                    data[0] as SignalUnsigned
                }
            }
            2 => {
                // Loading half word, most stuff here is for tracing and debugging
                if sign {
                    if big_endian {
                        // sign big endian
                        trace!("read signed half word be");
                        let i_16 = i16::from_be_bytes(data.try_into().unwrap());
                        trace!("i_16 {:x?}", i_16);
                        let i_32 = i_16 as i32;
                        trace!("i_32 {:x?}", i_32);
                        i_32 as SignalUnsigned
                    } else {
                        // sign little endian
                        trace!("read signed half word le");
                        let i_16 = i16::from_le_bytes(data.try_into().unwrap());
                        trace!("i_16 {:x?}", i_16);
                        let i_32 = i_16 as i32;
                        trace!("i_32 {:x?}", i_32);
                        i_32 as SignalUnsigned
                    }
                } else if big_endian {
                    // unsigned big endian
                    trace!("read unsigned half word be");
                    let u_16 = u16::from_be_bytes(data.try_into().unwrap());
                    trace!("u_16 {:x?}", u_16);
                    let u_32 = u_16 as u32;
                    trace!("u_32 {:x?}", u_32);
                    u_32 as SignalUnsigned
                } else {
                    // unsigned little endian
                    trace!("read unsigned half word le");
                    let u_16 = u16::from_le_bytes(data.try_into().unwrap());
                    trace!("u_16 {:x?}", u_16);
                    let u_32 = u_16 as u32;
                    trace!("u_32 {:x?}", u_32);
                    u_32 as SignalUnsigned
                }
            }
            4 => {
                // Load full word
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
        .into() // Convert to SignalValue
    }

    /// This function writes 1, 2, 4 bytes from the memory,
    /// using the a big_endian arguments
    ///
    /// # Panics
    /// This function panics if the argument size is NOT 1, 2 or 4
    /// # TODO
    /// - maybe make size an enum?
    pub fn write(&self, addr: usize, size: usize, big_endian: bool, data: SignalValue) {
        let data: SignalUnsigned = data.try_into().unwrap();
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

#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)] // Unfortunately Rust does not allow Signal here, we need to cast manually
pub enum MemCtrl {
    None,
    Read,
    Write,
}

// impl From<SignalValue> for MemCtrl {
//     fn from(value:SignalValue) -> Self {
//         CliError::ParseError(error)
//     }
// }

#[typetag::serde()]
impl Component for Mem {
    fn to_(&self) {
        trace!("Mem");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(Mem {
            id: id.to_string(),
            pos: (pos.0, pos.1),
            width: 100.0,
            height: 50.0,
            big_endian: true,
            data: dummy_input.clone(),
            addr: dummy_input.clone(),
            ctrl: dummy_input.clone(),
            size: dummy_input.clone(),
            sext: dummy_input.clone(),
            range: Range {
                start: 0,
                end: 0x20,
            },
            memory: Memory::new(BTreeMap::new()),
        }))
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
                    &InputPort {
                        port_id: MEM_SEXT_ID.to_string(),
                        input: self.sext.clone(),
                    },
                    &InputPort {
                        port_id: MEM_SIZE_ID.to_string(),
                        input: self.size.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![MEM_DATA_OUT_ID, MEM_ERR_OUT_ID],
            ),
        )
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        let data = simulator.get_input_value(&self.data);
        let addr = simulator.get_input_value(&self.addr);
        let size = simulator.get_input_value(&self.size);
        let sign = simulator.get_input_value(&self.sext);

        match simulator.get_input_value(&self.ctrl) {
            SignalValue::Data(ctrl) => {
                let ctrl = MemCtrl::try_from(ctrl as u8).unwrap();
                match ctrl {
                    MemCtrl::Read => {
                        let addr: u32 = addr.try_into().unwrap();
                        let size: u32 = size.try_into().unwrap();
                        let sign: u32 = sign.try_into().unwrap();
                        trace!("read addr {:?} size {:?}", addr, size);
                        let value = self.memory.read(
                            addr as usize,
                            size as usize,
                            sign != 0,
                            self.big_endian,
                        );
                        simulator.set_out_value(&self.id, "data_o", value);
                        let value = self.memory.align(addr as usize, size as usize);
                        trace!("align {:?}", value);
                        simulator.set_out_value(&self.id, "err", value); // align
                    }
                    MemCtrl::Write => {
                        let addr: u32 = addr.try_into().unwrap();
                        let size: u32 = size.try_into().unwrap();
                        trace!("write addr {:?} size {:?}", addr, size);
                        self.memory
                            .write(addr as usize, size as usize, self.big_endian, data);
                        let value = self.memory.align(addr as usize, size as usize);
                        trace!("align {:?}", value);
                        simulator.set_out_value(&self.id, "err", value); // align
                    }
                    MemCtrl::None => {
                        trace!("no read/write");
                    }
                }
            }
            _ => {
                simulator.set_out_value(&self.id, "data_o", SignalValue::Unknown);
                simulator.set_out_value(&self.id, "err", SignalValue::Unknown); // align
            }
        }

        for (idx, i) in self.memory.0.borrow().iter().enumerate() {
            if i.0 % 4 == 0 && idx < 40 {
                //only print 40 bytes so the trace isn't busy
                trace!(
                    "0x{:08x} : 0x{:02x}{:02x}{:02x}{:02x}",
                    i.0,
                    self.memory.0.borrow().get(i.0).unwrap_or(&0u8),
                    self.memory.0.borrow().get(&(i.0 + 1)).unwrap_or(&0u8),
                    self.memory.0.borrow().get(&(i.0 + 2)).unwrap_or(&0u8),
                    self.memory.0.borrow().get(&(i.0 + 3)).unwrap_or(&0u8),
                )
            }
        }

        Ok(())
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            MEM_DATA_ID => self.data = new_input,
            MEM_ADDR_ID => self.addr = new_input,
            MEM_CTRL_ID => self.ctrl = new_input,
            MEM_SEXT_ID => self.sext = new_input,
            MEM_SIZE_ID => self.size = new_input,
            _ => (),
        }
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
                Rc::new(ProbeOut::new("data_o")),
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
                    data: Input::new("data_o", "out"),
                    addr: Input::new("addr", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    size: Input::new("size", "out"),
                    sext: Input::new("sign", "out"),

                    // memory
                    memory: Memory(Rc::new(RefCell::new(BTreeMap::new()))),
                    range: Range {
                        start: 0u32,
                        end: 1u32,
                    },
                }),
            ],
        };

        let mut simulator = Simulator::new(cs).unwrap();

        assert_eq!(simulator.cycle, 1);

        // outputs
        let out = &Input::new("mem", "data_o");
        let err = &Input::new("mem", "err");

        // reset
        assert_eq!(simulator.get_input_value(out), 0.into());
        assert_eq!(
            simulator.get_input_value(err),
            (false as SignalUnsigned).into()
        );

        println!("<setup for write 42 to addr 4>");

        simulator.set_out_value("data_o", "out", 0xf0);
        simulator.set_out_value("addr", "out", 4);
        simulator.set_out_value("ctrl", "out", MemCtrl::Write as SignalUnsigned);
        simulator.set_out_value("size", "out", 1);
        println!("sim_state {:?}", simulator.sim_state);

        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);

        assert_eq!(simulator.cycle, 2);
        assert_eq!(simulator.get_input_value(out), 0.into());
        assert_eq!(
            simulator.get_input_value(err),
            (false as SignalUnsigned).into()
        );

        println!("<setup for read byte from addr 4>");

        simulator.set_out_value("ctrl", "out", MemCtrl::Read as SignalUnsigned);
        simulator.set_out_value("size", "out", 1);

        simulator.clock();

        assert_eq!(simulator.cycle, 3);
        assert_eq!(simulator.get_input_value(out), 0xf0.into());
        assert_eq!(
            simulator.get_input_value(err),
            (false as SignalUnsigned).into()
        );

        println!("<setup for read byte from addr 4>");
        simulator.set_out_value("size", "out", 1);
        simulator.set_out_value("sign", "out", true);

        simulator.clock();
        assert_eq!(simulator.cycle, 4);
        assert_eq!(simulator.get_input_value(out), 0xffff_fff0.into());
        assert_eq!(
            simulator.get_input_value(err),
            (false as SignalUnsigned).into()
        );

        println!("<setup for read half-word from addr 4>");
        simulator.set_out_value("size", "out", 2);
        simulator.set_out_value("sign", "out", true as SignalUnsigned);

        simulator.clock();
        assert_eq!(simulator.cycle, 5);
        assert_eq!(simulator.get_input_value(out), 0xffff_f000.into());
        assert_eq!(
            simulator.get_input_value(err),
            (false as SignalUnsigned).into()
        );

        println!("<setup for read word from addr 4>");
        simulator.set_out_value("size", "out", 4);
        simulator.set_out_value("sign", "out", true);

        simulator.clock();
        assert_eq!(simulator.cycle, 6);
        assert_eq!(simulator.get_input_value(out), 0xf000_0000.into());
        assert_eq!(simulator.get_input_value(err), false.into());

        println!("<setup for read word from addr 5>");
        simulator.set_out_value("addr", "out", 5);

        simulator.clock();
        assert_eq!(simulator.cycle, 7);
        assert_eq!(simulator.get_input_value(err), true.into());

        println!("<setup for read word from addr 6>");
        simulator.set_out_value("addr", "out", 6);

        simulator.clock();
        assert_eq!(simulator.cycle, 8);
        assert_eq!(simulator.get_input_value(err), true.into());

        println!("<setup for read word from addr 7>");
        simulator.set_out_value("addr", "out", 7);

        simulator.clock();
        assert_eq!(simulator.cycle, 9);
        assert_eq!(simulator.get_input_value(err), true.into());

        println!("<setup for read word from addr 8>");
        simulator.set_out_value("addr", "out", 8);

        simulator.clock();
        assert_eq!(simulator.cycle, 10);
        assert_eq!(simulator.get_input_value(err), false.into());

        println!("<setup for read half-word from addr 9>");
        simulator.set_out_value("addr", "out", 9);
        simulator.set_out_value("size", "out", 2);
        simulator.clock();
        assert_eq!(simulator.cycle, 11);
        assert_eq!(simulator.get_input_value(err), true.into());

        println!("<setup for read half-word from addr 10>");
        simulator.set_out_value("addr", "out", 10);

        simulator.clock();
        assert_eq!(simulator.cycle, 12);
        assert_eq!(simulator.get_input_value(err), false.into());

        println!("<setup for write half-word at add 10>");
        simulator.set_out_value("addr", "out", 10);
        simulator.set_out_value("data_o", "out", 0x1234);
        simulator.set_out_value("ctrl", "out", MemCtrl::Write as SignalUnsigned);
        simulator.clock();
        assert_eq!(simulator.cycle, 13);
        assert_eq!(simulator.get_input_value(err), false.into());

        println!("<setup for read byte at add 10>");
        simulator.set_out_value("ctrl", "out", MemCtrl::Read as SignalUnsigned);
        simulator.set_out_value("size", "out", 1);
        simulator.clock();
        assert_eq!(simulator.cycle, 14);
        assert_eq!(simulator.get_input_value(out), 0x12.into());

        println!("<setup for read byte at add 11>");
        simulator.set_out_value("addr", "out", 11);
        simulator.clock();
        assert_eq!(simulator.cycle, 15);
        assert_eq!(simulator.get_input_value(out), 0x34.into());

        println!("test done")
    }

    #[test]
    fn test_mem_le() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("data_o")),
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
                    data: Input::new("data_o", "out"),
                    addr: Input::new("addr", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    size: Input::new("size", "out"),
                    sext: Input::new("sign", "out"),

                    // memory
                    memory: Memory(Rc::new(RefCell::new(BTreeMap::new()))),
                    // later history... tbd
                    range: Range {
                        start: 0u32,
                        end: 1u32,
                    },
                }),
            ],
        };

        let mut simulator = Simulator::new(cs).unwrap();

        assert_eq!(simulator.cycle, 1);

        // outputs
        let out = &Input::new("mem", "data_o");
        let err = &Input::new("mem", "err");

        // reset
        assert_eq!(simulator.get_input_value(out), 0.into());
        assert_eq!(simulator.get_input_value(err), false.into());

        // println!("<setup for write 42 to addr 4>");

        simulator.set_out_value("data_o", "out", 0xf0);
        simulator.set_out_value("addr", "out", 4);
        simulator.set_out_value("ctrl", "out", MemCtrl::Write as SignalUnsigned);
        simulator.set_out_value("size", "out", 1); // byte

        println!("sim_state {:?}", simulator.sim_state);

        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);

        assert_eq!(simulator.cycle, 2);
        assert_eq!(simulator.get_input_value(out), 0.into());
        assert_eq!(simulator.get_input_value(err), false.into());

        println!("<setup for read byte from addr 4>");

        simulator.set_out_value("ctrl", "out", MemCtrl::Read as SignalUnsigned);
        simulator.set_out_value("size", "out", 1);

        simulator.clock();

        assert_eq!(simulator.cycle, 3);
        assert_eq!(simulator.get_input_value(out), 0xf0.into());
        assert_eq!(simulator.get_input_value(err), false.into());

        println!("<setup for read byte from addr 4>");
        simulator.set_out_value("size", "out", 1);
        simulator.set_out_value("sign", "out", true);

        simulator.clock();
        assert_eq!(simulator.cycle, 4);
        assert_eq!(simulator.get_input_value(out), 0xffff_fff0.into());
        assert_eq!(simulator.get_input_value(err), false.into());

        println!("<setup for read half-word from addr 4>");
        simulator.set_out_value("size", "out", 2);
        simulator.set_out_value("sign", "out", true);

        simulator.clock();
        assert_eq!(simulator.cycle, 5);
        assert_eq!(simulator.get_input_value(out), 0x0000_00f0.into());
        assert_eq!(simulator.get_input_value(err), false.into());

        println!("<setup for read word from addr 4>");
        simulator.set_out_value("size", "out", 4);
        simulator.set_out_value("sign", "out", true);
        simulator.clock();
        assert_eq!(simulator.cycle, 6);
        assert_eq!(simulator.get_input_value(out), 0x0000_00f0.into());
        assert_eq!(simulator.get_input_value(err), false.into());

        println!("<setup for write half-word at add 10>");
        simulator.set_out_value("addr", "out", 10); // b
        simulator.set_out_value("data_o", "out", 0x1234);
        simulator.set_out_value("ctrl", "out", MemCtrl::Write as SignalUnsigned);
        simulator.set_out_value("size", "out", 2);

        simulator.clock();
        assert_eq!(simulator.cycle, 7);
        assert_eq!(simulator.get_input_value(err), false.into());

        println!("<setup for read byte at add 10>");
        simulator.set_out_value("ctrl", "out", MemCtrl::Read as SignalUnsigned);
        simulator.set_out_value("size", "out", 1);
        simulator.clock();
        assert_eq!(simulator.cycle, 8);
        assert_eq!(simulator.get_input_value(out), 0x34.into());

        println!("<setup for read byte at add 11>");
        simulator.set_out_value("addr", "out", 11);
        simulator.clock();
        assert_eq!(simulator.cycle, 9);
        assert_eq!(simulator.get_input_value(out), 0x12.into());
    }
}
