use crate::common::{Component, Output, OutputType, Ports};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProbeOut {
    pub id: String,
}

#[typetag::serde]
impl Component for ProbeOut {
    fn to_(&self) {
        println!("ProbeOut");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                // Constants do not take any inputs
                inputs: vec![],
                out_type: OutputType::Combinatorial,
                // Single output value
                outputs: vec![Output::Function],
            },
        )
    }
}
