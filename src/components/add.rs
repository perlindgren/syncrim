use crate::common::{
    Component, EguiExtra, Id, Input, InputId, OutputType, Ports, Signal, SignedSignal, Simulator,
};
use log::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Add {
    pub id: Id,
    pub pos: (f32, f32),
    pub a_in: InputId,
    pub b_in: InputId,

    #[cfg(feature = "gui-egui")]
    #[serde(skip_serializing)]
    pub egui_x: EguiExtra,
}

impl Add {
    pub fn new(id: String, pos: (f32, f32), a_in: Input, b_in: Input) -> Self {
        Add {
            id: id.clone(),
            pos,
            a_in: InputId {
                id: String::from("a_in"),
                input: a_in,
            },
            b_in: InputId {
                id: String::from("b_in"),
                input: b_in,
            },
            egui_x: EguiExtra {
                properties_window: false,
                id_tmp: id,
            },
        }
    }
}

#[typetag::serde]
impl Component for Add {
    fn to_(&self) {
        trace!("Add");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![&self.a_in, &self.b_in],
                OutputType::Combinatorial,
                vec!["out", "overflow"],
            ),
        )
    }

    // propagate addition to output
    fn clock(&self, simulator: &mut Simulator) {
        // get input values
        let a_in = simulator.get_input_val(&self.a_in.input);
        let b_in = simulator.get_input_val(&self.b_in.input);

        // compute addition (notice will panic on overflow)
        let (value, overflow) =
            SignedSignal::overflowing_add(a_in as SignedSignal, b_in as SignedSignal);

        trace!(
            "eval Add a_in {}, b_in {}, value = {}, overflow = {}",
            a_in,
            b_in,
            value,
            overflow
        );

        // set output
        simulator.set_out_val(&self.id, "out", value as Signal);
        simulator.set_out_val(&self.id, "overflow", Signal::from(overflow));
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            "a_in" => self.a_in.input = new_input,
            "b_in" => self.b_in.input = new_input,
            _ => (),
        }
    }
}
