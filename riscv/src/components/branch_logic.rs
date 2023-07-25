use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, OutputType, Ports, Simulator};

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
    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![
                    self.rs1.clone(),
                    self.rs2.clone(),
                    self.ctrl.clone(),
                    self.enable.clone(),
                ],
                out_type: OutputType::Combinatorial,
                outputs: vec!["out".into()],
            },
        )
    }
    #[allow(non_snake_case)]
    fn clock(&self, simulator: &mut Simulator) {
        let ctrl: u32 = simulator.get_input_val(&self.ctrl).try_into().unwrap();
        let rs1: u32 = simulator.get_input_val(&self.rs1).try_into().unwrap();
        let rs2: u32 = simulator.get_input_val(&self.rs2).try_into().unwrap();
        let enable: u32 = simulator.get_input_val(&self.enable).try_into().unwrap();
        let out: u32;
        if enable != 0 {
            match ctrl {
                0b000 => {
                    if rs1 == rs2 {
                        out = 2;
                    } else {
                        out = 0;
                    }
                } //beq
                0b001 => {
                    if rs1 != rs2 {
                        out = 2;
                    } else {
                        out = 0;
                    }
                } //bne
                0b100 => {
                    if (rs1 as i32) < (rs2 as i32) {
                        out = 2;
                    } else {
                        out = 0;
                    }
                } //blt
                0b101 => {
                    if rs1 as i32 >= rs2 as i32 {
                        out = 2;
                    } else {
                        out = 0;
                    }
                } //bge
                0b110 => {
                    if rs1 < rs2 {
                        out = 2;
                    } else {
                        out = 0;
                    }
                } //bltu
                0b111 => {
                    if rs1 >= rs2 {
                        out = 2;
                    } else {
                        out = 0;
                    }
                } //bgeu
                0b011 => {
                    out = 1;
                } //jalr
                0b010 => {
                    out = 2; //jal
                }
                _ => {
                    out = 0;
                }
            }
        } else {
            out = 0;
        }

        simulator.set_out_val(&self.id, "out", out);
    }
}
