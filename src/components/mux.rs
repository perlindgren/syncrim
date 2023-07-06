use crate::common::{Component, Input, Output, OutputType, Ports, Simulator};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Mux {
    pub id: String,
    pub pos: (f32, f32),
    pub select: Input,
    pub m_in: Vec<Input>,
}

#[typetag::serde]
impl Component for Mux {
    fn to_(&self) {
        println!("mux");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        let mut inputs = vec![self.select.clone()];
        let mut m = self.m_in.clone();
        inputs.append(&mut m);

        (
            self.id.clone(),
            Ports {
                inputs,
                out_type: OutputType::Combinatorial,
                outputs: vec![Output::Function],
            },
        )
    }

    // propagate selected input value to output
    fn evaluate(&self, simulator: &mut Simulator) {
        // get input value
        let select = simulator.get_input_val(&self.select) as usize;
        println!("select {}", select);
        let value = simulator.get_input_val(&self.m_in[select]);

        // set output
        simulator.set_id_index(&self.id, 0, value);
    }
}
