use crate::{
    common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Signal, Simulator},
    signal::SignalValue,
};
use log::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

pub const PROBE_ASSERT_IN_ID: &str = "in";

#[derive(Serialize, Deserialize)]
pub struct ProbeAssert {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) input: Input,
    pub(crate) values: Vec<Signal>,
}

#[typetag::serde]
impl Component for ProbeAssert {
    fn to_(&self) {
        trace!("ProbeAssert {:?}", self.values);
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![&InputPort {
                    port_id: PROBE_ASSERT_IN_ID.to_string(),
                    input: self.input.clone(),
                }],
                OutputType::Combinatorial,
                vec![],
            ),
        )
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        trace!("-- cycle {} --", simulator.cycle);
        let lhs = simulator.get_input_value(&self.input);
        let rhs = match self.values.get(simulator.cycle) {
            Some(rhs) => rhs.get_value(),
            _ => SignalValue::Unknown,
        };

        // the assertion is checked only in test mode
        #[cfg(test)]
        assert_eq!(lhs, rhs);
        if lhs == rhs {
            Ok(())
        } else {
            Err(Condition::Assert(format!(
                "assertion failed {:?} != {:?}",
                lhs, rhs
            )))
        }
    }

    // notice we don't implement `un_clock` since the state is already kept in history
}

impl ProbeAssert {
    pub fn new(id: &str, pos: (f32, f32), input: Input, values: Vec<impl Into<Signal>>) -> Self {
        ProbeAssert {
            id: id.to_string(),
            pos,
            input,
            values: values.into_iter().map(|v| v.into()).collect(),
        }
    }

    pub fn rc_new(
        id: &str,
        pos: (f32, f32),
        input: Input,
        values: Vec<impl Into<Signal>>,
    ) -> Rc<Self> {
        Rc::new(ProbeAssert::new(id, pos, input, values))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        common::{ComponentStore, Input},
        components::ProbeStim,
    };

    #[test]
    fn test_probe_stim() {
        let cs = ComponentStore {
            store: vec![
                ProbeStim::rc_new("stim", (0.0, 0.0), vec![0, 1, 2]),
                ProbeAssert::rc_new(
                    "assert",
                    (0.0, 0.0),
                    Input::new("stim", "out"),
                    vec![0, 1, 2],
                ),
            ],
        };

        let mut simulator = Simulator::new(cs);
        // output
        let out = &Input::new("stim", "out");

        // reset
        println!("<reset>");
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 1);
        assert_eq!(simulator.get_input_value(out), 0.into());

        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 2);
        assert_eq!(simulator.get_input_value(out), 1.into());

        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 3);
        assert_eq!(simulator.get_input_value(out), 2.into());

        println!("<un_clock>");
        simulator.un_clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 2);
        assert_eq!(simulator.get_input_value(out), 1.into());

        println!("<un_clock>");
        simulator.un_clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 1);
        assert_eq!(simulator.get_input_value(out), 0.into());

        println!("<un_clock (already in reset)>");
        simulator.un_clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 1);
        assert_eq!(simulator.get_input_value(out), 0.into());

        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 2);
        assert_eq!(simulator.get_input_value(out), 1.into());

        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 3);
        assert_eq!(simulator.get_input_value(out), 2.into());

        println!("<reset>");
        simulator.reset();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 1);
        assert_eq!(simulator.get_input_value(out), 0.into());
    }

    #[test]
    #[should_panic]
    fn test_probe_stim_fail() {
        let cs = ComponentStore {
            store: vec![
                ProbeStim::rc_new("stim", (0.0, 0.0), vec![0, 1, 2]),
                ProbeAssert::rc_new(
                    "assert",
                    (0.0, 0.0),
                    Input::new("stim", "out"),
                    vec![0, 0, 2],
                ),
            ],
        };

        let mut simulator = Simulator::new(cs);
        // output
        let out = &Input::new("stim", "out");

        // reset
        println!("<reset>");
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 1);
        assert_eq!(simulator.get_input_value(out), 0.into());

        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 2);
        assert_eq!(simulator.get_input_value(out), 1.into());

        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 3);
        assert_eq!(simulator.get_input_value(out), 2.into());

        println!("<un_clock>");
        simulator.un_clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 2);
        assert_eq!(simulator.get_input_value(out), 1.into());

        println!("<un_clock>");
        simulator.un_clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 1);
        assert_eq!(simulator.get_input_value(out), 0.into());

        println!("<un_clock (already in reset)>");
        simulator.un_clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 1);
        assert_eq!(simulator.get_input_value(out), 0.into());

        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 2);
        assert_eq!(simulator.get_input_value(out), 1.into());

        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 3);
        assert_eq!(simulator.get_input_value(out), 2.into());

        println!("<reset>");
        simulator.reset();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 1);
        assert_eq!(simulator.get_input_value(out), 0.into());
    }
}
