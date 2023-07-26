use crate::common::{Component, Id, Input, OutputType, Ports};
use log::*;
use serde::{Deserialize, Serialize};
use std::{convert::Into, rc::Rc};
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
                vec![&self.input],
                OutputType::Combinatorial,
                // No output value
                vec![],
            ),
        )
    }
}

impl Probe {
    pub fn new(id: &str, pos: (impl Into<f32>, impl Into<f32>), input: Input) -> Self {
        Probe {
            id: id.to_string(),
            pos: (pos.0.into(), pos.1.into()),
            input,
        }
    }

    pub fn rc_new(id: &str, pos: (impl Into<f32>, impl Into<f32>), input: Input) -> Rc<Self> {
        Rc::new(Probe::new(id, pos, input))
    }
}
