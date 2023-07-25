// use std::fmt::Alignment;
use crate::common::{
    Component, Id, Input, InputPort, OutputType, Ports, Signal, SignalSigned, SignalUnsigned,
    Simulator,
};
use log::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct Sext {
    pub id: Id,
    pub pos: (f32, f32),
    pub sext_in: InputPort,
    pub in_size: u32,
    pub out_size: u32,

    #[cfg(feature = "gui-egui")]
    #[serde(skip)]
    pub egui_x: crate::common::EguiExtra,
}
impl Sext {
    pub fn new(id: &str, pos: (f32, f32), sext_in: Input, in_size: u32, out_size: u32) -> Self {
        Sext {
            id: id.to_string(),
            pos,
            sext_in: InputPort {
                port_id: String::from("sext_in"),
                input: sext_in,
            },
            in_size,
            out_size,
            #[cfg(feature = "gui-egui")]
            egui_x: crate::common::EguiExtra::default(),
        }
    }
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
        let mut value: SignalUnsigned = simulator
            .get_input_val(&self.sext_in.input)
            .try_into()
            .unwrap();

        let to_sext = self.out_size - self.in_size; // Amount to be arithmetically shifted
        let to_shl = SignalUnsigned::BITS - self.in_size; // To move input to MSB
        let to_shr = to_shl - to_sext; // To shift the result back to LSB

        value <<= to_shl;
        value = ((value as SignalSigned) >> to_sext) as SignalUnsigned;
        value >>= to_shr;

        // set output
        simulator.set_out_val(&self.id, "out", Signal::Data(value));
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::{common::ComponentStore, components::ProbeOut};
    use std::rc::Rc;

    #[test]
    fn test_sext() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("po")),
                Rc::new(Sext::new(
                    "sext32",
                    (0.0, 0.0),
                    Input::new("po", "out"),
                    4,
                    32,
                )),
                Rc::new(Sext::new(
                    "sext16",
                    (0.0, 0.0),
                    Input::new("po", "out"),
                    4,
                    16,
                )),
            ],
        };

        let mut simulator = Simulator::new(cs);

        assert_eq!(simulator.cycle, 1);

        // outputs
        let sext32_out = &Input::new("sext32", "out");
        let sext16_out = &Input::new("sext16", "out");

        // reset
        assert_eq!(simulator.get_input_val(sext32_out), 0.into());
        assert_eq!(simulator.get_input_val(sext16_out), 0.into());

        // Sign-extended
        println!("<setup for clock 2>");
        simulator.set_out_val("po", "out", 0b1111);
        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 2);
        assert_eq!(simulator.get_input_val(sext32_out), 0xFFFFFFFF.into());
        assert_eq!(simulator.get_input_val(sext16_out), 0xFFFF.into());

        // Zero-extended
        println!("<setup for clock 3>");
        simulator.set_out_val("po", "out", 0b111);
        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 3);
        assert_eq!(simulator.get_input_val(sext32_out), 0b111.into());
        assert_eq!(simulator.get_input_val(sext16_out), 0b111.into());

        // Unclean upper bits
        println!("<setup for clock 4>");
        simulator.set_out_val("po", "out", 0b10111);
        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 4);
        assert_eq!(simulator.get_input_val(sext32_out), 0b111.into());
        assert_eq!(simulator.get_input_val(sext16_out), 0b111.into());
    }
}
