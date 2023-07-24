use crate::common::{Component, Id, Input, InputId, OutputType, Ports, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Register {
    pub id: Id,
    pub pos: (f32, f32),
    pub r_in: InputId,

    #[cfg(feature = "gui-egui")]
    #[serde(skip)]
    pub egui_x: crate::common::EguiExtra,
}
impl Register {
    pub fn new(id: &str, pos: (f32, f32), r_in: Input) -> Self {
        Register {
            id: id.to_string(),
            pos,
            r_in: InputId {
                id: String::from("r_in"),
                input: r_in,
            },
            #[cfg(feature = "gui-egui")]
            egui_x: crate::common::EguiExtra::default(),
        }
    }
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
        let value = simulator.get_input_val(&self.r_in.input);
        // set output
        simulator.set_out_val(&self.id, "out", value);
        trace!("eval: register id {} in {:?}", self.id, value);
    }
}
