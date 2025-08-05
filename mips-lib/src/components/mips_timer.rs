// use std::fmt::Alignment;
use log::*;
use serde::{Deserialize, Serialize};
use std::{any::Any, cell::RefCell};
use syncrim::{
    common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator},
    signal::SignalValue,
};

pub const TIMER_ADDRESS_IN_ID: &str = "timer_address_in";
pub const TIMER_DATA_IN_ID: &str = "timer_data_in";
pub const TIMER_WRITE_ENABLE: &str = "timer_write_enable_in";

pub const TIMER_DATA_OUT_ID: &str = "timer_data_out";
pub const TIMER_INTERRUPT_OUT_ID: &str = "timer_interrupt_out";

const COUNTER_ENABLE: u8 = 0b0000_0001;
const OVERFLOW_IE: u8 = 0b0000_0010;
const OVERFLOW_FG: u8 = 0b0000_0100;
const COMPARE1_IE: u8 = 0b0000_1000;
const COMPARE1_FG: u8 = 0b0001_0000;
const COMPARE1_CR: u8 = 0b0010_0000;

#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct MipsTimer {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) address_in: Input,
    pub(crate) data_in: Input,
    pub(crate) we_in: Input,

    #[serde(skip)]
    pub data: RefCell<MipsTimerData>,
}

#[derive(Clone,Debug)]
pub struct MipsTimerData {
    // 1 = counter enabled, 2 = overflow_ie = 2, 4 overflow_FG,
    // 8 = compare1_IE, 16 = compare1_FG, 32 = compare1_CR
    pub flags: u8,
    pub counter: u32,
    pub compare: u32,
    pub divider: u32,
    pub div_counter: u32,
}
impl Default for MipsTimerData {
    fn default() -> Self {
        Self {
            flags: 0,
            counter: 0,
            compare: 0,
            divider: 16,
            div_counter: 0,
        }
    }
}

#[typetag::serde]
impl Component for MipsTimer {
    fn to_(&self) {
        trace!("mips_timer");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: TIMER_ADDRESS_IN_ID.to_string(),
                        input: self.address_in.clone(),
                    },
                    &InputPort {
                        port_id: TIMER_DATA_IN_ID.to_string(),
                        input: self.data_in.clone(),
                    },
                    &InputPort {
                        port_id: TIMER_WRITE_ENABLE.to_string(),
                        input: self.we_in.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![TIMER_DATA_OUT_ID, TIMER_INTERRUPT_OUT_ID],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            TIMER_ADDRESS_IN_ID => self.address_in = new_input,
            TIMER_DATA_IN_ID => self.data_in = new_input,
            TIMER_WRITE_ENABLE => self.we_in = new_input,
            _ => {}
        }
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // from lab 3a we have this information for regarding hardware addresses
        //
        // IO component address 0xFFFF_0000
        // +8 IO out
        //
        //
        // timer address 0xFFFF_0010
        // +0 Timer Control Register
        // +4 Timer Count Register
        // +8 Timer Compare Register
        let mut data = self.data.borrow_mut();
        if data.flags & COUNTER_ENABLE == COUNTER_ENABLE {
            // inc our dive counter so there is only one counter inc per divider amount of cycles
            data.div_counter = data.div_counter.wrapping_add(1);

            if data.div_counter >= data.divider {
                data.div_counter = 0;

                data.counter = data.counter.wrapping_add(1);

                // if overflow set the overflow_fg flag
                if data.counter == 0 {
                    data.flags |= OVERFLOW_FG
                }
                // if our compare matches
                if data.counter == data.compare {
                    data.flags |= COMPARE1_FG
                }

                // reset timer when compare is reached
                // +1 for syncsim compatibility
                if (data.counter == data.compare + 1) && data.flags & COMPARE1_CR == COMPARE1_CR {
                    data.counter = 0
                }
            }
        }

        // NOTE addresses are designed for mips there is the possibilty for
        // the addresses to be modified by the mmu so that different values
        // could be used so that the timer component becomes more generic
        let mut ret: Result<(), Condition> = Ok(());

        // Read write enable, if 0x1 write data, if 0x0 read data, other dont do anything
        match simulator.get_input_value(&self.we_in) {
            // write enable write data
            SignalValue::Data(0x1) => {
                // if data is valid, aka not undefined or unset
                if let SignalValue::Data(in_data) = simulator.get_input_value(&self.data_in) {
                    // set data according to address
                    match simulator.get_input_value(&self.address_in) {
                        SignalValue::Data(0xFFFF_0010) => data.flags = in_data as u8,
                        SignalValue::Data(0xFFFF_0014) => data.counter = in_data,
                        SignalValue::Data(0xFFFF_0018) => data.compare = in_data,
                        _ => ret = Err(Condition::Warning("invalid write address".to_string())),
                    }
                } else {
                    ret = Err(Condition::Error("not valid data to write".to_string()));
                };
            }
            // no write enable but WE line is still defined, read
            SignalValue::Data(_) => match simulator.get_input_value(&self.address_in) {
                SignalValue::Data(0xFFFF_0010) => {
                    simulator.set_out_value(&self.id, TIMER_DATA_OUT_ID, data.flags as u32)
                }
                SignalValue::Data(0xFFFF_0014) => {
                    simulator.set_out_value(&self.id, TIMER_DATA_OUT_ID, data.counter)
                }
                SignalValue::Data(0xFFFF_0018) => {
                    simulator.set_out_value(&self.id, TIMER_DATA_OUT_ID, data.compare)
                }
                _ => {}
            },
            _ => {}
        };


        // if flag overflow_ie is set and overflow occurred according to overflow_fg flag
        // or
        // if flag compare_ie is set and compare happened according to compare_fg flag
        // set overflow out signal
        simulator.set_out_value(
            &self.id,
            TIMER_INTERRUPT_OUT_ID,
            if data.flags & (OVERFLOW_FG | OVERFLOW_IE) == (OVERFLOW_FG | OVERFLOW_IE)
                || data.flags & (COMPARE1_FG | COMPARE1_IE) == (COMPARE1_FG | COMPARE1_IE)
            {
                1
            } else {
                0
            },
        );

        ret
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MipsTimer {
    pub fn new(
        id: &str,
        pos: (f32, f32),
        address_in: Input,
        data_in: Input,
        write_enable_in: Input,
    ) -> Self {
        MipsTimer {
            id: id.to_string(),
            pos,
            address_in,
            data_in: data_in,
            we_in: write_enable_in,
            data: RefCell::new(MipsTimerData {
                flags: 0,
                counter: 0,
                compare: 0,
                divider: 16,
                div_counter: 0,
            }),
        }
    }
}
