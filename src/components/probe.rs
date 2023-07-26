use crate::common::{Component, Id, Input, InputPort, OutputType, Ports};
use log::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

pub const PROBE_IN_ID: &str = "in";

#[derive(Serialize, Deserialize)]
pub struct Probe {
    pub id: Id,
    pub pos: (f32, f32),
    pub input: Input,
}

#[typetag::serde]
impl Component for Probe {
    fn to_(&self) {
        trace!("Probe");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                // Probes take one input
                vec![&InputPort {
                    port_id: PROBE_IN_ID.to_string(),
                    input: self.input.clone(),
                }],
                OutputType::Combinatorial,
                // No output value
                vec![],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        if target_port_id.as_str() == PROBE_IN_ID {
            self.input = new_input
        }
    }
}

impl Probe {
    pub fn new(id: &str, pos: (f32, f32), input: Input) -> Self {
        Probe {
            id: id.to_string(),
            pos,
            input,
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), input: Input) -> Rc<Self> {
        Rc::new(Probe::new(id, pos, input))
    }
}
