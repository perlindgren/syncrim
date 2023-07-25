use crate::common::{Component, Id, Input, OutputType, Ports};
use log::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Wire {
    pub id: Id,
    pub pos: Vec<(f32, f32)>,
    pub input: Input,
}

#[typetag::serde]
impl Component for Wire {
    fn to_(&self) {
        trace!("Wire");
    }
    fn to_string(&self)->String{"".to_string()}
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                // Wires take one input
                vec![&self.input],
                OutputType::Combinatorial,
                // No output value
                vec![],
            ),
        )
    }
}
