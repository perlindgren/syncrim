use crate::common::{
    Component, Id, Input, OutputType, Ports, SignalData, SignalSigned, SignalUnsigned, Simulator,
};
use log::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize)]
pub struct Add {
    id: Id,
    pub(crate) pos: (f32, f32),
    a_in: Input,
    b_in: Input,
}

#[typetag::serde]
impl Component for Add {
    fn to_(&self) {
        trace!("Add");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![&self.a_in, &self.b_in],
                OutputType::Combinatorial,
                vec!["out", "overflow"],
            ),
        )
    }

    // propagate addition to output
    fn clock(&self, simulator: &mut Simulator) {
        // get input values
        let a_in = u32::try_from(simulator.get_input_val(&self.a_in));
        let b_in = u32::try_from(simulator.get_input_val(&self.b_in));

        let (value, overflow) = match (&a_in, &b_in) {
            (Ok(a), Ok(b)) => {
                let (res, overflow) =
                    SignalSigned::overflowing_add(*a as SignalSigned, *b as SignalSigned);
                (
                    (res as SignalUnsigned).into(),
                    (overflow as SignalUnsigned).into(),
                )
            }
            _ => (SignalData::Unknown, SignalData::Unknown),
        };

        trace!(
            "eval Add a_in {:?}, b_in {:?}, value = {:?}, overflow = {:?}",
            a_in,
            b_in,
            value,
            overflow
        );

        // set output
        simulator.set_out_val(&self.id, "out", value);
        simulator.set_out_val(&self.id, "overflow", overflow);
    }
}

impl Add {
    pub fn new(id: &str, pos: (f32, f32), a_in: Input, b_in: Input) -> Self {
        Add {
            id: id.to_string(),
            pos,
            a_in,
            b_in,
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), a_in: Input, b_in: Input) -> Rc<Self> {
        Rc::new(Add::new(id, pos, a_in, b_in))
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
                }),
            ],
        };
        let mut simulator = Simulator::new(&cs);

        assert_eq!(simulator.cycle, 1);

        // outputs
        let add_val = &Input::new("add", "out");
        let add_overflow = &Input::new("add", "overflow");

        // reset
        assert_eq!(simulator.get_input_val(add_val), (0 + 0).into());
        assert_eq!(
            simulator.get_input_val(add_overflow),
            (false as SignalUnsigned).into()
        );

        println!("<setup for clock 2>");
        simulator.set_out_val("po1", "out", 42);
        simulator.set_out_val("po2", "out", 1337);
        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock();

        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 2);
        assert_eq!(simulator.get_input_val(add_val), (42 + 1337).into());
        assert_eq!(
            simulator.get_input_val(add_overflow),
            (false as SignalUnsigned).into()
        );

        // trigger positive overflow
        println!("<setup for clock 3>");
        simulator.set_out_val("po1", "out", SignalUnsigned::MAX / 2);
        simulator.set_out_val("po2", "out", 1);
        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 3);
        assert_eq!(
            simulator.get_input_val(add_val),
            (SignalUnsigned::MAX / 2 + 1).into()
        );
        assert_eq!(
            simulator.get_input_val(add_overflow),
            (true as SignalUnsigned).into()
        );
    }
}
