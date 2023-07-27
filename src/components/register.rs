use crate::common::{Component, Id, Input, InputPort, OutputType, Ports, Simulator};
use log::*;
use serde::{Deserialize, Serialize};

pub const REGISTER_R_IN_ID: &str = "r_in";

pub const REGISTER_OUT_ID: &str = "out";

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
                    port_id: REGISTER_R_IN_ID.to_string(),
                    input: self.r_in.clone(),
                }],
                OutputType::Sequential,
                vec![REGISTER_OUT_ID],
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

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        if target_port_id == REGISTER_R_IN_ID {
            self.r_in = new_input;
        }
    }
}
