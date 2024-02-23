#[cfg(feature = "gui-egui")]
use crate::common::EguiComponent;
use crate::common::{Component, Id, Input, InputPort, OutputType, Ports};
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;

pub const CROSS_IN_ID: &str = "in";

#[derive(Serialize, Deserialize, Clone)]
pub struct Cross {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) input: Input,
}

#[typetag::serde]
impl Component for Cross {
    fn to_(&self) {
        trace!("Cross");
    }

    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(Cross {
            id: id.to_string(),
            pos: (pos.0, pos.1),
            input: dummy_input.clone(),
        }))
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                // Probes take one input
                vec![&InputPort {
                    port_id: CROSS_IN_ID.to_string(),
                    input: self.input.clone(),
                }],
                OutputType::Combinatorial,
                // No output value
                vec![],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        if target_port_id.as_str() == CROSS_IN_ID {
            self.input = new_input
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Cross {
    pub fn new(id: &str, pos: (f32, f32), input: Input) -> Self {
        Cross {
            id: id.to_string(),
            pos,
            input,
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), input: Input) -> Rc<Self> {
        Rc::new(Cross::new(id, pos, input))
    }
}
