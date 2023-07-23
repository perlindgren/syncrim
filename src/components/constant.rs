use crate::common::{Component, EguiExtra, Id, OutputType, Ports, Signal, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Constant {
    pub id: Id,
    pub pos: (f32, f32),
    pub value: Signal,
    #[cfg(feature = "gui-egui")]
    #[serde(skip_serializing)]
    pub egui_x: EguiExtra,
}

impl Constant {
    pub fn new(id: String, pos: (f32, f32), value: Signal) -> Self {
        Constant {
            id: id.clone(),
            pos,
            value,
            egui_x: EguiExtra {
                properties_window: false,
                id_tmp: id,
            },
        }
    }
}

#[typetag::serde]
impl Component for Constant {
    fn to_(&self) {
        trace!("constant {:?}", self.value);
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                // Constants do not take any inputs
                vec![],
                OutputType::Combinatorial,
                vec!["out"],
            ),
        )
    }

    fn clock(&self, simulator: &mut Simulator) {
        simulator.set_out_val(&self.id, "out", self.value);
    }
}
