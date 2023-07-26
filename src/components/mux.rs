use crate::common::{
    Component, Id, Input, OutputType, Ports, SignalData, SignalUnsigned, Simulator,
};
use log::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

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
        let select: SignalData = simulator.get_input_val(&self.select);

        let value = if let Ok(select) = TryInto::<SignalUnsigned>::try_into(select) {
            let select = select as usize;
            trace!("select {}", select);
            if select < self.m_in.len() {
                simulator.get_input_val(&self.m_in[select])
            } else {
                SignalData::Unknown
            }
        } else {
            SignalData::Unknown
        };

        // set output
        simulator.set_out_val(&self.id, "out", value);
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
