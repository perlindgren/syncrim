use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
#[cfg(feature = "gui-egui")]
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
pub const CP0_INSTRUCTIO_ADDRESS_IN: &str = "cp0_instruction_address_in";

// out 0: intaddr 0x80001000
// out 1: readout
// out 2: isInt
pub const CP0_INTADDR_OUT_ID: &str = "cp0_intaddr_out_id";
pub const CP0_REGISTER_OUT_ID: &str = "cp0_register_out_id";
pub const CP0_ISINT_OUT_ID: &str = "cp0_isint_out_id";

#[derive(Serialize, Deserialize, Clone)]
struct RegOp {
    pub addr: u8,
    pub data: u32, // might save whole signal in future
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
    pub registers: RefCell<[u32; 32]>, // all 32 registers, in the future, we might save the whole signal
    #[serde(skip)]
    history: RefCell<Vec<RegOp>>, // contains the value before it was modified used for unclock.
}

#[typetag::serde]
impl Component for CP0 {
    fn to_(&self) {
        trace!("alu");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, _id: &str, _pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        let arr: [u32; 32] = [0; 32];
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
            registers: RefCell::new(arr),
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
                        port_id: CP0_INSTRUCTIO_ADDRESS_IN.to_string(),
                        input: self.instruction_address_in.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![CP0_INTADDR_OUT_ID, CP0_REGISTER_OUT_ID, CP0_ISINT_OUT_ID],
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
            CP0_INSTRUCTIO_ADDRESS_IN => self.instruction_address_in = new_input,
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
        let interupt_address_in: u32 = simulator
            .get_input_value(&self.instruction_address_in)
            .try_into()
            .unwrap();

        let mut interrupt_occurd: u32 = 0;

        //save value to history before write, no need for {} as borrows is dropped after operation?
        self.history.borrow_mut().push(RegOp {
            addr: register_address as u8,
            data: *self.registers.borrow().get(register_address).unwrap(),
        });

        if syscall == 1 || io_interrupt == 1 || timer_interrupt == 1 {
            interrupt_occurd = 1;
            let mut regs = self.registers.borrow_mut();

            // Save the instruction address that caused the exception
            regs[EPC] = interupt_address_in;

            // set current state and interrupt
            let mut regs = self.registers.borrow_mut();
            let tmp = regs[SR] & 0xF;
            tmp << 2;
            regs[SR] &= 0xFFFFFFC0;
            regs[SR] = regs[SR] | tmp;

            // Set bits in ECR according to the interrupt type
            if timer_interrupt == 1 && ((regs[SR] & 0x401) == 0x401) {
                regs[ECR] = regs[ECR] & 0xFFFF0003 | 0x400;
            }
            if io_interrupt == 1 && ((regs[SR] & 0x801) == 0x801) {
                regs[ECR] = regs[ECR] & 0xFFFF0003 | 0x800;
            }
            if syscall == 1 {
                regs[ECR] = regs[ECR] & 0xFFFF0003 | 0x120;
            }
        }

        if rfe == 1 {
            let mut regs = self.registers.borrow_mut();
            let mut status = regs[SR] & 0x3c;
            status >>= 2;
            let tmp = (status & 0x30) | status;
            regs[SR] = (status & 0xFFFFFFC0) | tmp;
        }

        //bad code, follows same structure as syncsim
        // TODO: update so that it allows writing to all registers
        if write_enable_in == 1 {
            let mut regs = self.registers.borrow_mut();
            if register_address == 0x6000 {
                regs[SR] = input_data;
            } else if register_address == 6800 {
                regs[ECR] = input_data;
            } else if register_address == 7000 {
                regs[EPC] = input_data;
            }
        }

        let mut read_register = 0;
        if register_address == 0x6000 {
            read_register = SR;
        } else if register_address == 6800 {
            read_register = ECR;
        } else if register_address == 7000 {
            read_register = EPC;
        }

        // out 0: intaddr 0x80001000
        // out 1: readout
        // out 2: isInt
        simulator.set_out_value(&self.id, CP0_INTADDR_OUT_ID, 0x80001000);
        simulator.set_out_value(
            &self.id,
            CP0_REGISTER_OUT_ID,
            self.registers.borrow()[read_register],
        );
        simulator.set_out_value(&self.id, CP0_ISINT_OUT_ID, interrupt_occurd);
        Ok(())
    }

    fn un_clock(&self) {
        if let Some(last_op) = self.history.borrow_mut().pop() {
            let mut regs = self.registers.borrow_mut();
            // if regs[last_op.addr as usize] != last_op.data {
            //     *self.changed_register.borrow_mut() = last_op.addr as u32;
            // }
            regs[last_op.addr as usize] = last_op.data;
        }
    }

    fn reset(&self) {
        *self.registers.borrow_mut() = [0; 32];
        self.registers.borrow_mut()[SR] = 0x0000_0010;
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
        let mut arr: [u32; 32] = [0; 32];
        arr[SR] = 0x0000_0010;
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
            registers: RefCell::new(arr), // create 32 zeros
            history: RefCell::new(vec![]),
        }
    }

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

    pub fn get_cp0_register(&self, i: usize) -> u32 {
        self.registers.borrow()[i]
    }
}
