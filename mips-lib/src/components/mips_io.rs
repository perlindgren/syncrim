use log::*;
use serde::{Deserialize, Serialize};
use std::{any::Any, cell::RefCell, collections::VecDeque};
use syncrim::{
    common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator},
    signal::SignalValue,
};

pub const IO_REGISTER_SELECT_IN_ID: &str = "io_reg_select_in";
pub const IO_DATA_IN_ID: &str = "io_data_in";
pub const IO_WRITE_ENABLE_IN: &str = "io_write_enable_in";
pub const IO_READ_ENABLE_IN: &str = "io_read_enable_in";

pub const IO_DATA_OUT_ID: &str = "io_data_out";
pub const IO_INTERRUPT_OUT_ID: &str = "io_interrupt_out";

/// # IO component
/// This compote provides IO functionality with output buffer and input buffer.
/// At reg select 0, the input control register is available, this register contains the following flags
/// - 0b01 this register denotes if there is input data available to be read *
/// - 0b10, interrupt control bit, when set and input is received the interrupt line will be set to 0x1
///
/// \* this bit is read only \
/// Reg select 1, is the input buffer, when read this will return the first u8 available,
/// it will also set input bit to 0 and interrupt to zero when the buffer becomes empty.
/// If empty the line output stays the same as before  \
/// Reg select 2, is the output buffer, data written to this ins truncated to a u8,
/// when using egui this is rendered as an utf8 string
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MipsIO {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) address_in: Input,
    pub(crate) data_in: Input,
    pub(crate) we_in: Input,
    pub(crate) re_in: Input,

    #[serde(skip)]
    pub data: RefCell<MipsIOData>,

    // used in egui to determine if the input/output window should show
    #[cfg(feature = "gui-egui")]
    #[serde(skip)]
    pub gui_show: RefCell<bool>,
}

#[derive(Debug, Clone)]
pub struct MipsIOData {
    pub interrupt: bool,
    pub input_control: u32,
    pub key_buff: VecDeque<u8>,
    pub out_buff: Vec<u8>,
    // pub history: TODO
}

impl Default for MipsIOData {
    fn default() -> Self {
        Self {
            interrupt: false,
            key_buff: Default::default(),
            input_control: 0,
            out_buff: Default::default(),
        }
    }
}

#[typetag::serde]
impl Component for MipsIO {
    fn to_(&self) {
        trace!("mips_io");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: IO_REGISTER_SELECT_IN_ID.to_string(),
                        input: self.address_in.clone(),
                    },
                    &InputPort {
                        port_id: IO_DATA_IN_ID.to_string(),
                        input: self.data_in.clone(),
                    },
                    &InputPort {
                        port_id: IO_WRITE_ENABLE_IN.to_string(),
                        input: self.we_in.clone(),
                    },
                    &InputPort {
                        port_id: IO_READ_ENABLE_IN.to_string(),
                        input: self.re_in.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![IO_DATA_OUT_ID, IO_INTERRUPT_OUT_ID],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            IO_REGISTER_SELECT_IN_ID => self.address_in = new_input,
            IO_DATA_IN_ID => self.data_in = new_input,
            IO_WRITE_ENABLE_IN => self.we_in = new_input,
            IO_READ_ENABLE_IN => self.we_in = new_input,
            _ => {}
        }
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // if write enable
        if simulator.get_input_value(&self.we_in) == SignalValue::Data(0x1) {
            // get the data component data ref
            let mut data = self.data.borrow_mut();
            // if data is valid
            if let SignalValue::Data(in_data) = simulator.get_input_value(&self.data_in) {
                // the register/address to write data at
                match simulator.get_input_value(&self.address_in) {
                    SignalValue::Data(0) => {
                        data.input_control = in_data & 0xFFFF_FFFE | data.input_control & 0x1
                    }
                    SignalValue::Data(2) => {
                        data.out_buff.push(in_data as u8);
                    }
                    // if the address dont exist in our IO part
                    SignalValue::Data(_) => todo!("bad address warning condition"),
                    // if signal is uninitialized dont care or unknown return error
                    _ => todo!("bad address error condition"),
                }
            } else {
                // trying to write a signal that isn't data
                todo!("return bad data error")
            }
        }

        // if read enable
        if simulator.get_input_value(&self.re_in) == SignalValue::Data(0x1) {
            // get component data ref
            let mut data = self.data.borrow_mut();
            // the register/address to read at
            match simulator.get_input_value(&self.address_in) {
                SignalValue::Data(0x0) => {
                    simulator.set_out_value(&self.id, IO_DATA_OUT_ID, data.input_control);
                }
                SignalValue::Data(0x1) => {
                    if let Some(key) = data.key_buff.pop_front() {
                        //set the output to the key
                        simulator.set_out_value(&self.id, IO_DATA_OUT_ID, key as u32);

                        //clear interrupt
                        data.interrupt = false;

                        // if we have read all the data clear the data bit
                        if data.key_buff.is_empty() {
                            data.input_control &= 0xFFFF_FFFE
                        }
                    }
                }
                _ => {}
            }
        }

        // set the interrupt signal
        simulator.set_out_value(&self.id, IO_INTERRUPT_OUT_ID, self.data.borrow().interrupt);

        Ok(())
    }

    fn reset(&self) {
        *self.data.borrow_mut() = MipsIOData {
            interrupt: false,
            input_control: 0,
            key_buff: VecDeque::default(),
            out_buff: vec![],
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MipsIO {
    pub fn new(
        id: &str,
        pos: (f32, f32),
        address_in: Input,
        data_in: Input,
        write_enable_in: Input,
        read_enable_in: Input,
    ) -> Self {
        MipsIO {
            id: id.to_string(),
            pos,
            address_in,
            data_in,
            we_in: write_enable_in,
            re_in: read_enable_in,
            data: Default::default(),
            #[cfg(feature = "gui-egui")]
            gui_show: RefCell::new(false),
        }
    }
}
