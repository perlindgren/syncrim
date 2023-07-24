use crate::common::{Component, Id, OutputType, Ports, Signal, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
#[derive(Serialize, Deserialize)]
pub struct Constant {
    pub id: Id,
    pub pos: (f32, f32),
    pub value: Signal,
    #[cfg(feature = "gui-egui")]
    #[serde(skip)]
    pub egui_x: EguiExtra,
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

impl Constant {
    pub fn new(id: &str, pos: (f32, f32), value: impl Into<Signal>) -> Self {
        Constant {
            id: id.to_string(),
            pos,
            value: value.into(),
            #[cfg(feature = "gui-egui")]
            egui_x: crate::common::EguiExtra::default(),
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), value: impl Into<Signal>) -> Rc<Self> {
        Rc::new(Constant::new(id, pos, value))
    }
}
