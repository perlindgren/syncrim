use crate::common::{Component, EguiExtra, Id, Input, InputId, OutputType, Ports};
use log::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Wire {
    pub id: Id,
    pub pos: Vec<(f32, f32)>,
    pub input_id: InputId,

    #[cfg(feature = "gui-egui")]
    #[serde(skip)]
    pub egui_x: EguiExtra,
}

impl Wire {
    pub fn new(id: String, pos: Vec<(f32, f32)>, input: Input) -> Self {
        Wire {
            id: id.clone(),
            pos,
            input_id: InputId {
                id: String::from("in"),
                input,
            },
            egui_x: EguiExtra {
                properties_window: false,
                id_tmp: id,
            },
        }
    }
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
                vec![&self.input_id],
                OutputType::Combinatorial,
                // No output value
                vec![],
            ),
        )
    }
}
