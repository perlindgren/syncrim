#[cfg(feature = "gui-egui")]
use crate::common::EguiComponent;
use crate::common::{Component, Condition, Id, OutputType, Ports, Signal, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
use std::{convert::Into, rc::Rc};
pub const CONSTANT_OUT_ID: &str = "out";
use std::any::Any;
#[derive(Serialize, Deserialize, Clone)]
pub struct Constant {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) value: Signal,
}

#[typetag::serde]
impl Component for Constant {
    fn to_(&self) {
        trace!("constant {:?}", self.value);
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        Box::new(Rc::new(Constant {
            id: id.to_string(),
            pos: (pos.0, pos.1),
            value: 0.into(),
        }))
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                // Constants do not take any inputs
                vec![],
                OutputType::Combinatorial,
                vec![CONSTANT_OUT_ID],
            ),
        )
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        simulator.set_out_value(&self.id, CONSTANT_OUT_ID, self.value.get_value());
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Constant {
    pub fn new(id: &str, pos: (f32, f32), value: impl Into<Signal>) -> Self {
        Constant {
            id: id.to_string(),
            pos,
            value: value.into(),
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), value: impl Into<Signal>) -> Rc<Self> {
        Rc::new(Constant::new(id, pos, value))
    }
}
