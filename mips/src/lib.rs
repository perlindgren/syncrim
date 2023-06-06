use syncrim::common::{Component, OutputType, Ports};

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

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![],
                out_type: OutputType::Combinatorial,
                outputs: vec![],
            },
        )
    }
}
