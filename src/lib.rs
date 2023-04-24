use serde::{Deserialize, Serialize};
// use serde_derive::{Deserialize, Serialize};

pub trait Component {}

pub struct Constant {
    pub v: u32,
}

impl Component for Constant {}

pub struct Register {
    pub r_in: usize,
    pub r_out: usize,
}

impl Component for Register {}

pub trait NewTrait: Serialize + Component {}

// #[derive(Serialize)]
// pub struct ComponentStore {
//     pub store: dyn NewTrait,
// }
