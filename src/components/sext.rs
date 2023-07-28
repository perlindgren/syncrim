// use std::fmt::Alignment;
use crate::{
    common::{Component, Id, Input, OutputType, Ports, SignalSigned, SignalUnsigned, Simulator},
    signal::SignalValue,
};
use log::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
#[derive(Serialize, Deserialize)]
pub struct Sext {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) sext_in: Input,
    pub(crate) in_size: u32,
    pub(crate) out_size: u32,
}

#[typetag::serde]
impl Component for Sext {
    fn to_(&self) {
        trace!("Sign Extension");
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(vec![&self.sext_in], OutputType::Combinatorial, vec!["out"]),
        )
    }

    // propagate sign extension to output
    // TODO: always extend to Signal size? (it should not matter and should be slightly cheaper)
    fn clock(&self, simulator: &mut Simulator) {
        assert!(
            self.out_size <= SignalUnsigned::BITS,
            "{}: Output size {} is larger than maximum size {}",
            self.id,
            self.out_size,
            SignalUnsigned::BITS
        );

        // get input values
        match simulator.get_input_val(&self.sext_in) {
            SignalValue::Data(mut value) => {
                let to_sext = self.out_size - self.in_size; // Amount to be arithmetically shifted
                let to_shl = SignalUnsigned::BITS - self.in_size; // To move input to MSB
                let to_shr = to_shl - to_sext; // To shift the result back to LSB

                value <<= to_shl;
                value = ((value as SignalSigned) >> to_sext) as SignalUnsigned;
                value >>= to_shr;

                // set output
                simulator.set_out_value(&self.id, "out", SignalValue::Data(value));
            }
            _ => {
                simulator.set_out_value(&self.id, "out", SignalValue::Unknown);
                trace!("{} unknown input", self.id);
            }
        }
    }
}

impl Sext {
    pub fn new(id: &str, pos: (f32, f32), sext_in: Input, in_size: u32, out_size: u32) -> Self {
        Sext {
            id: id.to_string(),
            pos,
            sext_in,
            in_size,
            out_size,
        }
    }

    pub fn rc_new(
        id: &str,
        pos: (f32, f32),
        sext_in: Input,
        in_size: u32,
        out_size: u32,
    ) -> Rc<Self> {
        Rc::new(Sext::new(id, pos, sext_in, in_size, out_size))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{common::ComponentStore, components::ProbeOut};

    #[test]
    fn test_sext() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("po")),
                Rc::new(Sext {
                    id: "sext32".to_string(),
                    pos: (0.0, 0.0),
                    sext_in: Input::new("po", "out"),
                    in_size: 4,
                    out_size: 32,
                }),
                Rc::new(Sext {
                    id: "sext16".to_string(),
                    pos: (0.0, 0.0),
                    sext_in: Input::new("po", "out"),
                    in_size: 4,
                    out_size: 16,
                }),
            ],
        };

        let mut simulator = Simulator::new(&cs);

        assert_eq!(simulator.cycle, 1);

        // outputs
        let sext32_out = &Input::new("sext32", "out");
        let sext16_out = &Input::new("sext16", "out");

        // reset
        assert_eq!(simulator.get_input_val(sext32_out), 0.into());
        assert_eq!(simulator.get_input_val(sext16_out), 0.into());

        // Sign-extended
        println!("<setup for clock 2>");
        simulator.set_out_value("po", "out", 0b1111);
        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 2);
        assert_eq!(simulator.get_input_val(sext32_out), 0xFFFFFFFF.into());
        assert_eq!(simulator.get_input_val(sext16_out), 0xFFFF.into());

        // Zero-extended
        println!("<setup for clock 3>");
        simulator.set_out_value("po", "out", 0b111);
        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 3);
        assert_eq!(simulator.get_input_val(sext32_out), 0b111.into());
        assert_eq!(simulator.get_input_val(sext16_out), 0b111.into());

        // Unclean upper bits
        println!("<setup for clock 4>");
        simulator.set_out_value("po", "out", 0b10111);
        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 4);
        assert_eq!(simulator.get_input_val(sext32_out), 0b111.into());
        assert_eq!(simulator.get_input_val(sext16_out), 0b111.into());
    }
}
