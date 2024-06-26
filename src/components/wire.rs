use crate::common::{Component, Id, Input, InputPort, OutputType, Ports};
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;

pub const WIRE_INPUT_ID: &str = "in";

#[derive(Serialize, Deserialize, Clone)]
pub struct Wire {
    pub(crate) id: Id,
    pub(crate) pos: Vec<(f32, f32)>,
    pub(crate) input: Input,
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

    fn as_any(&self) -> &dyn Any {
        self
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
