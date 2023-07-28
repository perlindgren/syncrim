use crate::common::{Component, Id, Input, OutputType, Ports, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize)]
pub struct Register {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) r_in: Input,
}

#[typetag::serde]
impl Component for Register {
    fn to_(&self) {
        trace!("register");
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                // Vector of inputs
                vec![&self.r_in],
                OutputType::Sequential,
                vec!["out"],
            ),
        )
    }

    // propagate input value to output
    fn clock(&self, simulator: &mut Simulator) {
        // get input value
        let value = simulator.get_input_val(&self.r_in);
        // set output
        simulator.set_out_value(&self.id, "out", value);
        trace!("eval: register id {} in {:?}", self.id, value);
    }
}

impl Register {
    pub fn new(id: &str, pos: (f32, f32), r_in: Input) -> Self {
        Register {
            id: id.to_string(),
            pos,
            r_in,
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), r_in: Input) -> Rc<Self> {
        Rc::new(Register::new(id, pos, r_in))
    }
}
