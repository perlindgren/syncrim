use crate::common::{Component, Id, Input, OutputType, Ports, Signal, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Mux {
    pub id: Id,
    pub pos: (f32, f32),
    pub select: Input,
    pub m_in: Vec<Input>,
}

#[typetag::serde]
impl Component for Mux {
    fn to_(&self) {
        trace!("mux");
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        let mut inputs = vec![self.select.clone()];
        let mut m = self.m_in.clone();
        inputs.append(&mut m);

        (
            self.id.clone(),
            Ports {
                inputs,
                out_type: OutputType::Combinatorial,
                outputs: vec!["out".into()],
            },
        )
    }

    // propagate selected input value to output
    fn clock(&self, simulator: &mut Simulator) {
        // get input value
        match simulator.get_input_val(&self.select) {
            Signal::Data(select) => {
                let select = select as usize;
                trace!("{} select {}", &self.id, select);
                let value = if select < self.m_in.len() {
                    simulator.get_input_val(&self.m_in[select])
                } else {
                    Signal::Unknown
                };

                // set output
                simulator.set_out_val(&self.id, "out", value);
            }
            _ => {
                trace!("{} select unknown", &self.id);
                simulator.set_out_val(&self.id, "out", Signal::Unknown);
            }
        }
    }
}
