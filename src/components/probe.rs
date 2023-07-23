use crate::common::{Component, Id, Input, InputId, OutputType, Ports};
use log::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Probe {
    pub id: Id,
    pub pos: (f32, f32),
    pub input_id: InputId,
    // this is ugly... (egui)
    pub properties_window: bool,
    pub id_tmp: Id,
}

impl Probe {
    pub fn new(id: String, pos: (f32, f32), input: Input) -> Self {
        Probe {
            id: id.clone(),
            pos,
            input_id: InputId {
                id: String::from("in"),
                input,
            },
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
                vec![&self.input_id],
                OutputType::Combinatorial,
                // No output value
                vec![],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            "in" => self.input_id.input = new_input,
            _ => (),
        }
    }
}
