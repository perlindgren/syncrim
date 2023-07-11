use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, Output, OutputType, Ports, Simulator};

#[derive(Serialize, Deserialize)]
pub struct CtrlLogic {
    pub id: String,
    pub pos: (f32, f32),
    pub width: f32,
    pub height: f32,

    // ports
    pub op: Input,
    pub shamt: Input,
    pub funct: Input,
}

#[typetag::serde()]
impl Component for CtrlLogic {
    fn to_(&self) {
        println!("Ctrl");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.op.clone(), self.shamt.clone(), self.funct.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec![
                    Output::Function, // 0
                    Output::Function, // 1
                    Output::Function, // 2
                ],
            },
        )
    }

    fn evaluate(&self, simulator: &mut Simulator) {}
}
