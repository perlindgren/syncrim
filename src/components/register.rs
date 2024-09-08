#[cfg(feature = "gui-egui")]
use crate::common::EguiComponent;
use crate::common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;

pub const REGISTER_R_IN_ID: &str = "r_in";

pub const REGISTER_OUT_ID: &str = "out";

#[derive(Serialize, Deserialize, Clone)]
pub struct Register {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) r_in: Input,
}

#[typetag::serde]
impl Component for Register {
    fn to_(&self) {
        trace!("register");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(Register {
            id: id.to_string(),
            pos: (pos.0, pos.1),
            r_in: dummy_input.clone(),
        }))
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                // Vector of inputs
                vec![&InputPort {
                    port_id: REGISTER_R_IN_ID.to_string(),
                    input: self.r_in.clone(),
                }],
                OutputType::Sequential,
                vec![REGISTER_OUT_ID],
            ),
        )
    }

    // propagate input value to output
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input value
        let value = simulator.get_input_value_mut(&self.r_in);
        // set output
        simulator.set_out_value(&self.id, "out", value);
        trace!("eval: register id {} in {:?}", self.id, value);
        Ok(())
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        if target_port_id == REGISTER_R_IN_ID {
            self.r_in = new_input;
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Register {
    pub fn new(id: &str, pos: (f32, f32), r_in: Input) -> Self {
        Register {
            id: id.to_string(),
            pos,
            r_in,
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), r_in: Input) -> Rc<Self> {
        Rc::new(Register::new(id, pos, r_in))
    }
}
