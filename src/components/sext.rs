// use std::fmt::Alignment;
use crate::common::{Component, Input, Output, OutputType, Ports, Signal, Simulator};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Sext {
    pub id: String,
    pub pos: (f32, f32),
    pub sext_in: Input,
    pub in_size: u8,
    pub out_size: u8,
}

#[typetag::serde]
impl Component for Sext {
    fn to_(&self) {
        println!("Sign Extension");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.sext_in.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec![Output::Function],
            },
        )
    }

    // propagate sign extension to output
    // TODO: always extend to Signal size? (it should not matter and should be slightly cheaper)
    fn evaluate(&self, simulator: &mut Simulator) {
        // get input values
        let mut value = simulator.get_input_val(&self.sext_in);
        let max_size: Signal = 1 << self.in_size as Signal;
        assert!(
            value < max_size,
            "SXT input ({}) greater than allowed input size ({})",
            value,
            max_size
        );

        if (value & 1 << (self.in_size - 1)) != 0 {
            value |= (1 << self.out_size as Signal) - (1 << self.in_size as Signal)
        }

        println!(
            "{}, {}, {}",
            value,
            1 << (self.out_size as Signal),
            1 << (self.in_size as Signal)
        );

        // set output
        simulator.set_id_index(&self.id, 0, value);
    }
}
