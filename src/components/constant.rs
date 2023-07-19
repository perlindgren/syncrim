use crate::common::{Component, Id, OutputType, Ports, Signal, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Constant {
    pub id: Id,
    pub pos: (f32, f32),
    pub value: Signal,
    // this is ugly... (egui)
    pub properties_window: bool,
    pub id_tmp: Id,
}

impl Constant {
    pub fn new(id: String, pos: (f32, f32), value: Signal) -> Self {
        Constant {
            id: id.clone(),
            pos,
            value,
            properties_window: false,
            id_tmp: id,
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

    fn evaluate(&self, simulator: &mut Simulator) {
        simulator.set_out_val(&self.id, "out", self.value);
    }
}
