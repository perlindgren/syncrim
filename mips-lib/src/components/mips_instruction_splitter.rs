// use std::fmt::Alignment;
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;
#[cfg(feature = "gui-egui")]
use syncrim::common::EguiComponent;
use syncrim::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, SignalValue, Simulator,
};

pub const INSTRUCTION_SPLITTER_IN_ID: &str = "instruction_in";

pub const INSTRUCTION_SPLITTER_OP_ID: &str = "op_out";
pub const INSTRUCTION_SPLITTER_RS_ID: &str = "rs_out";
pub const INSTRUCTION_SPLITTER_RT_ID: &str = "rt_out";
pub const INSTRUCTION_SPLITTER_RD_ID: &str = "rd_out";
pub const INSTRUCTION_SPLITTER_SHAMT_ID: &str = "shamt_out";
pub const INSTRUCTION_SPLITTER_FUNCT_ID: &str = "funct_out";
pub const INSTRUCTION_SPLITTER_IMMEDIATE_ID: &str = "immediate_out";
pub const INSTRUCTION_SPLITTER_TARGET_ID: &str = "target_out";

#[derive(Serialize, Deserialize, Clone)]
pub struct InstrSplit {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) instruction_in: Input,
}

#[typetag::serde]
impl Component for InstrSplit {
    fn to_(&self) {
        trace!("pc+4");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, _id: &str, _pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(InstrSplit {
            id: "dummy".to_string(),
            pos: (0.0, 0.0),
            instruction_in: dummy_input.clone(),
        }))
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![&InputPort {
                    port_id: INSTRUCTION_SPLITTER_IN_ID.to_string(),
                    input: self.instruction_in.clone(),
                }],
                OutputType::Combinatorial,
                vec![
                    INSTRUCTION_SPLITTER_OP_ID,
                    INSTRUCTION_SPLITTER_RS_ID,
                    INSTRUCTION_SPLITTER_RT_ID,
                    INSTRUCTION_SPLITTER_RD_ID,
                    INSTRUCTION_SPLITTER_SHAMT_ID,
                    INSTRUCTION_SPLITTER_FUNCT_ID,
                    INSTRUCTION_SPLITTER_IMMEDIATE_ID,
                    INSTRUCTION_SPLITTER_TARGET_ID,
                ],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            INSTRUCTION_SPLITTER_IN_ID => self.instruction_in = new_input,
            _ => {}
        }
    }

    // propagate sign extension to output
    // TODO: always extend to Signal size? (it should not matter and should be slightly cheaper)
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input values
        let instruction: u32 = simulator
            .get_input_value(&self.instruction_in)
            .try_into()
            .unwrap();

        let op = (instruction >> 26) & 0x0000_003f;
        let rs = (instruction >> 21) & 0x0000_001f;
        let rt = (instruction >> 16) & 0x0000_001f;
        let rd = (instruction >> 11) & 0x0000_001f;
        let shamt = (instruction >> 6) & 0x0000_001f;
        let funct = instruction & 0x0000_003f;
        let immediate = instruction & 0x0000_ffff;
        let target = instruction & 0x03ff_ffff;

        simulator.set_out_value(&self.id, INSTRUCTION_SPLITTER_OP_ID, SignalValue::Data(op));
        simulator.set_out_value(&self.id, INSTRUCTION_SPLITTER_RS_ID, SignalValue::Data(rs));
        simulator.set_out_value(&self.id, INSTRUCTION_SPLITTER_RT_ID, SignalValue::Data(rt));
        simulator.set_out_value(&self.id, INSTRUCTION_SPLITTER_RD_ID, SignalValue::Data(rd));
        simulator.set_out_value(
            &self.id,
            INSTRUCTION_SPLITTER_SHAMT_ID,
            SignalValue::Data(shamt),
        );
        simulator.set_out_value(
            &self.id,
            INSTRUCTION_SPLITTER_FUNCT_ID,
            SignalValue::Data(funct),
        );
        simulator.set_out_value(
            &self.id,
            INSTRUCTION_SPLITTER_IMMEDIATE_ID,
            SignalValue::Data(immediate),
        );
        simulator.set_out_value(
            &self.id,
            INSTRUCTION_SPLITTER_TARGET_ID,
            SignalValue::Data(target),
        );
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl InstrSplit {
    pub fn new(id: &str, pos: (f32, f32), instruction_in: Input) -> Self {
        InstrSplit {
            id: id.to_string(),
            pos,
            instruction_in,
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), instruction_in: Input) -> Rc<Self> {
        Rc::new(InstrSplit::new(id, pos, instruction_in))
    }
}
