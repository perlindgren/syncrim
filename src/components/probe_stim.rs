use crate::common::{Component, Id, OutputType, Ports, Signal, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize)]
pub struct ProbeStim {
    pub id: Id,
    pub pos: (f32, f32),
    pub values: Vec<Signal>,
}

#[typetag::serde]
impl Component for ProbeStim {
    fn to_(&self) {
        trace!("constant {:?}", self.values);
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                // ProbeStim do not take any inputs
                vec![],
                OutputType::Combinatorial,
                vec!["out"],
            ),
        )
    }

    fn clock(&self, simulator: &mut Simulator) {
        // trace!("-- clock --", simulator.clock)
        simulator.set_out_val(&self.id, "out", self.values[0]);
    }
}

impl ProbeStim {
    pub fn new(id: &str, pos: (f32, f32), values: Vec<impl Into<Signal>>) -> Self {
        ProbeStim {
            id: id.to_string(),
            pos,
            values: values.into_iter().map(|v| v.into()).collect(),
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), values: Vec<impl Into<Signal>>) -> Rc<Self> {
        Rc::new(ProbeStim::new(id, pos, values))
    }
}
