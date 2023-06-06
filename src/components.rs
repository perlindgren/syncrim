use crate::common::{Component, Input, Output, OutputType, Ports, SimState};
use serde::{Deserialize, Serialize};

// components

#[derive(Serialize, Deserialize)]
pub struct Constant {
    pub id: String,
    pub value: u32, // perhaps vector here ... not sure
}

#[derive(Serialize, Deserialize)]
pub struct Register {
    pub id: String,
    pub r_in: Input,
}

#[derive(Serialize, Deserialize)]
pub struct Mux {
    pub id: String,
    pub select: Input,
    pub m_in: Vec<Input>,
}

#[derive(Serialize, Deserialize)]
pub struct Add {
    pub id: String,
    pub a_in: Input,
    pub b_in: Input,
}

// --- not sure where these should go ---

#[typetag::serde]
impl Component for Constant {
    fn to_(&self) {
        println!("constant {:?}", self.value);
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                // Constants do not take any inputs
                inputs: vec![],
                out_type: OutputType::Combinatorial,
                // Single output value
                outputs: vec![Output::Constant(self.value)],
            },
        )
    }

    fn evaluate(&self, sim_state: &mut SimState) {
        sim_state.set_id_index(&self.id, 0, self.value);
    }
}

#[typetag::serde]
impl Component for Register {
    fn to_(&self) {
        println!("register");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                // Vector of inputs
                inputs: vec![self.r_in.clone()],
                out_type: OutputType::Sequential,
                outputs: vec![Output::Function],
            },
        )
    }

    // propagate input value to output
    fn evaluate(&self, sim_state: &mut SimState) {
        // get input value
        let value = sim_state.get_input_val(&self.r_in);
        // set output
        sim_state.set_id_index(&self.id, 0, value);
    }
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
    fn evaluate(&self, sim_state: &mut SimState) {
        // get input value
        let select = sim_state.get_input_val(&self.select) as usize;
        let value = sim_state.get_input_val(&self.m_in[select]);

        // set output
        sim_state.set_id_index(&self.id, 0, value);
    }
}

#[typetag::serde]
impl Component for Add {
    fn to_(&self) {
        println!("add");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.a_in.clone(), self.b_in.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec![Output::Function],
            },
        )
    }

    // propagate addition to output
    fn evaluate(&self, sim_state: &mut SimState) {
        // get input values
        let a_in = sim_state.get_input_val(&self.a_in);
        let b_in = sim_state.get_input_val(&self.b_in);

        // compute addition (notice will panic on overflow)
        let value = a_in + b_in;

        // set output
        sim_state.set_id_index(&self.id, 0, value);
    }
}
