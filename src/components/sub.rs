// use std::fmt::Alignment;
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

pub const SUB_A_IN_ID: &str = "sub_a_in";
pub const SUB_B_IN_ID: &str = "sub_b_in";

pub const SUB_OUT_ID: &str = "out";

#[derive(Serialize, Deserialize, Clone)]
pub struct Sub {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) a_in: Input,
    pub(crate) b_in: Input,
}

#[typetag::serde]
impl Component for Sub {
    fn to_(&self) {
        trace!("Subtractor");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(Sub {
            id: "dummy".to_string(),
            pos: (0.0, 0.0),
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
                        port_id: SUB_A_IN_ID.to_string(),
                        input: self.a_in.clone(),
                    },
                    &InputPort {
                        port_id: SUB_B_IN_ID.to_string(),
                        input: self.b_in.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![SUB_OUT_ID],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            SUB_A_IN_ID => self.a_in = new_input,
            SUB_B_IN_ID => self.b_in = new_input,
            _ => {}
        }
    }

    // propagate sign extension to output
    // TODO: always extend to Signal size? (it should not matter and should be slightly cheaper)
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input values
        let a: u32 = simulator.get_input_value(&self.a_in).try_into().unwrap();
        let b: u32 = simulator.get_input_value(&self.b_in).try_into().unwrap();

        simulator.set_out_value(
            &self.id,
            SUB_OUT_ID,
            SignalValue::Data(((a as i32) - (b as i32)) as u32),
        );
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Sub {
    pub fn new(id: &str, pos: (f32, f32), a_in: Input, b_in: Input) -> Self {
        Sub {
            id: id.to_string(),
            pos,
            a_in,
            b_in,
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), a_in: Input, b_in: Input) -> Rc<Self> {
        Rc::new(Sub::new(id, pos, a_in, b_in))
    }
}
