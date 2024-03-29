use crate::common::{
    Component, Condition, Id, Input, OutputType, Ports, SignalUnsigned, SignalValue, Simulator,
};
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
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input value
        let select: SignalValue = simulator.get_input_value(&self.select);

        let (value, res) = if let Ok(select) = TryInto::<SignalUnsigned>::try_into(select) {
            let select = select as usize;
            trace!("select {}", select);
            if select < self.m_in.len() {
                (simulator.get_input_value(&self.m_in[select]), Ok(()))
            } else {
                (
                    SignalValue::Unknown,
                    Err(Condition::Error(format!("select {} out of range", select))),
                )
            }
        } else {
            (
                SignalValue::Unknown,
                Err(Condition::Warning("select unknown".to_string())),
            )
        };

        // set output
        simulator.set_out_value(&self.id, "out", value);
        res
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
