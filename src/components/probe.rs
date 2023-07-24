use crate::common::{Component, Id, Input, InputId, OutputType, Ports};
use log::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Probe {
    pub id: Id,
    pub pos: (f32, f32),
    pub input_id: InputId,
    #[cfg(feature = "gui-egui")]
    #[serde(skip)]
    pub egui_x: EguiExtra,
}

impl Probe {
    pub fn new(id: &str, pos: (f32, f32), input: Input) -> Self {
        Probe {
            id: id.to_string(),
            pos,
            input_id: InputId {
                id: String::from("in"),
                input,
            },
            #[cfg(feature = "gui-egui")]
            egui_x: EguiExtra::default(),
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
