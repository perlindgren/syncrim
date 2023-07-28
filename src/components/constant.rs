use crate::common::{Component, Id, OutputType, Ports, Signal, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
use std::{convert::Into, rc::Rc};
#[derive(Serialize, Deserialize)]
pub struct Constant {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) value: Signal,
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
        simulator.set_out_value(&self.id, "out", self.value.get_value());
    }
}

impl Constant {
    pub fn new(id: &str, pos: (f32, f32), value: impl Into<Signal>) -> Self {
        Constant {
            id: id.to_string(),
            pos,
            value: value.into(),
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), value: impl Into<Signal>) -> Rc<Self> {
        Rc::new(Constant::new(id, pos, value))
    }
}
