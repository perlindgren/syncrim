use crate::common::{Component, Id, OutputType, Ports};
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProbeOut {
    pub(crate) id: Id,
}

#[typetag::serde]
impl Component for ProbeOut {
    fn to_(&self) {
        trace!("ProbeOut");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                // Constants do not take any inputs
                vec![],
                OutputType::Combinatorial,
                // Single output value
                vec!["out"],
            ),
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ProbeOut {
    pub fn new(id: &str) -> Self {
        ProbeOut { id: id.into() }
    }
}
