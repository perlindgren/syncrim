use crate::common::{Component, Id, Input, OutputType, Ports};
use log::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
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
                vec![&self.input],
                OutputType::Combinatorial,
                // No output value
                vec![],
            ),
        )
    }
}

impl Wire {
    pub fn new(id: &str, pos: Vec<(f32, f32)>, input: Input) -> Self {
        Wire {
            id: id.to_string(),
            pos,
            input,
        }
    }

    pub fn rc_new(id: &str, pos: Vec<(f32, f32)>, input: Input) -> Rc<Wire> {
        Rc::new(Wire::new(id, pos, input))
    }
}
