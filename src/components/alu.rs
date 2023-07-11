use crate::common::{Component, Input, Output, OutputType, Ports, Signal, SignedSignal, Simulator};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Alu {
    pub id: String,
    pub pos: (f32, f32),
    pub a_in: Input,
    pub b_in: Input,
    pub op: Input,
}

#[typetag::serde]
impl Component for Alu {
    fn to_(&self) {
        println!("Alu");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.a_in.clone(), self.b_in.clone(), self.op.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec![Output::Function; 2],
            },
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

        println!(
            "eval Add a_in {}, b_in {}, value = {}, overflow = {}",
            a_in, b_in, value, overflow
        );

        // set output
        simulator.set_id_index(&self.id, 0, value as Signal);
        simulator.set_id_index(&self.id, 1, Signal::from(overflow));
    }
}
