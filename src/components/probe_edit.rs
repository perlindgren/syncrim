use crate::common::{Component, Id, OutputType, Ports, Signal, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};

use log::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProbeEdit {
    pub id: Id,
    pub pos: (f32, f32),
    pub data: Rc<RefCell<Signal>>,
}

#[typetag::serde]
impl Component for ProbeEdit {
    fn to_(&self) {
        trace!("ProbeEdit");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                // Constants do not take any inputs
                vec![],
                OutputType::Combinatorial,
                // Single output value
                vec!["out"],
            ),
        )
    }

    // propagate editable value
    fn evaluate(&self, simulator: &mut Simulator) {
        let value = *self.data.borrow();
        error!("value {}", value);

        // set output
        simulator.set_out_val(&self.id, "out", value);
    }
}

impl ProbeEdit {
    pub fn new(id: &str, pos: (f32, f32)) -> Self {
        ProbeEdit {
            id: id.into(),
            pos,
            data: Rc::new(RefCell::new(0)),
        }
    }
}
