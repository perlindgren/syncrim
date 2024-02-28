use crate::components::mem::{MemCtrl, Memory};
use log::trace;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
#[cfg(feature = "gui-egui")]
use std::rc::Rc;
#[cfg(feature = "gui-egui")]
use syncrim::common::EguiComponent;
use syncrim::{
    common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator},
    signal::{SignalUnsigned, SignalValue},
};

use std::collections::HashMap;
pub const GPIO_CSR_BASE: u32 = 0x0;
pub const GPIO_MMIO_BASE: u32 = 0x6000_0000;
pub const GPIO_DATA_I_ID: &str = "data_i";
pub const GPIO_SIZE_I_ID: &str = "size_i";
pub const GPIO_WE_I_ID: &str = "we_i";
pub const GPIO_ADDR_I_ID: &str = "addr_i";
pub const GPIO_SE_I_ID: &str = "se_i";
pub const GPIO_CSR_D_ID: &str = "csr_d";
pub const GPIO_CSR_A_ID: &str = "csr_a";
pub const GPIO_CSR_CTL_ID: &str = "csr_ctl";

pub const GPIO_DATA_O_ID: &str = "data_o";
pub const GPIO_PIN_O_ID: &str = "pin_o";
//pub const GPIO_DATA_I_ID: &str = "";
//pub const GPIO_SIZE_I_ID: &str = "out";
pub const PIN_AMOUNT: i8 = 8;

pub const GPIO_HEIGHT: f32 = 50.0;
pub const GPIO_WIDTH: f32 = 250.0;

#[derive(Serialize, Deserialize)]
pub struct GPIO {
    pub height: f32,
    pub width: f32,
    pub id: String,
    pub pos: (f32, f32),

    // internal state
    #[serde(skip)]
    pub memory: Memory,
    #[serde(skip)]
    pub pins: Pins,
    #[serde(skip)]
    pub csrstore: GPIOCsrStore,

    pub data_i: Input,
    pub size_i: Input,
    pub we_i: Input,
    pub addr_i: Input,
    pub se_i: Input,
    pub csr_d: Input,
    pub csr_a: Input,
    pub csr_ctl: Input,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Pin {
    pub enabled: bool,
    pub interrupts: bool,
    pub is_input: bool,
    pub state: bool,
    pub id: u8,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pins(pub Rc<RefCell<Vec<Pin>>>);

impl Default for Pins {
    fn default() -> Pins {
        let mut v = vec![];
        for i in 0..PIN_AMOUNT {
            v.push(Pin {
                enabled: false,
                interrupts: false,
                is_input: false,
                state: false,
                id: i as u8,
            });
        }
        Self::new(v)
    }
}
impl Pins {
    pub fn new(v: Vec<Pin>) -> Self {
        Pins(Rc::new(RefCell::new(v)))
    }
}

pub struct GPIOCsrStore(Rc<RefCell<HashMap<usize, usize>>>);

impl Default for GPIOCsrStore {
    fn default() -> GPIOCsrStore {
        let mut h = HashMap::new();
        for i in ((0 + GPIO_CSR_BASE) as usize)..=((6 + GPIO_CSR_BASE) as usize) {
            h.insert(i, 0);
        }
        GPIOCsrStore(Rc::new(RefCell::new(h)))
    }
}

#[typetag::serde()]
impl Component for GPIO {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_(&self) {
        println!("GPIO");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy = Input::new("dummy", "out");
        Box::new(Rc::new(GPIO {
            height: GPIO_HEIGHT,
            width: GPIO_WIDTH,
            id: id.to_string(),
            pos: (pos.0, pos.1),
            csr_d: dummy.clone(),
            csr_a: dummy.clone(),
            csr_ctl: dummy.clone(),
            data_i: dummy.clone(),
            size_i: dummy.clone(),
            we_i: dummy.clone(),
            addr_i: dummy.clone(),
            se_i: dummy.clone(),
            csrstore: GPIOCsrStore::default(),
            pins: Pins::default(),
            memory: Memory::default(),
        }))
    }
    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        if target_port_id.as_str() == GPIO_DATA_I_ID {
            self.data_i = new_input;
        } else if target_port_id.as_str() == GPIO_ADDR_I_ID {
            self.addr_i = new_input;
        } else if target_port_id.as_str() == GPIO_SIZE_I_ID {
            self.size_i = new_input;
        } else if target_port_id.as_str() == GPIO_WE_I_ID {
            self.we_i = new_input;
        } else if target_port_id.as_str() == GPIO_SE_I_ID {
            self.se_i = new_input;
        } else if target_port_id.as_str() == GPIO_CSR_D_ID {
            self.csr_d = new_input;
        } else if target_port_id.as_str() == GPIO_CSR_A_ID {
            self.csr_a = new_input;
        } else if target_port_id.as_str() == GPIO_CSR_CTL_ID {
            self.csr_ctl = new_input;
        }
    }
    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: GPIO_DATA_I_ID.to_string(),
                        input: self.data_i.clone(),
                    },
                    &InputPort {
                        port_id: GPIO_ADDR_I_ID.to_string(),
                        input: self.addr_i.clone(),
                    },
                    &InputPort {
                        port_id: GPIO_SIZE_I_ID.to_string(),
                        input: self.size_i.clone(),
                    },
                    &InputPort {
                        port_id: GPIO_WE_I_ID.to_string(),
                        input: self.we_i.clone(),
                    },
                    &InputPort {
                        port_id: GPIO_SE_I_ID.to_string(),
                        input: self.se_i.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![
                    GPIO_DATA_O_ID,
                    "pin_o0",
                    "pin_o1",
                    "pin_o2",
                    "pin_o3",
                    "pin_o4",
                    "pin_o5",
                    "pin_o6",
                    "pin_o7",
                ],
            ),
        )
    }
    #[allow(non_snake_case)]
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        let data = simulator.get_input_value(&self.data_i);
        let addr = simulator.get_input_value(&self.addr_i);
        let size = simulator.get_input_value(&self.size_i);
        let sign = simulator.get_input_value(&self.se_i);
        let csr_data = simulator.get_input_value(&self.csr_d);
        let csr_addr = simulator.get_input_value(&self.csr_a);
        let csr_ctl = simulator.get_input_value(&self.csr_ctl);
        match csr_ctl {
            SignalValue::Data(ctl) => {
                let csr_addr: u32 = csr_addr.try_into().unwrap();
                let csr_data: u32 = csr_data.try_into().unwrap_or(0); // could be a read still
                let mut csrstore = self.csrstore.0.borrow_mut();
                let csr_ret = self.csr_op(&mut csrstore, ctl, csr_data, csr_addr);
                trace!(
                    "CSR TOUCH addr: {:x}",
                    (csr_addr - GPIO_CSR_BASE) * 4 + GPIO_MMIO_BASE
                );
                trace!("CSR addr: {:x}", csr_addr);
                self.handle_gpio_write((csr_addr - GPIO_CSR_BASE) * 4 + GPIO_MMIO_BASE);
            }
            _ => {}
        }
        match simulator.get_input_value(&self.we_i) {
            SignalValue::Data(ctrl) => {
                let addr: u32 = addr.try_into().unwrap_or(0);
                if (0x6000_0000..=0x6000_0018).contains(&addr) {
                    let ctrl = MemCtrl::try_from(ctrl as u8).unwrap();
                    match ctrl {
                        MemCtrl::Read => {
                            let size: u32 = size.try_into().unwrap();
                            let sign: u32 = sign.try_into().unwrap();
                            let value =
                                self.memory
                                    .read(addr as usize, size as usize, sign != 0, false);
                            simulator.set_out_value(&self.id, "data_o", value);
                        }
                        MemCtrl::Write => {
                            let size: u32 = size.try_into().unwrap();
                            //let data: u32 = size.try_into().unwrap();
                            self.memory.write(addr as usize, size as usize, false, data);
                            self.handle_gpio_write(addr);
                        }
                        _ => {
                            simulator.set_out_value(&self.id, "data_o", SignalValue::Unknown);
                            trace!("nothing for GPIO")
                        }
                    }
                }
            }
            _ => {
                trace!("ctrl uninit");
            }
        };
        for pin in &*self.pins.0.borrow() {
            let mut name = GPIO_PIN_O_ID.to_string();
            name.push_str(&format!("{}", pin.id));
            if pin.state {
                simulator.set_out_value(&self.id, &name, SignalValue::Data(1));
            } else {
                simulator.set_out_value(&self.id, &name, SignalValue::Data(0));
            }
        }
        Ok(())
    }
}

impl GPIO {
    fn handle_gpio_write(&self, addr: u32) {
        let rel_addr = addr - 0x6000_0000;
        let touched_indices = [
            (rel_addr - rel_addr % 4) / 4,
            (rel_addr + 4 - (rel_addr % 4)) / 4,
        ];
        let mut pins = self.pins.0.borrow_mut();
        trace!("touched indices {:?}", touched_indices);
        // enable
        if touched_indices.contains(&0) {
            let mut data: u32 = self
                .memory
                .read(0x6000_0000, 4, false, false)
                .try_into()
                .unwrap();
            for i in 0..PIN_AMOUNT {
                let mut pin = *pins.get(i as usize).unwrap();
                if data & 0b1 == 1 {
                    pin.enabled = true;
                    trace!("PIN {} ENABLED", i);
                } else {
                    pin.enabled = false;
                    trace!("PIN {} DISABLED", i);
                }
                let _ = std::mem::replace(&mut pins[i as usize], pin);

                data = data >> 1;
            }
        }
        // set input/output
        if touched_indices.contains(&1) {
            let mut data: u32 = self
                .memory
                .read(0x6000_0004, 4, false, false)
                .try_into()
                .unwrap();
            for i in 0..PIN_AMOUNT {
                let mut pin = *pins.get(i as usize).unwrap();
                if data & 0b1 == 1 {
                    pin.is_input = true;
                    trace!("PIN {} IS INPUT", i);
                } else {
                    pin.is_input = false;
                    trace!("PIN {} IS OUTPUT", i);
                }
                let _ = std::mem::replace(&mut pins[i as usize], pin);
                data = data >> 1;
            }
        }
        // enable interrupts
        if touched_indices.contains(&2) {}
        // set
        if touched_indices.contains(&3) {
            let mut data: u32 = self
                .memory
                .read(0x6000_000C, 4, false, false)
                .try_into()
                .unwrap();
            let mut state: u32 = self
                .memory
                .read(0x6000_0018, 4, false, false)
                .try_into()
                .unwrap();
            for i in 0..PIN_AMOUNT {
                let mut pin = *pins.get(i as usize).unwrap();
                if data & 0b1 == 1 {
                    if pin.enabled && !pin.is_input {
                        pin.state = true;
                        trace!("PIN {} SET HIGH", i);
                        state |= 1 << i;
                    }
                }
                data = data >> 1;
                self.memory.write(0x6000_000C, 4, false, 0.into());
                let _ = std::mem::replace(&mut pins[i as usize], pin);
            }
            self.memory.write(0x6000_0018, 4, false, state.into());
        }
        // clear
        if touched_indices.contains(&4) {
            let mut data: u32 = self
                .memory
                .read(0x6000_0010, 4, false, false)
                .try_into()
                .unwrap();
            let mut state: u32 = self
                .memory
                .read(0x6000_0018, 4, false, false)
                .try_into()
                .unwrap();
            for i in 0..PIN_AMOUNT {
                let mut pin = *pins.get(i as usize).unwrap();
                if data & 0b1 == 1 {
                    if !pin.is_input {
                        pin.state = false;
                        trace!("PIN {} CLEAR", i);
                        state &= !(1 << i);
                    }
                }
                data = data >> 1;
                self.memory.write(0x6000_0010, 4, false, 0.into());
                let _ = std::mem::replace(&mut pins[i as usize], pin);
            }
            trace!("CLEAR STATE: {}", state);
            self.memory.write(0x6000_0018, 4, false, state.into());
        }
        // toggle
        if touched_indices.contains(&5) {
            let mut data: u32 = self
                .memory
                .read(0x6000_0014, 4, false, false)
                .try_into()
                .unwrap();
            let mut state: u32 = self
                .memory
                .read(0x6000_0018, 4, false, false)
                .try_into()
                .unwrap();
            for i in 0..PIN_AMOUNT {
                let mut pin = *pins.get(i as usize).unwrap();
                if data & 0b1 == 1 {
                    if pin.enabled && !pin.is_input {
                        pin.state = !pin.state;
                        trace!("PIN {} TOGGLE", i);
                        if pin.state {
                            state |= 1 << i;
                        } else {
                            state &= !(1 << i);
                        }
                    }
                }
                data = data >> 1;
                self.memory.write(0x6000_0014, 4, false, 0.into());
                let _ = std::mem::replace(&mut pins[i as usize], pin);
            }
            self.memory.write(0x6000_0018, 4, false, state.into());
        }
        if touched_indices.contains(&6) {
            let mut data: u32 = self
                .memory
                .read(0x6000_0018, 4, false, false)
                .try_into()
                .unwrap();
            for i in 0..PIN_AMOUNT {
                let mut pin = *pins.get(i as usize).unwrap();
                if data & 0b1 == 1 {
                    if pin.enabled && !pin.is_input {
                        pin.state = true;
                        trace!("PIN {} state high", i);
                    }
                } else {
                    pin.state = false;
                    trace!("PIN {} state low", i);
                }
                data = data >> 1;
                let _ = std::mem::replace(&mut pins[i as usize], pin);
            }
        }
    }
    fn csr_op(
        &self,
        csrstore: &mut HashMap<usize, usize>,
        csr_ctl: SignalUnsigned,
        csr_data: SignalUnsigned,
        csr_addr: SignalUnsigned,
    ) -> SignalValue {
        let mut val = SignalValue::Unknown;
        let mut csr_data = csr_data.clone();
        match csr_ctl {
            0 => {}
            //write
            1 => {
                if csrstore.contains_key(&(csr_addr as usize)) {
                    val = self.memory.read(
                        ((csr_addr - GPIO_CSR_BASE) * 4 + GPIO_MMIO_BASE) as usize,
                        4,
                        false,
                        false,
                    );
                    csrstore.insert(csr_addr as usize, csr_data as usize);
                    self.memory.write(
                        ((csr_addr - GPIO_CSR_BASE) * 4 + GPIO_MMIO_BASE) as usize,
                        4,
                        false,
                        SignalValue::Data(csr_data),
                    );
                }
            }
            //set
            2 => {
                if csrstore.contains_key(&(csr_addr as usize)) {
                    trace!("csr set");
                    val = self.memory.read(
                        ((csr_addr - GPIO_CSR_BASE) * 4 + GPIO_MMIO_BASE) as usize,
                        4,
                        false,
                        false,
                    );
                    let val_i: usize = val.try_into().unwrap();
                    csrstore.insert(csr_addr as usize, (csr_data as usize) | val_i);
                    self.memory.write(
                        ((csr_addr - GPIO_CSR_BASE) * 4 + GPIO_MMIO_BASE) as usize,
                        4,
                        false,
                        SignalValue::Data((csr_data) | val_i as u32),
                    );
                }
            }
            //clear
            3 => {
                if csrstore.contains_key(&(csr_addr as usize)) {
                    trace!("csr clear");
                    val = self.memory.read(
                        ((csr_addr - GPIO_CSR_BASE) * 4 + GPIO_MMIO_BASE) as usize,
                        4,
                        false,
                        false,
                    );
                    let val_i: usize = val.try_into().unwrap();
                    csrstore.insert(csr_addr as usize, val_i & !(csr_data as usize));
                    self.memory.write(
                        ((csr_addr - GPIO_CSR_BASE) * 4 + GPIO_MMIO_BASE) as usize,
                        4,
                        false,
                        SignalValue::Data((val_i as u32) & !csr_data),
                    );
                }
            }
            _ => {}
        }
        val
    }
}
