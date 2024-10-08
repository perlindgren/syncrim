// use std::fmt::Alignment;
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;
use syncrim::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, SignalValue, Simulator,
};

pub const MERGE_INSTR_ADDR_ID: &str = "merge_instr_addr_in";
pub const MERGE_JUMP_ADDR_ID: &str = "merge_jump_addr_in";

pub const MERGE_OUT_ID: &str = "merge_out";

#[derive(Serialize, Deserialize, Clone)]
pub struct JumpMerge {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) instr_addr_in: Input,
    pub(crate) jump_addr_in: Input,
}

#[typetag::serde]
impl Component for JumpMerge {
    fn to_(&self) {
        trace!("jump_merge");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: MERGE_INSTR_ADDR_ID.to_string(),
                        input: self.instr_addr_in.clone(),
                    },
                    &InputPort {
                        port_id: MERGE_JUMP_ADDR_ID.to_string(),
                        input: self.jump_addr_in.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![MERGE_OUT_ID],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            MERGE_INSTR_ADDR_ID => self.instr_addr_in = new_input,
            MERGE_JUMP_ADDR_ID => self.jump_addr_in = new_input,
            _ => {}
        }
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input values
        let instr_addr: u32 = simulator
            .get_input_value(&self.instr_addr_in)
            .try_into()
            .unwrap();
        let jump_addr: u32 = simulator
            .get_input_value(&self.jump_addr_in)
            .try_into()
            .unwrap();

        let output = (instr_addr & 0xf000_0000) | (jump_addr.overflowing_shl(2).0 & 0x0fff_ffff);

        simulator.set_out_value(&self.id, MERGE_OUT_ID, SignalValue::Data(output));
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl JumpMerge {
    pub fn new(id: &str, pos: (f32, f32), instr_addr_in: Input, jump_addr_in: Input) -> Self {
        JumpMerge {
            id: id.to_string(),
            pos,
            instr_addr_in,
            jump_addr_in,
        }
    }

    pub fn rc_new(
        id: &str,
        pos: (f32, f32),
        instr_addr_in: Input,
        jump_addr_in: Input,
    ) -> Rc<Self> {
        Rc::new(JumpMerge::new(id, pos, instr_addr_in, jump_addr_in))
    }
}
