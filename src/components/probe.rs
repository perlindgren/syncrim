use crate::common::{Component, Id, Input, OutputType, Ports};
use log::*;
use serde::{Deserialize, Serialize};
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
