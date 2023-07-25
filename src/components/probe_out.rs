use crate::common::{Component, Id, OutputType, Ports};
use log::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProbeOut {
    pub id: Id,
}

#[typetag::serde]
impl Component for ProbeOut {
    fn to_(&self) {
        trace!("ProbeOut");
    }
    fn to_string(&self)->String{"".to_string()}
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
}

impl ProbeOut {
    pub fn new(id: &str) -> Self {
        ProbeOut { id: id.into() }
    }
}
