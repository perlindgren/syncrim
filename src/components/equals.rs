//TODO: add so it can take undefiend number of inputs
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
pub const EQUAL_A_IN_ID: &str = "a_in";
pub const EQUAL_B_IN_ID: &str = "b_in";

pub const EQUAL_OUT_ID: &str = "equals_out";

#[derive(Serialize, Deserialize, Clone)]
pub struct Equal {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) a_in: Input,
    pub(crate) b_in: Input,
}

#[typetag::serde]
impl Component for Equal {
    fn to_(&self) {
        trace!("Equal");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: EQUAL_A_IN_ID.to_string(),
                        input: self.a_in.clone(),
                    },
                    &InputPort {
                        port_id: EQUAL_B_IN_ID.to_string(),
                        input: self.b_in.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![EQUAL_OUT_ID],
            ),
        )
    }

    // propagate addition to output
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input values
        let a_in: u32 = simulator.get_input_value(&self.a_in).try_into().unwrap();
        let b_in: u32 = simulator.get_input_value(&self.b_in).try_into().unwrap();

        let result: u32 = (a_in == b_in) as u32;

        simulator.set_out_value(&self.id, EQUAL_OUT_ID, SignalValue::Data(result));
        Ok(())
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            EQUAL_A_IN_ID => self.a_in = new_input,
            EQUAL_B_IN_ID => self.b_in = new_input,
            _ => (),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Equal {
    pub fn new(id: &str, pos: (f32, f32), a_in: Input, b_in: Input) -> Self {
        Equal {
            id: id.to_string(),
            pos,
            a_in,
            b_in,
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), a_in: Input, b_in: Input) -> Rc<Self> {
        Rc::new(Equal::new(id, pos, a_in, b_in))
    }
}
