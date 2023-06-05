use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Ports {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Input {
    pub id: String,
    pub index: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Output {
    // Will be evaluated as a constant (function without inputs)
    Constant(u32),
    // Will be evaluated as a combinatorial function from inputs to outputs
    Combinatorial,
    // Will be evaluated as synchronous copy from input to output
    Sequential,
}

#[typetag::serde()]
pub trait Component {
    // placeholder
    fn to_(&self) {}
    // returns the (id, Ports) of the component
    fn get_id_ports(&self) -> (String, Ports);
}

// components

#[derive(Serialize, Deserialize)]
pub struct Constant {
    pub id: String,
    pub value: u32,
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
                // A constant does not take any inputs
                inputs: vec![],
                // A single output
                outputs: vec![Output::Constant(self.value)],
            },
        )
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
                outputs: vec![Output::Sequential],
            },
        )
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
                outputs: vec![Output::Combinatorial],
            },
        )
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
                outputs: vec![Output::Combinatorial],
            },
        )
    }
}
