use crate::common::{Component, Output, OutputType, Ports, Signal, Simulator};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Constant {
    pub id: String,
    pub pos: (f32, f32),
    pub value: Signal, // perhaps vector here ... not sure
}

#[typetag::serde]
impl Component for Constant {
    fn to_(&self) {
        println!("constant {:?}", self.value);
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                // Constants do not take any inputs
                inputs: vec![],
                out_type: OutputType::Combinatorial,
                // Single output value
                outputs: vec![Output::Constant(self.value)],
            },
        )
    }

    fn evaluate(&self, simulator: &mut Simulator) {
        simulator.set_id_index(&self.id, 0, self.value);
    }
}
