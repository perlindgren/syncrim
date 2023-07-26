use crate::common::{Component, Id, Input, InputPort, OutputType, Ports, Simulator};
use log::*;
use serde::{Deserialize, Serialize};

pub const REGISTER_IN_ID: &str = "r_in";

#[derive(Serialize, Deserialize)]
pub struct Register {
    pub id: Id,
    pub pos: (f32, f32),
    pub r_in: Input,
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
                vec![&InputPort {
                    port_id: REGISTER_IN_ID.to_string(),
                    input: self.r_in.clone(),
                }],
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
        simulator.set_out_val(&self.id, "out", value);
        trace!("eval: register id {} in {:?}", self.id, value);
    }
}
