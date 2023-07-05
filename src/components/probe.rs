use crate::common::{Component, Input, OutputType, Ports};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Probe {
    pub id: String,
    pub pos: (f32, f32),
    pub input: Input,
}

#[typetag::serde]
impl Component for Probe {
    fn to_(&self) {
        println!("Probe");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                // Probes take one input
                inputs: vec![self.input.clone()],
                out_type: OutputType::Combinatorial,
                // No output value
                outputs: vec![],
            },
        )
    }
}
