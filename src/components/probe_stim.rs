use crate::common::{Component, Condition, Id, OutputType, Ports, Signal, SignalValue, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProbeStim {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) values: Vec<Signal>,
}

#[typetag::serde]
impl Component for ProbeStim {
    fn to_(&self) {
        trace!("constant {:?}", self.values);
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                // ProbeStim do not take any inputs
                vec![],
                OutputType::Combinatorial,
                vec!["out"],
            ),
        )
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        trace!("-- cycle {} --", simulator.cycle);
        let (out, res) = if let Some(signal) = self.values.get(simulator.cycle) {
            (signal.get_value(), Ok(()))
        } else {
            (
                SignalValue::Unknown,
                Err(Condition::Warning(format!(
                    "No stim value defined for cycle {}",
                    simulator.cycle
                ))),
            )
        };
        simulator.set_out_value(&self.id, "out", out);
        res
    }

    // notice we don't implement `un_clock` since the state is already kept in history
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ProbeStim {
    pub fn new(id: &str, pos: (f32, f32), values: Vec<impl Into<Signal>>) -> Self {
        ProbeStim {
            id: id.to_string(),
            pos,
            values: values.into_iter().map(|v| v.into()).collect(),
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), values: Vec<impl Into<Signal>>) -> Rc<Self> {
        Rc::new(ProbeStim::new(id, pos, values))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::{ComponentStore, Input};

    #[test]
    fn test_probe_stim() {
        let cs = ComponentStore {
            store: vec![ProbeStim::rc_new("stim", (0.0, 0.0), vec![0, 1, 2, 3])],
        };

        let mut simulator = Simulator::new(cs).unwrap();
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
