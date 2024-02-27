#[cfg(feature = "gui-egui")]
use crate::common::EguiComponent;
use crate::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, SignalUnsigned, SignalValue,
    Simulator,
};
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;

pub const MUX_SELECT_ID: &str = "select";
pub const MUX_TEMPLATE_ID: &str = "in";
pub const MUX_OUT_ID: &str = "out";

#[derive(Serialize, Deserialize, Clone)]
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
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(Mux {
            id: id.to_string(),
            pos: (pos.0, pos.1),
            select: dummy_input.clone(),
            m_in: vec![dummy_input.clone(), dummy_input.clone()],
        }))
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        let mut inputs: Vec<InputPort> = Vec::with_capacity(self.m_in.len() + 1);
        inputs.push(InputPort {
            port_id: MUX_SELECT_ID.to_string(),
            input: self.select.clone(),
        });
        for (i, item) in self.m_in.iter().enumerate() {
            inputs.push(InputPort {
                port_id: format!("{}{}", MUX_TEMPLATE_ID, i),
                input: item.clone(),
            });
        }

        (
            self.id.clone(),
            Ports {
                inputs,
                out_type: OutputType::Combinatorial,
                outputs: vec![MUX_OUT_ID.to_string()],
            },
        )
    }

    // propagate selected input value to output
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input value
        let select: SignalValue = simulator.get_input_value(&self.select);
        trace!("-----------{}------------", self.id);
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
        trace!("-----------------value:{:?}, end---------------", value);
        // set output
        simulator.set_out_value(&self.id, "out", value);
        res
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        let target_port_id = target_port_id.as_str();
        if target_port_id == MUX_SELECT_ID {
            self.select = new_input;
            return;
        }
        for i in 0..=self.m_in.len() - 1 {
            if target_port_id == format!("{}{}", MUX_TEMPLATE_ID, i) {
                self.m_in[i] = new_input;
                return;
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
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
