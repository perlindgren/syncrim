use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
#[cfg(feature = "gui-egui")]
use syncrim::common::EguiComponent;
use syncrim::common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator};

/// used to keep track of CP0 registers
pub const BPC: usize = 3; // Breakpoint Program Counter
pub const BDA: usize = 5; // Breakpoint Data Address
pub const TAR: usize = 6; // Target Address
pub const DCIC: usize = 7; // Debug and Cache Invalidate Control
pub const BADA: usize = 8; // Bad Address (read only)
pub const BDAM: usize = 9; // Breakpoint Data Address Mask
pub const BPCM: usize = 11; // Breakpoint Program Counter Mask
pub const SR: usize = 12; // Status
pub const ECR: usize = 13; // Cause
pub const EPC: usize = 14; // Exception Program Counter
pub const PRID: usize = 15; // Processor Revision Identifier

pub const CP0_WRITE_ENABLE_IN_ID: &str = "cp0_write_enable_in";
pub const CP0_REGISTER_ADDRESS_IN: &str = "cp0_register_address_in";
pub const CP0_DATA_IN: &str = "cp0_data_in";
pub const CP0_RFE_IN_ID: &str = "cp0_RFE_in";
pub const CP0_TIMER_INTERRUPT_IN_ID: &str = "cp0_timer_interrupt_in"; // update to timer and io
pub const CP0_IO_INTERRUPT_IN_ID: &str = "cp0_io_interrupt_in";
pub const CP0_SYSCALL_IN_ID: &str = "cp0_syscall_in";
pub const CP0_INSTRUCTION_ADDRESS_IN: &str = "cp0_instruction_address_in";

// out 0: int addr 0x80001000
// out 1: readout
// out 2: isInt
pub const CP0_INT_ADDR_OUT_ID: &str = "cp0_int_addr_out_id";
pub const CP0_REGISTER_OUT_ID: &str = "cp0_register_out_id";
pub const CP0_IS_INT_OUT_ID: &str = "cp0_is_int_out_id";

struct RegChangeFrom {
    cycle: usize,
    regs: Regs,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Regs {
    pub sr: u32,
    pub ecr: u32,
    pub epc: u32,
}
impl Default for Regs {
    fn default() -> Self {
        Self {
            sr: 0x0000_0010,
            ecr: 0,
            epc: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CP0 {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) write_enable: Input,
    pub(crate) register_address_in: Input,
    pub(crate) data_in: Input,
    pub(crate) rfe_in: Input,
    pub(crate) timer_interrupt_in: Input,
    pub(crate) io_interrupt_in: Input,
    pub(crate) syscall_in: Input,
    pub(crate) instruction_address_in: Input,

    #[serde(skip)]
    // SR, ECR, EPC
    pub registers: RefCell<Regs>, // all 32 registers, in the future, we might save the whole signal
    #[serde(skip)]
    history: RefCell<Vec<RegChangeFrom>>, // contains the value before it was modified used for unclock.
}

#[typetag::serde]
impl Component for CP0 {
    fn to_(&self) {
        trace!("alu");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, _id: &str, _pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(CP0 {
            id: "dummy".to_string(),
            pos: (0.0, 0.0),
            write_enable: dummy_input.clone(),
            register_address_in: dummy_input.clone(),
            data_in: dummy_input.clone(),
            rfe_in: dummy_input.clone(),
            timer_interrupt_in: dummy_input.clone(),
            io_interrupt_in: dummy_input.clone(),
            syscall_in: dummy_input.clone(),
            instruction_address_in: dummy_input.clone(),
            registers: RefCell::new(Regs {
                sr: 0,
                ecr: 0,
                epc: 0,
            }),
            history: RefCell::new(vec![]),
        }))
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: CP0_WRITE_ENABLE_IN_ID.to_string(),
                        input: self.write_enable.clone(),
                    },
                    &InputPort {
                        port_id: CP0_REGISTER_ADDRESS_IN.to_string(),
                        input: self.register_address_in.clone(),
                    },
                    &InputPort {
                        port_id: CP0_DATA_IN.to_string(),
                        input: self.data_in.clone(),
                    },
                    &InputPort {
                        port_id: CP0_RFE_IN_ID.to_string(),
                        input: self.rfe_in.clone(),
                    },
                    &InputPort {
                        port_id: CP0_TIMER_INTERRUPT_IN_ID.to_string(),
                        input: self.timer_interrupt_in.clone(),
                    },
                    &InputPort {
                        port_id: CP0_IO_INTERRUPT_IN_ID.to_string(),
                        input: self.io_interrupt_in.clone(),
                    },
                    &InputPort {
                        port_id: CP0_SYSCALL_IN_ID.to_string(),
                        input: self.syscall_in.clone(),
                    },
                    &InputPort {
                        port_id: CP0_INSTRUCTION_ADDRESS_IN.to_string(),
                        input: self.instruction_address_in.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![CP0_INT_ADDR_OUT_ID, CP0_REGISTER_OUT_ID, CP0_IS_INT_OUT_ID],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            CP0_WRITE_ENABLE_IN_ID => self.write_enable = new_input,
            CP0_REGISTER_ADDRESS_IN => self.register_address_in = new_input,
            CP0_DATA_IN => self.data_in = new_input,
            CP0_RFE_IN_ID => self.rfe_in = new_input,
            CP0_TIMER_INTERRUPT_IN_ID => self.timer_interrupt_in = new_input,
            CP0_IO_INTERRUPT_IN_ID => self.io_interrupt_in = new_input,
            CP0_SYSCALL_IN_ID => self.syscall_in = new_input,
            CP0_INSTRUCTION_ADDRESS_IN => self.instruction_address_in = new_input,
            _ => {}
        }
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input values
        // let mfc0: u32 = simulator.get_input_value(&self.mfc0_in).try_into().unwrap();
        // let mtc0: u32 = simulator.get_input_value(&self.mtc0_in).try_into().unwrap();
        let write_enable_in: u32 = simulator
            .get_input_value(&self.write_enable)
            .try_into()
            .unwrap();
        let register_address: usize = simulator
            .get_input_value(&self.register_address_in)
            .try_into()
            .unwrap();
        let input_data: u32 = simulator.get_input_value(&self.data_in).try_into().unwrap();
        let rfe: u32 = simulator.get_input_value(&self.rfe_in).try_into().unwrap();
        let timer_interrupt: u32 = simulator
            .get_input_value(&self.timer_interrupt_in)
            .try_into()
            .unwrap();
        let io_interrupt: u32 = simulator
            .get_input_value(&self.io_interrupt_in)
            .try_into()
            .unwrap();
        let syscall: u32 = simulator
            .get_input_value(&self.syscall_in)
            .try_into()
            .unwrap();
        let interrupt_address_in: u32 = simulator
            .get_input_value(&self.instruction_address_in)
            .try_into()
            .unwrap();

        let mut interrupt_occurred: u32 = 0;

        let regs_before = self.registers.borrow().clone();

        if (syscall == 1 || io_interrupt == 1 || timer_interrupt == 1)
            && self.registers.borrow().sr & 1 == 1
        {
            let mut regs = self.registers.borrow_mut();

            // Set bits in ECR according to the interrupt type
            if timer_interrupt == 1 && ((regs.sr & 0x400) == 0x400) {
                regs.epc = interrupt_address_in;
                // set current state and interrupt
                let tmp = (regs.sr & 0xF) << 2;
                regs.sr &= 0xFFFF_FFC0;
                regs.sr |= tmp;
                // enable interrupt
                interrupt_occurred = 1;
                // Set bits in ECR according to the interrupt type
                regs.ecr = regs.ecr & 0xFFFF0003 | 0x400;
            } else if io_interrupt == 1 && ((regs.sr & 0x800) == 0x800) {
                regs.epc = interrupt_address_in;
                // set current state and interrupt
                let tmp = (regs.sr & 0xF) << 2;
                regs.sr &= 0xFFFF_FFC0;
                regs.sr |= tmp;
                // enable interrupt
                interrupt_occurred = 1;
                // Set bits in ECR according to the interrupt type
                regs.ecr = regs.ecr & 0xFFFF0003 | 0x800;
            } else if syscall == 1 {
                regs.epc = interrupt_address_in;
                // set current state and interrupt
                let tmp = (regs.sr & 0xF) << 2;
                regs.sr &= 0xFFFF_FFC0;
                regs.sr |= tmp;
                // enable interrupt
                interrupt_occurred = 1;
                // Set bits in ECR according to the interrupt type
                regs.ecr = regs.ecr & 0xFFFF0003 | 0x120;
            }
        }

        if rfe == 1 {
            let mut regs = self.registers.borrow_mut();
            let mut status = regs.sr & 0x3c;
            status >>= 2;
            let tmp = (regs.sr & 0x30) | status;
            regs.sr = (regs.sr & 0xFFFF_FFC0) | tmp;
        }

        //bad code, follows same structure as syncsim
        // TODO: update so that it allows writing to all registers
        let mut out_data: u32 = 0;
        {
            let mut regs = self.registers.borrow_mut();
            if register_address == 0x6000 {
                out_data = regs.sr;
            } else if register_address == 0x6800 {
                out_data = regs.ecr;
            } else if register_address == 0x7000 {
                out_data = regs.epc;
            }

            if write_enable_in == 1 {
                if register_address == 0x6000 {
                    //
                    regs.sr = input_data;
                } else if register_address == 6800 {
                    regs.ecr = input_data;
                } else if register_address == 7000 {
                    regs.epc = input_data;
                }
            }
        }

        // if we changed any of the registers this cycle, add the change the before values
        if *self.registers.borrow() != regs_before {
            let reg_change = RegChangeFrom {
                cycle: simulator.cycle,
                regs: regs_before,
            };
            self.history.borrow_mut().push(reg_change);
        }

        // out 0: int addr 0x80001000
        // out 1: readout
        // out 2: isInt
        simulator.set_out_value(&self.id, CP0_INT_ADDR_OUT_ID, 0x80000080);
        simulator.set_out_value(&self.id, CP0_REGISTER_OUT_ID, out_data);
        simulator.set_out_value(&self.id, CP0_IS_INT_OUT_ID, interrupt_occurred);
        Ok(())
    }

    fn un_clock(&self, sim: &Simulator) {
        let mut history = self.history.borrow_mut();
        if let Some(last) = history.pop_if(|r| r.cycle == sim.cycle) {
            *self.registers.borrow_mut() = last.regs
        }
    }

    fn reset(&self) {
        *self.registers.borrow_mut() = Regs::default();
        *self.history.borrow_mut() = vec![];
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CP0 {
    pub fn new(
        id: &str,
        pos: (f32, f32),
        write_enable: Input,
        register_address_in: Input,
        data_in: Input,
        rfe_in: Input,
        timer_interrupt_in: Input,
        io_interrupt_in: Input,
        syscall_in: Input,
        instruction_address_in: Input,
    ) -> Self {
        CP0 {
            id: id.to_string(),
            pos,
            write_enable,
            register_address_in,
            data_in,
            rfe_in,
            timer_interrupt_in,
            io_interrupt_in,
            syscall_in,
            instruction_address_in,
            registers: RefCell::new(Regs::default()), // create 32 zeros
            history: RefCell::new(vec![]),
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn rc_new(
        id: &str,
        pos: (f32, f32),
        write_enable: Input,
        register_address_in: Input,
        data_in: Input,
        rfe_in: Input,
        timer_interrupt_in: Input,
        io_interrupt_in: Input,
        syscall_in: Input,
        instruction_address_in: Input,
    ) -> Rc<Self> {
        Rc::new(Self::new(
            id,
            pos,
            write_enable,
            register_address_in,
            data_in,
            rfe_in,
            timer_interrupt_in,
            io_interrupt_in,
            syscall_in,
            instruction_address_in,
        ))
    }
}
