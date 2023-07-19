use crate::common::{Component, Id, Input, OutputType, Ports};
use log::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Probe {
    pub id: Id,
    pub pos: (f32, f32),
    pub input: Input,
    // this is ugly... (egui)
    pub properties_window: bool,
    pub id_tmp: Id,
}

impl Probe {
    pub fn new(id: String, pos: (f32, f32), input: Input) -> Self {
        Probe {
            id: id.clone(),
            pos,
            input,
            properties_window: false,
            id_tmp: id,
        }
    }
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
