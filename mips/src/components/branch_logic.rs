use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, Output, OutputType, Ports, Signal, Simulator};

#[derive(Serialize, Deserialize)]
pub struct BranchLogic {
    pub id: String,
    pub pos: (f32, f32),
    pub width: f32,
    pub height: f32,

    // ports
    pub reg_a: Input,
    pub reg_b: Input,
    pub ctrl: Input,
}

#[typetag::serde()]
impl Component for BranchLogic {
    fn to_(&self) {
        println!("Branch");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.reg_a.clone(), self.reg_b.clone(), self.ctrl.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec![
                    Output::Function, // pc_mux
                ],
            },
        )
    }

    fn evaluate(&self, simulator: &mut Simulator) {}
}
