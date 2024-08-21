// use std::fmt::Alignment;
#[cfg(feature = "gui-egui")]
use crate::common::EguiComponent;
use crate::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, SignalValue, Simulator,
};
use log::trace;
use serde::{Deserialize, Serialize};
use std::{any::Any, rc::Rc};

use super::alu_op; // values used in communication to the alu
use super::data_op; // values used in communication with the data memory
/// The input and output felid ids for the control unit
pub mod cntr_field {

    pub const INSTR_IN: &str = "control_unit_instr_in";

    /// const REG_DEST_RT: u32 = 0;
    /// const REG_DEST_RD: u32 = 1;
    /// const REG_DEST_31: u32 = 2;
    pub const REG_DEST_OUT: &str = "reg_dest";

    /// 0 or 1
    pub const REG_WRITE_ENABLE_OUT: &str = "reg_write_enable";

    /// const WRITE_REG_SRC_ALU:u32 = 0;
    /// const WRITE_REG_SRC_MEM:u32 = 1;
    pub const REG_WRITE_SRC_OUT: &str = "reg_write_src";

    /// se module full_adder:alu_op
    pub const ALU_OP_OUT: &str = "alu_op";

    /// const ALU_SRC_A_OP:u32 = 0;
    /// const ALU_SRC_A_RS:u32 = 1;
    /// const ALU_SRC_A_ZERO:u32 = 3;
    pub const ALU_SRC_A_OUT: &str = "alu_src_a";

    /// const ALU_SRC_B_RT:u32 = 0;
    /// const ALU_SRC_B_PC:u32 = 1;
    /// const ALU_SRC_B_IMM:u32 = 2;
    pub const ALU_SRC_B_OUT: &str = "alu_src_b";

    // const EXTEND_ZERO:u32 = 0;
    // const EXTEND_SIGNED:u32 = 1;
    pub const EXTEND_SELECT_OUT: &str = "extend_select";

    // 0 or 1
    pub const MEM_WRITE_ENABLE_OUT: &str = "mem_write_enable";

    // 0 or 1, used for co-processor address stuff
    pub const BRANCH_INTERRUPT_OUT: &str = "branch_interrupt";

    // pub const CP0_MFC0 = 0;
    // pub const CP0_MTC0 = 1;
    // pub const CP0_RFE = 2;
    // pub const CP0_SYSCALL = 3;
    pub const CP0_OUT: &str = "cp0_out";

    pub const MMU_OUT: &str = "mmu_out";

    //TODO
    // Opcode is passed to branch unit wich is responsible to controll branch logic
    // pub const BRANCH_TYPE_OUT: &str = "branch";

    //TODO
    // NOTE no mem mode, decided to pass opcode to data mem instead,
    // might change when LWL/LWR is implemented along with the load/store controller
    pub const MEM_MODE_OUT: &str = "mem_mode";
}

const NOP: u32 = 0;
const OP_0: u32 = 0;

const FUNCT_SLL: u32 = 0;
const FUNCT_SRL: u32 = 0b00_0010;
const FUNCT_SRA: u32 = 0b00_0011;
const FUNCT_SLLV: u32 = 0b00_0100;
const FUNCT_SRLV: u32 = 0b00_0110;
const FUNCT_SRAV: u32 = 0b00_111;
const FUNCT_JR: u32 = 0b00_1000;
const FUNCT_JALR: u32 = 0b00_1001;
const SYSCALL: u32 = 0b00_1100;
const FUNCT_ADD: u32 = 0b10_0000;
const FUNCT_ADDU: u32 = 0b10_0001;
const FUNCT_SUB: u32 = 0b10_0010;
const FUNCT_SUBU: u32 = 0b10_0011;
const FUNCT_AND: u32 = 0b10_0100;
const FUNCT_OR: u32 = 0b10_0101;
const FUNCT_XOR: u32 = 0b10_0110;
const FUNCT_NOR: u32 = 0b10_0111;
const FUNCT_SLT: u32 = 0b10_1010;
const FUNCT_SLTU: u32 = 0b10_1011;

const OP_1: u32 = 1;

const B_FUNCT_BLTZ: u32 = 0;
const B_FUNCT_BGEZ: u32 = 1;
const B_FUNCT_BLTZAL: u32 = 0b1_0000;
const B_FUNCT_BGEZAL: u32 = 0b1_0001;

const OP_J: u32 = 0b00_0010;
const OP_JAL: u32 = 0b00_0011;
const OP_BEQ: u32 = 0b00_0100;
const OP_BNE: u32 = 0b00_0101;
const OP_BLEZ: u32 = 0b00_0110;
const OP_BGTZ: u32 = 0b00_0111;

const OP_ADDI: u32 = 0b00_1000;
const OP_ADDIU: u32 = 0b00_1001;
const OP_SLTI: u32 = 0b00_1010;
const OP_SLTIU: u32 = 0b00_1011;
const OP_ANDI: u32 = 0b00_1100;
const OP_ORI: u32 = 0b00_1101;
const OP_XORI: u32 = 0b00_1110;
const OP_LUI: u32 = 0b00_1111;

const OP_CP0: u32 = 0b01_0000;
const CP0_FUNCT_MFC0: u32 = 0;
const CP0_FUNCT_MTF0: u32 = 0b0_0100;
const CP0_FUNCT_SPECIAL: u32 = 0b1_0000;
const CP0_FUNCT_SPECIAL_: u32 = 0b1_0000;

const OP_LB: u32 = 0b10_0000;
const OP_LH: u32 = 0b10_0001;
const OP_LWL: u32 = 0b10_0010;
const OP_LW: u32 = 0b10_0011;
const OP_LBU: u32 = 0b10_0100;
const OP_LHU: u32 = 0b10_0101;
const OP_LWR: u32 = 0b10_0110;

const OP_SB: u32 = 0b10_1000;
const OP_SH: u32 = 0b10_1001;
const OP_SWL: u32 = 0b10_1010;
const OP_SW: u32 = 0b10_1011;
const OP_SWR: u32 = 0b10_1110;

/// module used to get what u32 represent. Used for communication between components
pub mod cntr_unit_signals {
    pub const REG_DEST_RT: u32 = 0;
    pub const REG_DEST_RD: u32 = 1;
    pub const REG_DEST_31: u32 = 2;

    pub const REG_WRITE_DISABLE: u32 = 0;
    pub const REG_WRITE_ENABLE: u32 = 1;

    pub const MEM_WRITE_DISABLE: u32 = 0;
    pub const MEM_WRITE_ENABLE: u32 = 1;

    pub const ALU_SRC_A_SHAMT: u32 = 0;
    pub const ALU_SRC_A_RS: u32 = 1;
    pub const ALU_SRC_A_ZERO: u32 = 2;

    pub const ALU_SRC_B_RT: u32 = 0;
    pub const ALU_SRC_B_PC: u32 = 1;
    pub const ALU_SRC_B_IMM: u32 = 2;

    pub const WRITE_REG_SRC_ALU: u32 = 0;
    pub const WRITE_REG_SRC_MEM: u32 = 1;

    pub const NO_BRANCH_INTERRUPT: u32 = 0;
    pub const BRANCH_INTERRUPT: u32 = 1;

    pub const EXTEND_ZERO: u32 = 0;
    pub const EXTEND_SIGNED: u32 = 1;

    pub const CP0_MFC0: u32 = 0;
    pub const CP0_MTC0: u32 = 1;
    pub const CP0_RFE: u32 = 2;
    pub const CP0_SYSCALL: u32 = 3;

    pub const MMU_NORMAL: u32 = 0;
    pub const MMU_CP0: u32 = 1;
    pub const MMU_NOP: u32 = 2;

    // Note, it was decided to pass opcode to data mem to handle load
    // and store instructions there
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ControlUnit {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) a_in: Input,
}
#[typetag::serde]
impl Component for ControlUnit {
    fn to_(&self) {
        trace!("control_unit");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, _id: &str, _pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(ControlUnit {
            id: "dummy".to_string(),
            pos: (0.0, 0.0),
            a_in: dummy_input.clone(),
        }))
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![&InputPort {
                    port_id: cntr_field::INSTR_IN.to_string(),
                    input: self.a_in.clone(),
                }],
                OutputType::Combinatorial,
                vec![
                    cntr_field::REG_DEST_OUT,
                    cntr_field::REG_WRITE_ENABLE_OUT,
                    cntr_field::REG_WRITE_SRC_OUT,
                    cntr_field::ALU_OP_OUT,
                    cntr_field::ALU_SRC_A_OUT,
                    cntr_field::ALU_SRC_B_OUT,
                    cntr_field::EXTEND_SELECT_OUT,
                    cntr_field::MEM_WRITE_ENABLE_OUT,
                    cntr_field::BRANCH_INTERRUPT_OUT,
                    cntr_field::CP0_OUT,
                    cntr_field::MMU_OUT,
                    cntr_field::MEM_MODE_OUT,
                ],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            cntr_field::INSTR_IN => self.a_in = new_input,
            _ => {}
        }
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        let instr_in: u32 = simulator.get_input_value(&self.a_in).try_into().unwrap();

        let op_code: u32 = (&instr_in >> 26) & 0x0000_003f;

        /// Sets the given field to the given value
        macro_rules! set {
            ($field:expr, $signal_val:expr) => {
                simulator.set_out_value(&self.id, $field, SignalValue::Data($signal_val))
            };
        }

        /// sets the relevant fields for an i instruction
        /// OP rt rs imm
        /// ALU_OP and EXTEND ned to be set separately
        macro_rules! set_i_instr {
            () => {
                // set target reg to be RT and read from alu
                set!(cntr_field::REG_DEST_OUT, cntr_unit_signals::REG_DEST_RT);
                set!(
                    cntr_field::REG_WRITE_ENABLE_OUT,
                    cntr_unit_signals::REG_WRITE_ENABLE
                );
                set!(
                    cntr_field::REG_WRITE_SRC_OUT,
                    cntr_unit_signals::WRITE_REG_SRC_ALU
                );

                // set alu src to be RS and imm
                set!(cntr_field::ALU_SRC_A_OUT, cntr_unit_signals::ALU_SRC_A_RS);
                set!(cntr_field::ALU_SRC_B_OUT, cntr_unit_signals::ALU_SRC_B_IMM);
            };
        }

        /// sets the relevant fields for an load operation
        /// reg_src = mem
        /// reg_dest = rt
        /// addu rs (imm sign extended)
        /// MEM MODE needs to be set separately
        macro_rules! set_load_instr {
            () => {
                // set target reg to be RT and read from mem
                set!(
                    cntr_field::REG_WRITE_ENABLE_OUT,
                    cntr_unit_signals::REG_WRITE_ENABLE
                );
                set!(cntr_field::REG_DEST_OUT, cntr_unit_signals::REG_DEST_RT);
                set!(
                    cntr_field::REG_WRITE_SRC_OUT,
                    cntr_unit_signals::WRITE_REG_SRC_MEM
                );

                // set alu to be addu with rs and signed imm
                set!(cntr_field::ALU_OP_OUT, alu_op::ADDU);
                set!(cntr_field::ALU_SRC_A_OUT, cntr_unit_signals::ALU_SRC_A_RS);
                set!(cntr_field::ALU_SRC_B_OUT, cntr_unit_signals::ALU_SRC_B_IMM);
                set!(
                    cntr_field::EXTEND_SELECT_OUT,
                    cntr_unit_signals::EXTEND_SIGNED
                );
            };
        }
        macro_rules! set_store_instr {
            () => {
                // SET reg_write to disabled nad mem write to enable
                set!(
                    cntr_field::REG_WRITE_ENABLE_OUT,
                    cntr_unit_signals::REG_WRITE_DISABLE
                );
                set!(
                    cntr_field::MEM_WRITE_ENABLE_OUT,
                    cntr_unit_signals::MEM_WRITE_ENABLE
                );

                // set alu to be addu with rs and signed imm
                set!(cntr_field::ALU_OP_OUT, alu_op::ADDU);
                set!(cntr_field::ALU_SRC_A_OUT, cntr_unit_signals::ALU_SRC_A_RS);
                set!(cntr_field::ALU_SRC_B_OUT, cntr_unit_signals::ALU_SRC_B_IMM);
                set!(
                    cntr_field::EXTEND_SELECT_OUT,
                    cntr_unit_signals::EXTEND_SIGNED
                );
            };
        }

        set!(
            cntr_field::REG_WRITE_ENABLE_OUT,
            cntr_unit_signals::REG_WRITE_DISABLE
        );
        set!(
            cntr_field::MEM_WRITE_ENABLE_OUT,
            cntr_unit_signals::MEM_WRITE_DISABLE
        );
        set!(
            cntr_field::BRANCH_INTERRUPT_OUT,
            cntr_unit_signals::NO_BRANCH_INTERRUPT
        );
        set!(cntr_field::MEM_MODE_OUT, data_op::NO_OP);
        //TODO an idea would be to init all variables
        // let alu_src_a : Signal;
        // this would make the compiler force us to populate all paths so to not let any signal be undefined
        // It would be more code, but would hopefully be more "secure" as it would stop us from forgetting a value
        // another idea is to set all signals to uninitialized or dont care for better debugging

        // match the opcode
        match op_code {
            OP_0 => {
                let funct: u32 = &instr_in & 0x0000_003f;

                set!(
                    cntr_field::REG_WRITE_ENABLE_OUT,
                    cntr_unit_signals::REG_WRITE_ENABLE
                ); // overwritten by JR and SYSCALL to disabled

                set!(cntr_field::REG_DEST_OUT, cntr_unit_signals::REG_DEST_RD); // overwritten by JALR to REG_DEST_31
                set!(cntr_field::ALU_SRC_A_OUT, cntr_unit_signals::ALU_SRC_A_RS); // overwritten by JALR, SRA, SRL to shamt or zero
                set!(cntr_field::ALU_SRC_B_OUT, cntr_unit_signals::ALU_SRC_B_RT); // overwritten by JALR to PC
                set!(
                    cntr_field::REG_WRITE_SRC_OUT,
                    cntr_unit_signals::WRITE_REG_SRC_ALU
                );
                set!(
                    cntr_field::BRANCH_INTERRUPT_OUT,
                    cntr_unit_signals::NO_BRANCH_INTERRUPT
                ); // overiden by jr and jalr

                match funct {
                    FUNCT_SLL => {
                        set!(
                            cntr_field::ALU_SRC_A_OUT,
                            cntr_unit_signals::ALU_SRC_A_SHAMT
                        );
                        set!(cntr_field::ALU_OP_OUT, alu_op::SLL);
                        Ok(())
                    }
                    FUNCT_SRL => {
                        set!(
                            cntr_field::ALU_SRC_A_OUT,
                            cntr_unit_signals::ALU_SRC_A_SHAMT
                        );
                        set!(cntr_field::ALU_OP_OUT, alu_op::SRL);
                        Ok(())
                    }
                    FUNCT_SRA => {
                        set!(
                            cntr_field::ALU_SRC_A_OUT,
                            cntr_unit_signals::ALU_SRC_A_SHAMT
                        );
                        set!(cntr_field::ALU_OP_OUT, alu_op::SRA);
                        Ok(())
                    }
                    FUNCT_SLLV => {
                        set!(cntr_field::ALU_OP_OUT, alu_op::SLL);
                        Ok(())
                    }
                    FUNCT_SRLV => {
                        set!(cntr_field::ALU_OP_OUT, alu_op::SRL);
                        Ok(())
                    }
                    FUNCT_SRAV => {
                        set!(cntr_field::ALU_OP_OUT, alu_op::SRL);
                        Ok(())
                    }
                    FUNCT_JR => {
                        set!(
                            cntr_field::BRANCH_INTERRUPT_OUT,
                            cntr_unit_signals::BRANCH_INTERRUPT
                        );
                        set!(
                            cntr_field::REG_WRITE_ENABLE_OUT,
                            cntr_unit_signals::REG_WRITE_DISABLE
                        );
                        Ok(())
                    }
                    FUNCT_JALR => {
                        set!(
                            cntr_field::BRANCH_INTERRUPT_OUT,
                            cntr_unit_signals::BRANCH_INTERRUPT
                        );
                        set!(cntr_field::REG_DEST_OUT, cntr_unit_signals::REG_DEST_RD); // this is different from syncsim as there its defined as 31, but that dosen't match mips documentation;
                        set!(cntr_field::ALU_SRC_A_OUT, cntr_unit_signals::ALU_SRC_A_ZERO);
                        set!(cntr_field::ALU_SRC_B_OUT, cntr_unit_signals::ALU_SRC_B_PC);
                        set!(cntr_field::ALU_OP_OUT, alu_op::ADDU);
                        Ok(())
                    }
                    SYSCALL => {
                        set!(
                            cntr_field::REG_WRITE_ENABLE_OUT,
                            cntr_unit_signals::REG_WRITE_DISABLE
                        );
                        set!(cntr_field::CP0_OUT, cntr_unit_signals::CP0_SYSCALL);
                        Ok(())
                    }
                    FUNCT_ADD => {
                        set!(cntr_field::ALU_OP_OUT, alu_op::ADD);
                        Ok(())
                    }
                    FUNCT_ADDU => {
                        set!(cntr_field::ALU_OP_OUT, alu_op::ADDU);
                        Ok(())
                    }
                    FUNCT_SUB => {
                        set!(cntr_field::ALU_OP_OUT, alu_op::SUB);
                        Ok(())
                    }
                    FUNCT_SUBU => {
                        set!(cntr_field::ALU_OP_OUT, alu_op::SUBU);
                        Ok(())
                    }
                    FUNCT_AND => {
                        set!(cntr_field::ALU_OP_OUT, alu_op::AND);
                        Ok(())
                    }
                    FUNCT_OR => {
                        set!(cntr_field::ALU_OP_OUT, alu_op::OR);
                        Ok(())
                    }
                    FUNCT_XOR => {
                        set!(cntr_field::ALU_OP_OUT, alu_op::XOR);
                        Ok(())
                    }
                    FUNCT_NOR => {
                        set!(cntr_field::ALU_OP_OUT, alu_op::NOR);
                        Ok(())
                    }
                    FUNCT_SLT => {
                        set!(cntr_field::ALU_OP_OUT, alu_op::SLT);
                        Ok(())
                    }
                    FUNCT_SLTU => {
                        set!(cntr_field::ALU_OP_OUT, alu_op::SLTU);
                        Ok(())
                    }
                    _ => Err(Condition::Error(format!(
                        "unknown funct {:#08b} for opcode 0b000000",
                        funct
                    ))),
                }
            }
            OP_1 => {
                // branch stuff, BGEZ BGEZAL BLTZ BLTZAL,
                // Note many branch and jump instructions are actually pseudo instructions and will be compiled to others
                // BAL => BGEZAL r0 offset
                let tmp: u32 = simulator.get_input_signal(&self.a_in).try_into().unwrap();
                let b_funct: u32 = (tmp >> 16) & 0b11111;

                set!(
                    cntr_field::BRANCH_INTERRUPT_OUT,
                    cntr_unit_signals::BRANCH_INTERRUPT
                );
                set!(
                    cntr_field::EXTEND_SELECT_OUT,
                    cntr_unit_signals::EXTEND_SIGNED
                );
                match b_funct {
                    B_FUNCT_BGEZ | B_FUNCT_BLTZ => Ok(()),
                    B_FUNCT_BGEZAL | B_FUNCT_BLTZAL => {
                        // save pc to reg
                        set!(cntr_field::REG_DEST_OUT, cntr_unit_signals::REG_DEST_31);
                        set!(
                            cntr_field::REG_WRITE_ENABLE_OUT,
                            cntr_unit_signals::REG_WRITE_ENABLE
                        );
                        set!(cntr_field::ALU_OP_OUT, alu_op::ADDU);
                        set!(cntr_field::ALU_SRC_A_OUT, cntr_unit_signals::ALU_SRC_A_ZERO);
                        set!(cntr_field::ALU_SRC_B_OUT, cntr_unit_signals::ALU_SRC_B_PC);
                        set!(
                            cntr_field::REG_WRITE_SRC_OUT,
                            cntr_unit_signals::WRITE_REG_SRC_ALU
                        );
                        Ok(())
                    }
                    _ => Err(Condition::Error(format!(
                        "unknown funct {:#07b} for opcode 0b000001",
                        b_funct
                    ))),
                }
            }
            OP_J => {
                set!(
                    cntr_field::BRANCH_INTERRUPT_OUT,
                    cntr_unit_signals::BRANCH_INTERRUPT
                );
                Ok(())
            }
            OP_JAL => {
                set!(
                    cntr_field::BRANCH_INTERRUPT_OUT,
                    cntr_unit_signals::BRANCH_INTERRUPT
                );
                set!(cntr_field::REG_DEST_OUT, cntr_unit_signals::REG_DEST_31);
                set!(
                    cntr_field::REG_WRITE_ENABLE_OUT,
                    cntr_unit_signals::REG_WRITE_ENABLE
                );
                set!(cntr_field::ALU_OP_OUT, alu_op::ADDU);
                set!(cntr_field::ALU_SRC_A_OUT, cntr_unit_signals::ALU_SRC_A_ZERO);
                set!(cntr_field::ALU_SRC_B_OUT, cntr_unit_signals::ALU_SRC_B_PC);
                set!(
                    cntr_field::REG_WRITE_SRC_OUT,
                    cntr_unit_signals::WRITE_REG_SRC_ALU
                );
                Ok(())
            }
            OP_BEQ | OP_BNE | OP_BLEZ | OP_BGTZ => {
                set!(
                    cntr_field::BRANCH_INTERRUPT_OUT,
                    cntr_unit_signals::BRANCH_INTERRUPT
                );
                set!(
                    cntr_field::EXTEND_SELECT_OUT,
                    cntr_unit_signals::EXTEND_SIGNED
                );
                Ok(())
            }
            OP_ADDI => {
                set_i_instr!();
                set!(
                    cntr_field::EXTEND_SELECT_OUT,
                    cntr_unit_signals::EXTEND_SIGNED
                );
                set!(cntr_field::ALU_OP_OUT, alu_op::ADD);
                Ok(())
            }
            OP_ADDIU => {
                set_i_instr!();
                set!(cntr_field::ALU_OP_OUT, alu_op::ADDU);
                Ok(())
            }
            OP_SLTI => {
                set_i_instr!();
                set!(
                    cntr_field::EXTEND_SELECT_OUT,
                    cntr_unit_signals::EXTEND_SIGNED
                );
                set!(cntr_field::ALU_OP_OUT, alu_op::SLT);
                Ok(())
            }
            OP_SLTIU => {
                set_i_instr!();
                set!(
                    cntr_field::EXTEND_SELECT_OUT,
                    cntr_unit_signals::EXTEND_ZERO
                );
                set!(cntr_field::ALU_OP_OUT, alu_op::SLTU);
                Ok(())
            }
            OP_ANDI => {
                set_i_instr!();
                set!(
                    cntr_field::EXTEND_SELECT_OUT,
                    cntr_unit_signals::EXTEND_ZERO
                );
                set!(cntr_field::ALU_OP_OUT, alu_op::AND);
                Ok(())
            }
            OP_ORI => {
                set_i_instr!();
                set!(
                    cntr_field::EXTEND_SELECT_OUT,
                    cntr_unit_signals::EXTEND_ZERO
                );
                set!(cntr_field::ALU_OP_OUT, alu_op::OR);
                Ok(())
            }
            OP_XORI => {
                set_i_instr!();
                set!(
                    cntr_field::EXTEND_SELECT_OUT,
                    cntr_unit_signals::EXTEND_ZERO
                );
                set!(cntr_field::ALU_OP_OUT, alu_op::XOR);
                Ok(())
            }
            OP_LUI => {
                set_i_instr!();
                set!(
                    cntr_field::EXTEND_SELECT_OUT,
                    cntr_unit_signals::EXTEND_ZERO
                );
                set!(cntr_field::ALU_OP_OUT, alu_op::LUI);
                Ok(())
            }
            OP_CP0 => {
                // let cp0_funct:u32 = (&instr_in >> 21) & 0b11111;
                // match cp0_funct {
                //     CP0_FUNCT_MFC0 =>{
                //         set!(cntr_field::REG_DEST_OUT,cntr_unit_signals::REG_DEST_RT);
                //         set!(cntr_field::REG_WRITE_ENABLE_OUT, cntr_unit_signals::REG_WRITE_ENABLE);

                //         //TODO no idea why alu would be required for this operation and cant find any path in syncsim,
                //         // but following blindly. If time permits figure out why and change
                //         set!(cntr_field::ALU_OP_OUT, alu_op::ADDU);
                //         set!(cntr_field::ALU_SRC_A_OUT,cntr_unit_signals::ALU_SRC_A_SHAMT);
                //         set!(cntr_field::ALU_SRC_B_OUT,cntr_unit_signals::ALU_SRC_B_IMM);
                //         todo!("implement memory mode to complete MFC0")

                //     }
                //     _ => {
                //         Err(Condition::Error(format!("unknown funct {:#07b}for opcode {:#08b} CP0", cp0_funct, OP_CP0)))
                //     }
                // }
                Err(Condition::Error(format!(
                    "CP0 instructions not yet implemented"
                )))
            }
            //TODO use mem_mode, now it assumed data_mem uses opcode to determine that itself
            OP_LB => {
                set!(cntr_field::MEM_MODE_OUT, data_op::LOAD_BYTE);
                set_load_instr!();
                Ok(())
            }
            OP_LBU => {
                set!(cntr_field::MEM_MODE_OUT, data_op::LOAD_BYTE_U);
                set_load_instr!();
                Ok(())
            }
            OP_LH => {
                set!(cntr_field::MEM_MODE_OUT, data_op::LOAD_HALF);
                set_load_instr!();
                Ok(())
            }
            OP_LHU => {
                set!(cntr_field::MEM_MODE_OUT, data_op::LOAD_HALF_U);
                set_load_instr!();
                Ok(())
            }
            OP_LW => {
                set!(cntr_field::MEM_MODE_OUT, data_op::LOAD_WORD);
                set_load_instr!();
                Ok(())
            }

            OP_SB => {
                set!(cntr_field::MEM_MODE_OUT, data_op::STORE_WORD);
                set_store_instr!();
                Ok(())
            }
            OP_SH => {
                set!(cntr_field::MEM_MODE_OUT, data_op::STORE_WORD);
                set_store_instr!();
                Ok(())
            }
            OP_SW => {
                set!(cntr_field::MEM_MODE_OUT, data_op::STORE_WORD);
                set_store_instr!();
                Ok(())
            }

            OP_LWL | OP_LWR | OP_SWL | OP_SWR => Err(Condition::Error(
                "LWL, LWR, SWL and SWR are not implemented".to_string(),
            )),

            _ => Err(Condition::Error(format!("Unknown opcode {:#08b}", op_code))),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ControlUnit {
    pub fn new(id: &str, pos: (f32, f32), a_in: Input) -> Self {
        ControlUnit {
            id: id.to_string(),
            pos,
            a_in,
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), a_in: Input) -> Rc<Self> {
        Rc::new(ControlUnit::new(id, pos, a_in))
    }
}
#[cfg(test)]
mod test {
    use crate::{common::ComponentStore, components::ProbeOut};

    use super::*;

    fn setup_simulator() -> Simulator {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("instr")),
                ControlUnit::rc_new("cntr", (0.0, 0.0), Input::new("instr", "out")),
            ],
        };
        Simulator::new(cs).unwrap()
    }
    /// This tests
    /// - beq t3 t0 7 => 000100 01011 01000 0000000000000111
    /// - xori $t6,$s4,32 => 001110 10100 01110 0000000000100000
    /// - sub $t0,$a0,$t0 => 000000 00100 01000 01000 00000 100010
    /// - lh $a1,14($s1) => 100001 10001 00101 0000000000001110
    /// - jal => 000011 10101010101010101010101010
    /// - jalr r10 r18 => 000000_10010_00000_01010_00000_001001
    #[test]
    fn test_random_instrs() {
        let mut sim = setup_simulator();

        assert_eq!(sim.cycle, 1);

        println!("testing beq t3 t0 7 => 000100 01011 01000 0000000000000111");
        sim.set_out_value("instr", "out", 0b000100_01011_01000_0000000000000111);
        println!("clock sim");
        sim.clock();
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::BRANCH_INTERRUPT_OUT)),
            cntr_unit_signals::BRANCH_INTERRUPT.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::REG_WRITE_ENABLE_OUT)),
            cntr_unit_signals::REG_WRITE_DISABLE.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::MEM_WRITE_ENABLE_OUT)),
            cntr_unit_signals::MEM_WRITE_DISABLE.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::EXTEND_SELECT_OUT)),
            cntr_unit_signals::EXTEND_SIGNED.into()
        );

        println!("testing xori $t6,$s4,32 => 001110 10100 01110 0000000000100000");
        sim.set_out_value("instr", "out", 0b001110_10100_01110_0000000000100000);
        println!("clock sim");
        sim.clock();
        // no branch or mem write
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::BRANCH_INTERRUPT_OUT)),
            cntr_unit_signals::NO_BRANCH_INTERRUPT.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::MEM_WRITE_ENABLE_OUT)),
            cntr_unit_signals::MEM_WRITE_DISABLE.into()
        );

        // reg write, src alu and reg_dest rt, since rd is occupied by imm
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::REG_WRITE_ENABLE_OUT)),
            cntr_unit_signals::REG_WRITE_ENABLE.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::REG_WRITE_SRC_OUT)),
            cntr_unit_signals::WRITE_REG_SRC_ALU.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::REG_DEST_OUT)),
            cntr_unit_signals::REG_DEST_RT.into()
        );

        // ALU xor rs and imm zero extend
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::EXTEND_SELECT_OUT)),
            cntr_unit_signals::EXTEND_ZERO.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::ALU_SRC_A_OUT)),
            cntr_unit_signals::ALU_SRC_A_RS.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::ALU_SRC_B_OUT)),
            cntr_unit_signals::ALU_SRC_B_IMM.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::ALU_OP_OUT)),
            alu_op::XOR.into()
        );

        print!("testing sub $t0,$a0,$t0 => 000000 00100 01000 01000 00000 100010");
        sim.set_out_value("instr", "out", 0b000000_00100_01000_01000_00000_100010);
        println!("clock sim");
        sim.clock();
        // no branch or mem write
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::BRANCH_INTERRUPT_OUT)),
            cntr_unit_signals::NO_BRANCH_INTERRUPT.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::MEM_WRITE_ENABLE_OUT)),
            cntr_unit_signals::MEM_WRITE_DISABLE.into()
        );

        // reg write, src alu and reg_dest rd
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::REG_WRITE_ENABLE_OUT)),
            cntr_unit_signals::REG_WRITE_ENABLE.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::REG_WRITE_SRC_OUT)),
            cntr_unit_signals::WRITE_REG_SRC_ALU.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::REG_DEST_OUT)),
            cntr_unit_signals::REG_DEST_RD.into(),
        );

        // ALU sub rs and rt
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::ALU_SRC_A_OUT)),
            cntr_unit_signals::ALU_SRC_A_RS.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::ALU_SRC_B_OUT)),
            cntr_unit_signals::ALU_SRC_B_RT.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::ALU_OP_OUT)),
            alu_op::SUB.into()
        );

        print!("testing lh $a1,14($s1) => 100001 10001 00101 0000000000001110");
        sim.set_out_value("instr", "out", 0b100001_10001_00101_0000000000001110);
        println!("clock sim");
        sim.clock();
        // reg write, no branch, no mem write
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::BRANCH_INTERRUPT_OUT)),
            cntr_unit_signals::NO_BRANCH_INTERRUPT.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::MEM_WRITE_ENABLE_OUT)),
            cntr_unit_signals::MEM_WRITE_DISABLE.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::REG_WRITE_ENABLE_OUT)),
            cntr_unit_signals::REG_WRITE_ENABLE.into()
        );

        // reg dst rt, reg src mem
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::REG_WRITE_SRC_OUT)),
            cntr_unit_signals::WRITE_REG_SRC_MEM.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::REG_DEST_OUT)),
            cntr_unit_signals::REG_DEST_RT.into()
        );

        // ADDU rs imm (signed)
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::EXTEND_SELECT_OUT)),
            cntr_unit_signals::EXTEND_SIGNED.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::ALU_SRC_A_OUT)),
            cntr_unit_signals::ALU_SRC_A_RS.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::ALU_SRC_B_OUT)),
            cntr_unit_signals::ALU_SRC_B_IMM.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::ALU_OP_OUT)),
            alu_op::ADDU.into()
        );

        println!("testing SW $r2  4($r3) => 101011 00011 00010 0000_0000_0000_0100");
        sim.set_out_value("instr", "out", 0b101011_00011_00010_0000_0000_0000_0100);
        println!("clock sim");
        sim.clock();
        // no reg write, no branch, mem write
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::BRANCH_INTERRUPT_OUT)),
            cntr_unit_signals::NO_BRANCH_INTERRUPT.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::MEM_WRITE_ENABLE_OUT)),
            cntr_unit_signals::MEM_WRITE_ENABLE.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::REG_WRITE_ENABLE_OUT)),
            cntr_unit_signals::REG_WRITE_DISABLE.into()
        );

        // ADDU rs imm (signed)
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::EXTEND_SELECT_OUT)),
            cntr_unit_signals::EXTEND_SIGNED.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::ALU_SRC_A_OUT)),
            cntr_unit_signals::ALU_SRC_A_RS.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::ALU_SRC_B_OUT)),
            cntr_unit_signals::ALU_SRC_B_IMM.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::ALU_OP_OUT)),
            alu_op::ADDU.into()
        );

        println!("testing jal => 000011 10101010101010101010101010 ");
        sim.set_out_value("instr", "out", 0b000011_10101010101010101010101010);
        println!("clock sim");
        sim.clock();
        // reg write, branch, no mem write
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::BRANCH_INTERRUPT_OUT)),
            cntr_unit_signals::BRANCH_INTERRUPT.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::MEM_WRITE_ENABLE_OUT)),
            cntr_unit_signals::MEM_WRITE_DISABLE.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::REG_WRITE_ENABLE_OUT)),
            cntr_unit_signals::REG_WRITE_ENABLE.into()
        );

        // ALU zero + pc
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::ALU_SRC_A_OUT)),
            cntr_unit_signals::ALU_SRC_A_ZERO.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::ALU_SRC_B_OUT)),
            cntr_unit_signals::ALU_SRC_B_PC.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::ALU_OP_OUT)),
            alu_op::ADDU.into()
        );

        // reg dst 31, reg src alu
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::REG_WRITE_SRC_OUT)),
            cntr_unit_signals::WRITE_REG_SRC_ALU.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::REG_DEST_OUT)),
            cntr_unit_signals::REG_DEST_31.into()
        );

        println!("testing jalr r10 r18 => 000000_10010_00000_01010_00000_001001");
        sim.set_out_value("instr", "out", 0b000000_10010_00000_01010_00000_001001);
        println!("clock sim");
        sim.clock();
        // reg write, branch, no mem write
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::BRANCH_INTERRUPT_OUT)),
            cntr_unit_signals::BRANCH_INTERRUPT.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::MEM_WRITE_ENABLE_OUT)),
            cntr_unit_signals::MEM_WRITE_DISABLE.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::REG_WRITE_ENABLE_OUT)),
            cntr_unit_signals::REG_WRITE_ENABLE.into()
        );

        // ALU zero + pc
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::ALU_SRC_A_OUT)),
            cntr_unit_signals::ALU_SRC_A_ZERO.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::ALU_SRC_B_OUT)),
            cntr_unit_signals::ALU_SRC_B_PC.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::ALU_OP_OUT)),
            alu_op::ADDU.into()
        );

        // reg dst rd, reg src alu
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::REG_WRITE_SRC_OUT)),
            cntr_unit_signals::WRITE_REG_SRC_ALU.into()
        );
        assert_eq!(
            sim.get_input_value(&Input::new("cntr", cntr_field::REG_DEST_OUT)),
            cntr_unit_signals::REG_DEST_RD.into()
        );
    }
}
