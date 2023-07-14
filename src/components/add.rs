use crate::common::{Component, Id, Input, OutputType, Ports, Signal, SignedSignal, Simulator};
use log::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Add {
    pub id: Id,
    pub pos: (f32, f32),
    pub a_in: Input,
    pub b_in: Input,
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
    fn evaluate(&self, simulator: &mut Simulator) {
        // get input values
        let a_in = simulator.get_input_val(&self.a_in);
        let b_in = simulator.get_input_val(&self.b_in);

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
}
