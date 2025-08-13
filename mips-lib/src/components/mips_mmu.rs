// use std::fmt::Alignment;
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use syncrim::{
    common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator},
    signal::SignalValue,
};

pub const MMU_ADDRESS_IN_ID: &str = "mmu_address_signal_in";
pub const MMU_READ_ENABLE_IN: &str = "mmu_re_in"; //TODO make control unit emit read enable signal
pub const MMU_WRITE_ENABLE_IN: &str = "mmu_we_in";

pub const MMU_MEM_WE_OUT: &str = "mmu_mem_we_out";
pub const MMU_MEM_RE_OUT: &str = "mmu_mem_re_out";
pub const MMU_TIMER_WE_OUT: &str = "mmu_timer_we_out";
pub const MMU_TIMER_RE_OUT: &str = "mmu_timer_re_out";
pub const MMU_IO_WE_OUT: &str = "mmu_io_we_out";
pub const MMU_IO_RE_OUT: &str = "mmu_io_re_out";

pub const MMU_MEM_ADDRESS_OUT_ID: &str = "mmu_mem_address_out";

pub const MMU_TIMER_ADDRESS_OUT: &str = "mmu_timer_address_out";

pub const MMU_IO_REG_SEL_OUT: &str = "mmu_io_reg_select_out";

// TODO CP0 STUFF

pub const MMU_COMPONENT_SELECT_OUT_ID: &str = "component_select_out";

pub mod mmu_signals {
    pub const MMU_SELECT_IO_SRC: u32 = 0;
    pub const MMU_SELECT_TIMER_SRC: u32 = 1;
    pub const MMU_SELECT_CP0_SRC: u32 = 2;
    pub const MMU_SELECT_MEM_SRC: u32 = 3;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MipsMmu {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),

    pub(crate) we_in: Input,
    pub(crate) re_in: Input,
    pub(crate) address_in: Input,
}

#[typetag::serde]
impl Component for MipsMmu {
    fn to_(&self) {
        trace!("Mips_mmu");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: MMU_ADDRESS_IN_ID.to_string(),
                        input: self.address_in.clone(),
                    },
                    &InputPort {
                        port_id: MMU_READ_ENABLE_IN.to_string(),
                        input: self.we_in.clone(),
                    },
                    &InputPort {
                        port_id: MMU_WRITE_ENABLE_IN.to_string(),
                        input: self.re_in.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![
                    MMU_MEM_ADDRESS_OUT_ID,
                    MMU_MEM_WE_OUT,
                    MMU_MEM_RE_OUT,

                    MMU_TIMER_ADDRESS_OUT,
                    MMU_TIMER_WE_OUT,
                    MMU_TIMER_RE_OUT,
                    
                    MMU_IO_REG_SEL_OUT,
                    MMU_IO_WE_OUT,
                    MMU_IO_RE_OUT,

                    MMU_COMPONENT_SELECT_OUT_ID,
                ],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            MMU_ADDRESS_IN_ID => self.address_in = new_input,
            MMU_READ_ENABLE_IN => self.re_in = new_input,
            MMU_WRITE_ENABLE_IN => self.we_in = new_input,
            _ => {}
        }
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // set default output values
        // No reads or writes to any component 
        simulator.set_out_value(&self.id, MMU_MEM_WE_OUT, SignalValue::Data(0));
        simulator.set_out_value(&self.id, MMU_MEM_RE_OUT, SignalValue::Data(0));
        simulator.set_out_value(&self.id, MMU_TIMER_WE_OUT, SignalValue::Data(0));
        simulator.set_out_value(&self.id, MMU_IO_WE_OUT, SignalValue::Data(0));
        simulator.set_out_value(&self.id, MMU_IO_RE_OUT, SignalValue::Data(0));
        // as well as no specific addresses
        simulator.set_out_value(&self.id, MMU_MEM_ADDRESS_OUT_ID, SignalValue::DontCare);
        simulator.set_out_value(&self.id, MMU_TIMER_ADDRESS_OUT, SignalValue::DontCare);
        simulator.set_out_value(&self.id, MMU_IO_REG_SEL_OUT, SignalValue::DontCare);

        // shorthand to pass signal trough the mmu component
        macro_rules! pass_signal {
            ($input:expr, $field:expr) => {
                simulator.set_out_value(&self.id, $field, simulator.get_input_value($input))
            };
        }


        // handle different addresses
        match simulator.get_input_value(&self.address_in) {
            // in range of the IO component
            SignalValue::Data(adrs) if (0xffff_0000..=0xffff_0008).contains(&adrs) => {
                // the io component doest work on addresses but have a register select input
                // so here we convert that address into a register select value
                simulator.set_out_value(
                    &self.id,
                    MMU_IO_REG_SEL_OUT,
                    match adrs {
                        0xffff_0000 => SignalValue::Data(0),
                        0xffff_0004 => SignalValue::Data(1),
                        0xffff_0008 => SignalValue::Data(2),
                        _ => SignalValue::Unknown,
                    },
                );
                // passthrough the read enable signal and the write enable signal
                pass_signal!(&self.we_in, MMU_IO_WE_OUT);
                pass_signal!(&self.re_in, MMU_IO_RE_OUT);

                // set mux to IO component
                simulator.set_out_value(
                    &self.id,
                    MMU_COMPONENT_SELECT_OUT_ID,
                    mmu_signals::MMU_SELECT_IO_SRC,
                );
            }
            // timer component
            SignalValue::Data(0xffff_0010..=0xffff_0018) => {
                pass_signal!(&self.address_in, MMU_TIMER_ADDRESS_OUT);
                pass_signal!(&self.we_in, MMU_TIMER_WE_OUT);
                pass_signal!(&self.re_in, MMU_TIMER_RE_OUT);
                // set mux to timer
                simulator.set_out_value(
                    &self.id,
                    MMU_COMPONENT_SELECT_OUT_ID,
                    mmu_signals::MMU_SELECT_TIMER_SRC,
                );
            }
            // if there is no special address send signal to data mem
            SignalValue::Data(_) => {
                pass_signal!(&self.address_in, MMU_MEM_ADDRESS_OUT_ID);
                pass_signal!(&self.we_in, MMU_MEM_WE_OUT);
                pass_signal!(&self.re_in, MMU_MEM_RE_OUT);
                // set mux to memory
                simulator.set_out_value(
                    &self.id,
                    MMU_COMPONENT_SELECT_OUT_ID,
                    mmu_signals::MMU_SELECT_MEM_SRC,
                );
            }
            _ => return Err(Condition::Error("Address is not defined".into())),
        };
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MipsMmu {
    pub fn new(
        id: &str,
        pos: (f32, f32),
        address_in: Input,
        write_enable_in: Input,
        read_enable_in: Input,
    ) -> Self {
        MipsMmu {
            id: id.to_string(),
            pos,
            address_in,
            we_in: write_enable_in,
            re_in: read_enable_in,
        }
    }
}
