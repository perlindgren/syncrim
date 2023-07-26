use crate::common::{Component, Id, Input, InputPort, OutputType, Ports};
use log::*;
use serde::{Deserialize, Serialize};

pub const WIRE_INPUT_ID: &str = "in";

#[derive(Serialize, Deserialize)]
pub struct Wire {
    pub id: Id,
    pub pos: Vec<(f32, f32)>,
    pub input: Input,
}

#[typetag::serde]
impl Component for Wire {
    fn to_(&self) {
        trace!("Wire");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                // Wires take one input
                vec![&InputPort {
                    port_id: WIRE_INPUT_ID.to_string(),
                    input: self.input.clone(),
                }],
                OutputType::Combinatorial,
                // No output value
                vec![],
            ),
        )
    }
}
