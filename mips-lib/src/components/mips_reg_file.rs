// use std::fmt::Alignment;
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
#[cfg(feature = "gui-egui")]
use syncrim::common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator};

pub mod reg_file_fields {
    pub const RS_ADDRESS_IN_ID: &str = "rs_address_in";
    pub const RT_ADDRESS_IN_ID: &str = "rt_address_in";
    pub const WRITE_ADDRESS_IN_ID: &str = "write_address_in";
    pub const WRITE_DATA_IN_ID: &str = "write_data_in";
    pub const WRITE_ENABLE_IN_ID: &str = "write_enable_in";

    pub const RT_VALUE_OUT_ID: &str = "rt_value_out";
    pub const RS_VALUE_OUT_ID: &str = "rs_value_out";
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegFile {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) rs_address_in: Input,
    pub(crate) rt_address_in: Input,
    pub(crate) write_address_in: Input,
    pub(crate) write_data_in: Input,
    pub(crate) write_enable_in: Input,

    #[serde(skip)]
    pub registers: RefCell<[u32; 32]>, // all 32 registers, in future, we might save the whole signal
    #[serde(skip)]
    history: RefCell<Vec<RegOp>>, // contains the value before it was modified used for unclock.

    //used for gui
    #[serde(skip)]
    pub show_reg_names: RefCell<bool>,

    #[serde(skip)]
    pub reg_format: RefCell<RegFormat>,
}
#[derive(Clone, Default, PartialEq, PartialOrd, Debug)]
pub enum RegFormat {
    #[default]
    Hex,
    Bin,
    DecSigned,
    DecUnsigned,
    UTF8BE,
    UTF8LE,
}

#[derive(Serialize, Deserialize, Clone)]
struct RegOp {
    pub addr: u8,
    pub data: u32, // might save whole signal in future
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
                        port_id: reg_file_fields::RS_ADDRESS_IN_ID.to_string(),
                        input: self.rs_address_in.clone(),
                    },
                    &InputPort {
                        port_id: reg_file_fields::RT_ADDRESS_IN_ID.to_string(),
                        input: self.rt_address_in.clone(),
                    },
                    &InputPort {
                        port_id: reg_file_fields::WRITE_ADDRESS_IN_ID.to_string(),
                        input: self.write_address_in.clone(),
                    },
                    &InputPort {
                        port_id: reg_file_fields::WRITE_DATA_IN_ID.to_string(),
                        input: self.write_data_in.clone(),
                    },
                    &InputPort {
                        port_id: reg_file_fields::WRITE_ENABLE_IN_ID.to_string(),
                        input: self.write_enable_in.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![
                    reg_file_fields::RS_VALUE_OUT_ID,
                    reg_file_fields::RT_VALUE_OUT_ID,
                ],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            reg_file_fields::RS_ADDRESS_IN_ID => self.rs_address_in = new_input,
            reg_file_fields::RT_ADDRESS_IN_ID => self.rt_address_in = new_input,
            reg_file_fields::WRITE_ADDRESS_IN_ID => self.write_address_in = new_input,
            reg_file_fields::WRITE_DATA_IN_ID => self.write_data_in = new_input,
            reg_file_fields::WRITE_ENABLE_IN_ID => self.write_enable_in = new_input,
            _ => {}
        }
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        //TODO check if inputs are invalid and return error
        // type of signal
        // address within bounds etc

        let rs_addr: usize = simulator
            .get_input_value(&self.rs_address_in)
            .try_into()
            .unwrap();
        let rt_addr: usize = simulator
            .get_input_value(&self.rt_address_in)
            .try_into()
            .unwrap();
        let w_addr: usize = simulator
            .get_input_value(&self.write_address_in)
            .try_into()
            .unwrap();
        let w_data: u32 = simulator
            .get_input_value(&self.write_data_in)
            .try_into()
            .unwrap();
        let w_enable: u32 = simulator
            .get_input_value(&self.write_enable_in)
            .try_into()
            .unwrap();

        //save value to history before write, no need for {} as borrows is dropped after operation?
        self.history.borrow_mut().push(RegOp {
            addr: w_addr as u8,
            data: self
                .registers
                .borrow()
                .get(w_addr as usize)
                .unwrap()
                .clone(),
        });

        // write data
        if w_enable == 1 && w_addr != 0 {
            self.registers.borrow_mut()[w_addr] = w_data;
        };

        // update out signals, no {} since self.registers are dropped at end of function
        let regs = self.registers.borrow();
        simulator.set_out_value(&self.id, reg_file_fields::RS_VALUE_OUT_ID, regs[rs_addr]);
        simulator.set_out_value(&self.id, reg_file_fields::RT_VALUE_OUT_ID, regs[rt_addr]);

        Ok(())
    }

    fn un_clock(&self) {
        if let Some(last_op) = self.history.borrow_mut().pop() {
            let mut regs = self.registers.borrow_mut();
            regs[last_op.addr as usize] = last_op.data;
        }
    }

    fn reset(&self) {
        *self.registers.borrow_mut() = [0; 32];
        self.registers.borrow_mut()[29] = 0x8000_0000;
        *self.history.borrow_mut() = vec![];
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl RegFile {
    pub fn new(
        id: &str,
        pos: (f32, f32),
        rs_address_in: Input,
        rt_address_in: Input,
        write_address_in: Input,
        write_data_in: Input,
        write_enable_in: Input,
    ) -> Self {
        let mut arr: [u32; 32] = [0; 32];
        arr[29] = 0x8000_0000;
        RegFile {
            id: id.to_string(),
            pos,
            rs_address_in,
            rt_address_in,
            write_address_in,
            write_data_in,
            write_enable_in,
            registers: RefCell::new(arr), // create 32 zeros, wit 29(stack pointer) at 0x8000_0000
            history: RefCell::new(vec![]),
            show_reg_names: RefCell::default(),
            reg_format: RefCell::default(),
        }
    }

    pub fn rc_new(
        id: &str,
        pos: (f32, f32),
        rs_address_in: Input,
        rt_address_in: Input,
        write_address_in: Input,
        write_data_in: Input,
        write_enable_in: Input,
    ) -> Rc<Self> {
        Rc::new(RegFile::new(
            id,
            pos,
            rs_address_in,
            rt_address_in,
            write_address_in,
            write_data_in,
            write_enable_in,
        ))
    }

    pub fn get_registers(&self, i: usize) -> u32 {
        self.registers.borrow()[i]
    }
}
