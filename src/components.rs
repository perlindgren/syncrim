use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Ports {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

#[typetag::serde()]
pub trait Component {
    fn to_(&self) {}
    fn to_ports(&self) -> (String, Ports);
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Input {
    pub id: String,
    pub index: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Output {
    Combinatorial,
    Sequential,
    Constant(u32),
}

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

    fn to_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![],
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

    fn to_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
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

    fn to_ports(&self) -> (String, Ports) {
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

    fn to_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.a_in.clone(), self.b_in.clone()],
                outputs: vec![Output::Combinatorial],
            },
        )
    }
}
