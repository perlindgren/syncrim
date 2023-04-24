use serde::{Deserialize, Serialize};

#[typetag::serde()]
pub trait Component {
    fn to_(&self) {}
}

#[derive(Serialize, Deserialize)]
pub struct Constant {
    pub v: u32,
}

#[typetag::serde]
impl Component for Constant {
    fn to_(&self) {
        println!("constant v {}", self.v);
    }
}

#[derive(Serialize, Deserialize)]
pub struct Register {
    pub r_in: usize,
    pub r_out: usize,
}

#[typetag::serde]
impl Component for Register {
    fn to_(&self) {
        println!("register");
    }
}

#[derive(Serialize, Deserialize)]
pub struct ComponentStore {
    pub store: Vec<Box<dyn Component>>,
}

impl ComponentStore {
    pub fn to_(&self) {
        self.store.iter().for_each(|c| c.to_());
    }
}
