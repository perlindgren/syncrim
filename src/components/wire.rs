use crate::common::{Component, Id, Input, InputPort, OutputType, Ports};
use log::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Wire {
    pub id: Id,
    pub pos: Vec<(f32, f32)>,
    pub input_port: InputPort,

    #[cfg(feature = "gui-egui")]
    #[serde(skip)]
    pub egui_x: crate::common::EguiExtra,
}

impl Wire {
    pub fn new(id: &str, pos: Vec<(f32, f32)>, input: Input) -> Self {
        Wire {
            id: id.to_string(),
            pos,
            input_port: InputPort {
                port_id: String::from("in"),
                input,
            },
            #[cfg(feature = "gui-egui")]
            egui_x: crate::common::EguiExtra::default(),
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
                vec![&self.input_port],
                OutputType::Combinatorial,
                // No output value
                vec![],
            ),
        )
    }
}
