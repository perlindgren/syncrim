// use std::fmt::Alignment;
#[cfg(feature = "gui-egui")]
use crate::common::EguiComponent;
use crate::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, SignalSigned, SignalUnsigned,
    SignalValue, Simulator,
};
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;

pub const CONTROL_UNIT_A_IN_ID: &str = "control_unit_a_in";

pub const CONTROL_UNIT_MEMTOREG_ID: &str = "MemToReg";
pub const CONTROL_UNIT_MEMWRITE_ID: &str = "MemWrite";
pub const CONTROL_UNIT_BRANCH_ID: &str = "Branch";
pub const CONTROL_UNIT_ALUCONTROL_ID: &str = "ALUControl";
pub const CONTROL_UNIT_ALUSRC_ID: &str = "ALUSrc";
pub const CONTROL_UNIT_REGDST_ID: &str = "RegDst";
pub const CONTROL_UNIT_WRITEENABLE_ID: &str = "WriteEnable";
pub const CONTROL_UNIT_JUMP_ID: &str = "Jump";

const NOP: u32 = 0;
const OP_0: u32 = 0;
const OP_1: u32 = 1;
const OP_J: u32 = 0b000_010;
const OP_JAL: u32 = 0b000_011;
const OP_BEQ: u32 = 0b000_100;
const OP_BNE: u32 = 0b000_101;
const OP_BLEZ: u32 = 0b000_110;
const OP_BGTZ: u32 = 0b000_111;
const OP_ADDI: u32 = 0b001_000;
const OP_ADDIU: u32 = 0b001_001;
const OP_SLTI: u32 = 0b001_010;
const OP_SLTIU: u32 = 0b001_011;
const OP_ANDI: u32 = 0b001_100;
const OP_ORI: u32 = 0b001_101;
const OP_XORI: u32 = 0b001_110;
const OP_LUI: u32 = 0b001_111;
const CP0: u32 = 0b010_000;

const OP_LB: u32 = 0b100_000;
const OP_LH: u32 = 0b100_001;
const OP_LWL: u32 = 0b100_010;
const OP_LW: u32 = 0b100_011;
const OP_LBU: u32 = 0b100_100;
const OP_LHU: u32 = 0b100_101;
const OP_LWR: u32 = 0b100_110;

const OP_SB: u32 = 0b101_000;
const OP_SH: u32 = 0b101_001;
const OP_SWL: u32 = 0b101_010;
const OP_SW: u32 = 0b101_011;
const OP_SWR: u32 = 0b101_110;

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
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
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
                    port_id: CONTROL_UNIT_A_IN_ID.to_string(),
                    input: self.a_in.clone(),
                }],
                OutputType::Combinatorial,
                vec![
                    CONTROL_UNIT_MEMTOREG_ID,
                    CONTROL_UNIT_MEMWRITE_ID,
                    CONTROL_UNIT_BRANCH_ID,
                    CONTROL_UNIT_ALUCONTROL_ID,
                    CONTROL_UNIT_ALUSRC_ID,
                    CONTROL_UNIT_REGDST_ID,
                    CONTROL_UNIT_WRITEENABLE_ID,
                    CONTROL_UNIT_JUMP_ID,
                ],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            CONTROL_UNIT_A_IN_ID => self.a_in = new_input,
            _ => {}
        }
    }

    // propagate sign extension to output
    // TODO: always extend to Signal size? (it should not matter and should be slightly cheaper)
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input values
        let a: u32 = simulator.get_input_value(&self.a_in).try_into().unwrap();

        let a_OpCode: u32 = (&a >> 26) & 0x0000_003f;
        let a_func: u32 = &a & 0x0000_001f;

        let MemToReg;
        let MemWrite;
        let Branch;
        let ALUControl;
        let ALUSrc;
        let RegDst;
        let WriteEnable;
        let Jump;

        if a_OpCode == 0 || a_OpCode == 4 {
            ALUSrc = 0;
        } else {
            ALUSrc = 1;
        }

        if a_OpCode == 0 {
            RegDst = 1;
        } else {
            RegDst = 0;
        }

        if a_OpCode == 4 {
            Branch = 1;
        } else {
            Branch = 0;
        }

        if a_OpCode == 35 {
            MemToReg = 1;
        } else {
            MemToReg = 0;
        }

        if a_OpCode == 43 {
            MemWrite = 1;
        } else {
            MemWrite = 0;
        }

        if a_OpCode == 2 {
            Jump = 1;
        } else {
            Jump = 0;
        }

        if (a_OpCode == 0)
            & (a_func == 32 || a_func == 34 || a_func == 42 || a_func == 36 || a_func == 37)
        {
            WriteEnable = 1;
        } else if a_OpCode == 8 || a_OpCode == 10 || a_OpCode == 35 {
            WriteEnable = 1;
        } else {
            WriteEnable = 0;
        }

        if (a_OpCode == 0) & (a_func == 32) {
            ALUControl = 2; // AND
        } else if a_OpCode == 8 {
            ALUControl = 2; // ANDI
        } else if a_OpCode == 35 {
            ALUControl = 2; //lw
        } else if a_OpCode == 43 {
            ALUControl = 2; //sw
        } else if a_OpCode == 4 {
            ALUControl = 6; // beq
        } else if (a_OpCode == 0) & (a_func == 34) {
            ALUControl = 6; // SUB
        } else if (a_OpCode == 0) & (a_func == 42) {
            ALUControl = 7; // SLT
        } else if a_OpCode == 10 {
            ALUControl = 7; // SLTI
        } else if (a_OpCode == 0) & (a_func == 36) {
            ALUControl = 0; // AND
        } else if (a_OpCode == 0) & (a_func == 37) {
            ALUControl = 1; //OR
        } else {
            ALUControl = 0;
        }

        simulator.set_out_value(
            &self.id,
            CONTROL_UNIT_MEMTOREG_ID,
            SignalValue::Data(MemToReg),
        );
        simulator.set_out_value(
            &self.id,
            CONTROL_UNIT_MEMWRITE_ID,
            SignalValue::Data(MemWrite),
        );
        simulator.set_out_value(&self.id, CONTROL_UNIT_BRANCH_ID, SignalValue::Data(Branch));
        simulator.set_out_value(
            &self.id,
            CONTROL_UNIT_ALUCONTROL_ID,
            SignalValue::Data(ALUControl),
        );
        simulator.set_out_value(&self.id, CONTROL_UNIT_ALUSRC_ID, SignalValue::Data(ALUSrc));
        simulator.set_out_value(&self.id, CONTROL_UNIT_REGDST_ID, SignalValue::Data(RegDst));
        simulator.set_out_value(&self.id, CONTROL_UNIT_JUMP_ID, SignalValue::Data(Jump));
        simulator.set_out_value(
            &self.id,
            CONTROL_UNIT_WRITEENABLE_ID,
            SignalValue::Data(WriteEnable),
        );

        Ok(())
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
