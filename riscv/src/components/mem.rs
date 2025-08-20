use log::*;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::ops::Range;
use std::{cell::RefCell, collections::BTreeMap, convert::TryFrom, rc::Rc};
use syncrim::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, SignalSigned, SignalUnsigned,
    SignalValue, Simulator,
};
//#[feature("gui-egui")]
//use egui_extras::TableBuilder;
pub const RV_MEM_DATA_I_ID: &str = "data_i";
pub const RV_MEM_ADDR_ID: &str = "addr";
pub const RV_MEM_CTRL_ID: &str = "ctrl";
pub const RV_MEM_SEXT_ID: &str = "sext";
pub const RV_MEM_SIZE_ID: &str = "size";
pub const RV_INTERRUPT_ID: &str = "interrupt";
pub const RV_MEM_DATA_O_ID: &str = "data_o";
#[derive(Serialize, Deserialize, Clone)]
pub struct RVMem {
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
    pub(crate) interrupt: Input,
    //  pub(crate) mem_int_addr: Input,

    // memory
    #[serde(skip)]
    pub memory: Memory,
    pub(crate) range: Range<u32>,
    // later history... tbd
    //
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

impl RVMem {
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
        interrupt: Input,
        memory: BTreeMap<usize, u8>,
        range: Range<u32>,
    ) -> Self {
        RVMem {
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
            interrupt,
            memory: Memory::new(memory.clone()),
            range,
            history: RefCell::new(vec![]),
            init_state: memory.clone(),
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
        interrupt: Input,
        range: Range<u32>,
    ) -> Rc<Self> {
        let mut mem = BTreeMap::new();
        //fill the defined memory range with zeroes
        for i in range.clone() {
            mem.insert(i as usize, 0u8);
        }
        Rc::new(RVMem::new(
            id, pos, width, height, big_endian, data, addr, ctrl, sext, size, interrupt, mem, range,
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
        interrupt: Input,
        memory: BTreeMap<usize, u8>,
        range: Range<u32>,
    ) -> Rc<Self> {
        Rc::new(RVMem::new(
            id, pos, width, height, big_endian, data, addr, ctrl, sext, size, interrupt, memory,
            range,
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
    ReadIsr,
}

// impl From<SignalValue> for MemCtrl {
//     fn from(value:SignalValue) -> Self {
//         CliError::ParseError(error)
//     }
// }

#[typetag::serde()]
impl Component for RVMem {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_(&self) {
        trace!("Mem");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: RV_MEM_DATA_I_ID.to_string(),
                        input: self.data.clone(),
                    },
                    &InputPort {
                        port_id: RV_MEM_ADDR_ID.to_string(),
                        input: self.addr.clone(),
                    },
                    &InputPort {
                        port_id: RV_MEM_CTRL_ID.to_string(),
                        input: self.ctrl.clone(),
                    },
                    &InputPort {
                        port_id: RV_MEM_SEXT_ID.to_string(),
                        input: self.sext.clone(),
                    },
                    &InputPort {
                        port_id: RV_MEM_SIZE_ID.to_string(),
                        input: self.size.clone(),
                    },
                    &InputPort {
                        port_id: RV_INTERRUPT_ID.to_string(),
                        input: self.interrupt.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec!["data_o", "err", "mmio_mux_ctl"],
            ),
        )
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        let mut history_entry = MemOp {
            data: None,
            addr: 0,
            size: 0,
        };
        let data = simulator.get_input_value(&self.data);
        let addr = simulator.get_input_value(&self.addr);
        let size = simulator.get_input_value(&self.size);
        let sign = simulator.get_input_value(&self.sext);
        let interrupt = simulator.get_input_value(&self.interrupt);
        // let mem_int_addr = simulator.get_input_value(&self.mem_int_addr);

        // match mem_int_addr {
        //     SignalValue::Data(addr) => {
        //         let value = self.memory.read(addr as usize, 4, false, self.big_endian);
        //         simulator.set_out_value(&self.id, "isr_addr", value);
        //     }
        //     _ => simulator.set_out_value(&self.id, "isr_addr", SignalValue::Unknown),
        // }

        match simulator.get_input_value(&self.ctrl) {
            SignalValue::Data(ctrl) => {
                let interrupt_occurred = SignalValue::Data(true as u32) == interrupt;

                let ctrl: MemCtrl = if interrupt_occurred {
                    MemCtrl::ReadIsr
                } else {
                    MemCtrl::try_from(ctrl as u8).unwrap()
                };

                match ctrl {
                    MemCtrl::ReadIsr => {
                        let addr: u32 = addr.try_into().unwrap();

                        //if not in mmio range

                        trace!("read isr {:?}", addr);
                        let value = self.memory.read(addr as usize, 4, false, self.big_endian);
                        simulator.set_out_value(&self.id, "data_o", value);
                        let error = self.memory.align(addr as usize, 4);
                        trace!("align {:?}", error);
                        simulator.set_out_value(&self.id, "err", error); // align
                        simulator.set_out_value(&self.id, "mmio_mux_ctl", 0);
                    }
                    MemCtrl::Read => {
                        let addr: u32 = addr.try_into().unwrap();
                        if !(0x1000..=0x500F).contains(&addr) {
                            //if not in mmio range
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
                            let error = self.memory.align(addr as usize, size as usize);
                            trace!("align {:?}", error);
                            simulator.set_out_value(&self.id, "err", error); // align
                            simulator.set_out_value(&self.id, "mmio_mux_ctl", 0);
                        } else {
                            simulator.set_out_value(&self.id, "mmio_mux_ctl", 1);
                        }
                    }
                    MemCtrl::Write => {
                        let addr: u32 = addr.try_into().unwrap();
                        if !(0x1000..=0x500F).contains(&addr) {
                            //if not in mmio range
                            let size: u32 = size.try_into().unwrap();
                            history_entry = MemOp {
                                data: match self.memory.read(
                                    addr as usize,
                                    size as usize,
                                    false,
                                    self.big_endian,
                                ) {
                                    SignalValue::Data(d) => Some(d as usize),
                                    _ => None,
                                },
                                addr: addr as usize,
                                size: size as u8,
                            };
                            trace!("write addr {:?} size {:?}", addr, size);
                            self.memory
                                .write(addr as usize, size as usize, self.big_endian, data);
                            let value = self.memory.align(addr as usize, size as usize);
                            trace!("align {:?}", value);
                            simulator.set_out_value(&self.id, "err", value); // align
                        }
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

        // for (idx, i) in self.memory.0.borrow().iter().enumerate() {
        //     if i.0 % 4 == 0 && idx < 40 {
        //         //only print 40 bytes so the trace isn't busy
        //         trace!(
        //             "0x{:08x} : 0x{:02x}{:02x}{:02x}{:02x}",
        //             i.0,
        //             self.memory.0.borrow().get(i.0).unwrap_or(&0u8),
        //             self.memory.0.borrow().get(&(i.0 + 1)).unwrap_or(&0u8),
        //             self.memory.0.borrow().get(&(i.0 + 2)).unwrap_or(&0u8),
        //             self.memory.0.borrow().get(&(i.0 + 3)).unwrap_or(&0u8),
        //         )
        //     }
        // }
        self.history.borrow_mut().push(history_entry);
        Ok(())
    }

    fn un_clock(&self, _: &Simulator) {
        let entry = self.history.borrow_mut().pop().unwrap();
        if let Some(d) = entry.data {
            self.memory.write(
                entry.addr,
                entry.size.into(),
                self.big_endian,
                SignalValue::Data(d as u32),
            )
        } //self.memory.write(, size, big_endian, data)
    }

    fn reset(&self) {
        //let mut mem = self.memory.0.borrow_mut();
        //let b = self.init_state.0.clone();
        self.memory.0.replace(self.init_state.clone());
        //mem = self.init_state.0.borrow_mut();
        //self.memory.0.swap(&*self.init_state.0.clone());
        self.history.swap(&RefCell::new(vec![]));
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
    use std::rc::Rc;
    use syncrim::common::ComponentStore;
    use syncrim::components::ProbeOut;

    #[test]
    fn test_mem_be() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("data")),
                Rc::new(ProbeOut::new("addr")),
                Rc::new(ProbeOut::new("ctrl")),
                Rc::new(ProbeOut::new("size")),
                Rc::new(ProbeOut::new("sign")),
                Rc::new(ProbeOut::new("interrupt")),
                Rc::new(RVMem {
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
                    sext: Input::new("sign", "out"),
                    //interrupt: Input::new("sign", "out"),
                    interrupt: Input::new("interrupt", "out"),

                    // memory
                    memory: Memory(Rc::new(RefCell::new(BTreeMap::new()))),
                    range: Range {
                        start: 0u32,
                        end: 1u32,
                    },
                    history: RefCell::new(vec![]),
                    init_state: BTreeMap::new(),
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

        println!("<setup for write 0xf to addr 4>");

        simulator.set_out_value("data", "out", 0xf0);
        simulator.set_out_value("addr", "out", 4);
        simulator.set_out_value("ctrl", "out", MemCtrl::Write as SignalUnsigned);
        simulator.set_out_value("size", "out", 1);
        simulator.set_out_value("interrupt", "out", false as u32);
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
        simulator.set_out_value("data", "out", 0x1234);
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
                Rc::new(ProbeOut::new("data")),
                Rc::new(ProbeOut::new("addr")),
                Rc::new(ProbeOut::new("ctrl")),
                Rc::new(ProbeOut::new("size")),
                Rc::new(ProbeOut::new("sign")),
                Rc::new(ProbeOut::new("interrupt")),
                Rc::new(RVMem {
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
                    sext: Input::new("sign", "out"),
                    interrupt: Input::new("interrupt", "out"),

                    // memory
                    memory: Memory(Rc::new(RefCell::new(BTreeMap::new()))),
                    // later history... tbd
                    range: Range {
                        start: 0u32,
                        end: 1u32,
                    },
                    history: RefCell::new(vec![]),
                    init_state: BTreeMap::new(),
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

        simulator.set_out_value("data", "out", 0xf0);
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
        simulator.set_out_value("data", "out", 0x1234);
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
