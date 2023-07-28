use log::trace;
use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, OutputType, Ports, SignalData, Simulator};

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
        let enable: u32 = simulator.get_input_val(&self.enable).try_into().unwrap();
        let out: SignalData;
        let rs1: SignalData = simulator.get_input_val(&self.rs1);
        let rs2: SignalData = simulator.get_input_val(&self.rs2);
        if enable != 0 {
            match simulator.get_input_val(&self.ctrl) {
                SignalData::Unknown | SignalData::DontCare | SignalData::Uninitialized => {
                    out = SignalData::Unknown
                }
                SignalData::Data(ctrl) => {
                    match ctrl {
                        0b000 => {
                            let rs1: u32 = rs1.try_into().unwrap();
                            let rs2: u32 = rs2.try_into().unwrap();
                            if rs1 == rs2 {
                                out = SignalData::from(2);
                            } else {
                                out = SignalData::from(0);
                            }
                        } //beq
                        0b001 => {
                            let rs1: u32 = rs1.try_into().unwrap();
                            let rs2: u32 = rs2.try_into().unwrap();
                            if rs1 != rs2 {
                                out = SignalData::from(2);
                            } else {
                                out = SignalData::from(0);
                            }
                        } //bne
                        0b100 => {
                            let rs1: u32 = rs1.try_into().unwrap();
                            let rs2: u32 = rs2.try_into().unwrap();
                            if (rs1 as i32) < (rs2 as i32) {
                                out = SignalData::from(2);
                            } else {
                                out = SignalData::from(0);
                            }
                        } //blt
                        0b101 => {
                            let rs1: u32 = rs1.try_into().unwrap();
                            let rs2: u32 = rs2.try_into().unwrap();
                            if rs1 as i32 >= rs2 as i32 {
                                out = SignalData::from(2);
                            } else {
                                out = SignalData::from(0);
                            }
                        } //bge
                        0b110 => {
                            let rs1: u32 = rs1.try_into().unwrap();
                            let rs2: u32 = rs2.try_into().unwrap();
                            if rs1 < rs2 {
                                out = SignalData::from(2);
                            } else {
                                out = SignalData::from(0);
                            }
                        } //bltu
                        0b111 => {
                            let rs1: u32 = rs1.try_into().unwrap();
                            let rs2: u32 = rs2.try_into().unwrap();
                            if rs1 >= rs2 {
                                out = SignalData::from(2);
                            } else {
                                out = SignalData::from(0);
                            }
                        } //bgeu
                        0b011 => {
                            out = SignalData::from(1);
                        } //jalr
                        0b010 => {
                            out = SignalData::from(2); //jal
                        }
                        _ => {
                            out = SignalData::from(0);
                        }
                    }
                }
            }
        } else {
            out = SignalData::from(0); // pick pc+4 signal if disabled
        }
        trace!("BranchLogic Out:{:?}", out);
        println!(
            "HIDFAJSDFPIJWAPEOFKOWAPEFKPOEWKFPAWEFPIAWJEFPOAWKEFWAEMFPOWEFPOWAMEFAOWEMFXDDDDD"
        );
        simulator.set_out_val(&self.id, "out", out);
    }
}
