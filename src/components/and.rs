#[cfg(feature = "gui-egui")]
use crate::common::EguiComponent;
use crate::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, SignalSigned, SignalUnsigned,
    SignalValue, Simulator,
};
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;
pub const AND_A_IN_ID: &str = "a_in";
pub const AND_B_IN_ID: &str = "b_in";

pub const AND_OUT_ID: &str = "and_out";

#[derive(Serialize, Deserialize, Clone)]
pub struct And {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) a_in: Input,
    pub(crate) b_in: Input,
}

#[typetag::serde]
impl Component for And {
    fn to_(&self) {
        trace!("And");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(And {
            id: id.to_string(),
            pos: (pos.0, pos.1),
            a_in: dummy_input.clone(),
            b_in: dummy_input.clone(),
        }))
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: AND_A_IN_ID.to_string(),
                        input: self.a_in.clone(),
                    },
                    &InputPort {
                        port_id: AND_B_IN_ID.to_string(),
                        input: self.b_in.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![AND_OUT_ID],
            ),
        )
    }

    // propagate addition to output
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input values
        let a_in: u32 = simulator.get_input_value(&self.a_in).try_into().unwrap();
        let b_in: u32 = simulator.get_input_value(&self.b_in).try_into().unwrap();

        let result: u32 = a_in & b_in;

        simulator.set_out_value(&self.id, AND_OUT_ID, SignalValue::Data(result));
        Ok(())
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            AND_A_IN_ID => self.a_in = new_input,
            AND_B_IN_ID => self.b_in = new_input,
            _ => (),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl And {
    pub fn new(id: &str, pos: (f32, f32), a_in: Input, b_in: Input) -> Self {
        And {
            id: id.to_string(),
            pos,
            a_in,
            b_in,
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), a_in: Input, b_in: Input) -> Rc<Self> {
        Rc::new(And::new(id, pos, a_in, b_in))
    }
}
