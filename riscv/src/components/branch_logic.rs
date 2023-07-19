use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, Output, OutputType, Ports, Simulator};

#[derive(Serialize, Deserialize)]
pub struct BranchLogic {
    pub id: String,
    pub pos: (f32, f32),

    pub rs1: Input,
    pub rs2: Input,

    pub ctrl: Input,
    pub enable: Input,

}

#[typetag::serde()]
impl Component for BranchLogic {
    fn to_(&self) {
        println!("BranchLogic");
    }
    fn to_string(&self)->String{"".to_string()}
    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.rs1.clone(),
                self.rs2.clone(),
                self.ctrl.clone(),
                self.enable.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec![
                    "out".into(),
                ],
            },
        )
    }
    #[allow(non_snake_case)]
    fn evaluate(&self, simulator: &mut Simulator) {
        let ctrl = simulator.get_input_val(&self.ctrl);
        let rs1 = simulator.get_input_val(&self.rs1);
        let rs2 = simulator.get_input_val(&self.rs2);
        let enable = simulator.get_input_val(&self.enable);
        let mut out = 0;
        if enable!=0{
            match(ctrl){
                0b000=>{
                    if rs1 == rs2{
                        out = 2;
                    }
                    else{
                        out = 0;
                    }
                }, //beq
                0b001=>{
                    if rs1 != rs2{
                        out = 2;
                    }
                    else{
                        out = 0;
                    }
                }, //bne
                0b100=>{
                    if (rs1 as i32) < (rs2 as i32){
                        out = 2;
                    }
                    else{
                        out = 0;
                    }
                }, //blt
                0b101=>{
                    if rs1 as i32 >= rs2 as i32{
                        out = 2;
                    }
                    else{
                        out = 0;
                    }
                }, //bge
                0b110=>{
                    if rs1 < rs2{
                        out = 2;
                    }
                    else{
                        out = 0;
                    }
                }, //bltu
                0b111=>{
                    if rs1 >= rs2{
                        out = 2;
                    }
                    else{
                        out = 0;
                    }
                }, //bgeu
                0b011=>{
                    out = 1;
                }, //jalr
                0b010=>{
                    out = 2;//jal
                },
                _=>{out = 0;}

            }
        }
        else {out = 0;}


        simulator.set_out_val(&self.id, "out", out);

    }
}
