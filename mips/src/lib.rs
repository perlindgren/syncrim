use syncrim::components::{Component, Ports};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MipsCtrl {
    pub id: String,
}

#[typetag::serde()]
impl Component for MipsCtrl {
    fn to_(&self) {
        println!("mips");
    }

    fn to_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![],
                outputs: vec![],
            },
        )
    }
}
