use crate::common::{Component, Id, Input, OutputType, Ports, Signal, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize)]
pub struct Mux {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) select: Input,
    pub(crate) m_in: Vec<Input>,
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

impl Mux {
    pub fn new(id: &str, pos: (f32, f32), select: Input, m_in: Vec<Input>) -> Self {
        Mux {
            id: id.to_string(),
            pos,
            select,
            m_in,
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), select: Input, m_in: Vec<Input>) -> Rc<Self> {
        Rc::new(Mux::new(id, pos, select, m_in))
    }
}
