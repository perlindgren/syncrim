#[cfg(feature = "gui-egui")]
use crate::common::EguiComponent;
use crate::common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;

pub const PASS_THROUGH_IN_ID: &str = "pass_through_in";

pub const PASS_THROUGH_OUT_ID: &str = "pass_through_out";

#[derive(Serialize, Deserialize, Clone)]
pub struct PassThrough {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) input: Input,
}

#[typetag::serde]
impl Component for PassThrough {
    fn to_(&self) {
        trace!("pass_through");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(PassThrough {
            id: id.to_string(),
            pos: (pos.0, pos.1),
            input: dummy_input.clone(),
        }))
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                // Vector of inputs
                vec![&InputPort {
                    port_id: PASS_THROUGH_IN_ID.to_string(),
                    input: self.input.clone(),
                }],
                OutputType::Combinatorial,
                vec![PASS_THROUGH_OUT_ID],
            ),
        )
    }

    // propagate input value to output
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input value
        let value = simulator.get_input_value_mut(self.id.clone(), &self.input);
        // set output
        simulator.set_out_value(&self.id, PASS_THROUGH_OUT_ID, value);
        Ok(())
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        if target_port_id == PASS_THROUGH_IN_ID {
            self.input = new_input;
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PassThrough {
    pub fn new(id: &str, pos: (f32, f32), input: Input) -> Self {
        PassThrough {
            id: id.to_string(),
            pos,
            input,
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), input: Input) -> Rc<Self> {
        Rc::new(PassThrough::new(id, pos, input))
    }
}
