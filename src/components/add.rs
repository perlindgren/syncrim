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
pub const ADD_A_IN_ID: &str = "a_in";
pub const ADD_B_IN_ID: &str = "b_in";

pub const ADD_OUT_ID: &str = "add_out";
pub const ADD_OVERFLOW_ID: &str = "overflow";

#[derive(Serialize, Deserialize, Clone)]
pub struct Add {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) a_in: Input,
    pub(crate) b_in: Input,
    pub(crate) scale: f32,
}

#[typetag::serde]
impl Component for Add {
    fn to_(&self) {
        trace!("Add");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(Add {
            id: id.to_string(),
            pos: (pos.0, pos.1),
            a_in: dummy_input.clone(),
            b_in: dummy_input.clone(),
            scale: 1.0,
        }))
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: ADD_A_IN_ID.to_string(),
                        input: self.a_in.clone(),
                    },
                    &InputPort {
                        port_id: ADD_B_IN_ID.to_string(),
                        input: self.b_in.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![ADD_OUT_ID, ADD_OVERFLOW_ID],
            ),
        )
    }

    // propagate addition to output
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input values
        let a_in = u32::try_from(simulator.get_input_value_mut(self.id.clone(), &self.a_in));
        let b_in = u32::try_from(simulator.get_input_value_mut(self.id.clone(), &self.b_in));

        let (value, overflow, res) = match (&a_in, &b_in) {
            (Ok(a), Ok(b)) => {
                let (res, overflow) =
                    SignalSigned::overflowing_add(*a as SignalSigned, *b as SignalSigned);
                (
                    (res as SignalUnsigned).into(),
                    (overflow as SignalUnsigned).into(),
                    Ok(()),
                )
            }
            _ => (
                SignalValue::Unknown,
                SignalValue::Unknown,
                Err(Condition::Warning("Unknown".to_string())),
            ),
        };

        trace!(
            "eval Add a_in {:?}, b_in {:?}, value = {:?}, overflow = {:?}",
            a_in,
            b_in,
            value,
            overflow
        );

        // set output
        simulator.set_out_value(&self.id, ADD_OUT_ID, value);
        simulator.set_out_value(&self.id, ADD_OVERFLOW_ID, overflow);
        res
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            ADD_A_IN_ID => self.a_in = new_input,
            ADD_B_IN_ID => self.b_in = new_input,
            _ => (),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Add {
    pub fn new(id: &str, pos: (f32, f32), a_in: Input, b_in: Input) -> Self {
        Add {
            id: id.to_string(),
            pos,
            a_in,
            b_in,
            scale: 1.0,
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), a_in: Input, b_in: Input) -> Rc<Self> {
        Rc::new(Add::new(id, pos, a_in, b_in))
    }

    pub fn rc_new_with_scale(
        id: &str,
        pos: (f32, f32),
        a_in: Input,
        b_in: Input,
        scale: f32,
    ) -> Rc<Self> {
        Rc::new(Add {
            id: id.to_string(),
            pos,
            a_in,
            b_in,
            scale,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::{
        common::{ComponentStore, Input, SignalUnsigned, Simulator},
        components::ProbeOut,
    };
    use std::rc::Rc;

    #[test]
    fn test_add() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("po1")),
                Rc::new(ProbeOut::new("po2")),
                Rc::new(Add {
                    id: "add".to_string(),
                    pos: (0.0, 0.0),
                    a_in: Input::new("po1", "out"),
                    b_in: Input::new("po2", "out"),
                    scale: 1.0,
                }),
            ],
        };
        let mut simulator = Simulator::new(cs).unwrap();

        assert_eq!(simulator.cycle, 1);

        // outputs
        let add_val = &Input::new("add", "out");
        let add_overflow = &Input::new("add", "overflow");

        // reset
        assert_eq!(simulator.get_input_value(add_val), (0 + 0).into());
        assert_eq!(
            simulator.get_input_value(add_overflow),
            (false as SignalUnsigned).into()
        );

        println!("<setup for clock 2>");
        simulator.set_out_value("po1", "out", 42);
        simulator.set_out_value("po2", "out", 1337);
        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock();

        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 2);
        assert_eq!(simulator.get_input_value(add_val), (42 + 1337).into());
        assert_eq!(
            simulator.get_input_value(add_overflow),
            (false as SignalUnsigned).into()
        );

        // trigger positive overflow
        println!("<setup for clock 3>");
        simulator.set_out_value("po1", "out", SignalUnsigned::MAX / 2);
        simulator.set_out_value("po2", "out", 1);
        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 3);
        assert_eq!(
            simulator.get_input_value(add_val),
            (SignalUnsigned::MAX / 2 + 1).into()
        );
        assert_eq!(
            simulator.get_input_value(add_overflow),
            (true as SignalUnsigned).into()
        );
    }
}
