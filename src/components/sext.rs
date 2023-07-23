// use std::fmt::Alignment;
use crate::common::{
    Component, Id, Input, InputId, OutputType, Ports, Signal, SignedSignal, Simulator,
};
use log::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct Sext {
    pub id: Id,
    pub pos: (f32, f32),
    pub sext_in: InputId,
    pub in_size: u32,
    pub out_size: u32,
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
            self.out_size <= Signal::BITS,
            "{}: Output size {} is larger than maximum size {}",
            self.id,
            self.out_size,
            Signal::BITS
        );

        // get input values
        let mut value = simulator.get_input_val(&self.sext_in.input);

        let to_sext = self.out_size - self.in_size; // Amount to be arithmetically shifted
        let to_shl = Signal::BITS - self.in_size; // To move input to MSB
        let to_shr = to_shl - to_sext; // To shift the result back to LSB

        value <<= to_shl;
        value = (value as SignedSignal >> to_sext) as Signal;
        value >>= to_shr;

        // set output
        simulator.set_out_val(&self.id, "out", value);
    }
}
