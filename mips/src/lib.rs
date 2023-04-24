use syncrim::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MipsCtrl {}

#[typetag::serde()]
impl Component for MipsCtrl {
    fn to_(&self) {
        println!("mips");
    }
}
