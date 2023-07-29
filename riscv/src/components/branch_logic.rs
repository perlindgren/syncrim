use log::trace;
use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, OutputType, Ports, SignalValue, Simulator};

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
        let enable: u32 = simulator.get_input_value(&self.enable).try_into().unwrap();
        let out: SignalValue;
        let rs1: SignalValue = simulator.get_input_value(&self.rs1);
        let rs2: SignalValue = simulator.get_input_value(&self.rs2);
        if enable != 0 {
            match simulator.get_input_value(&self.ctrl) {
                SignalValue::Unknown | SignalValue::DontCare | SignalValue::Uninitialized => {
                    out = SignalValue::Unknown
                }
                SignalValue::Data(ctrl) => {
                    match ctrl {
                        0b000 => {
                            let rs1: u32 = rs1.try_into().unwrap();
                            let rs2: u32 = rs2.try_into().unwrap();
                            if rs1 == rs2 {
                                out = SignalValue::from(2);
                                trace!("beq ok");
                            } else {
                                out = SignalValue::from(0);
                                trace!("beq failed");
                            }
                        } //beq
                        0b001 => {
                            let rs1: u32 = rs1.try_into().unwrap();
                            let rs2: u32 = rs2.try_into().unwrap();
                            if rs1 != rs2 {
                                out = SignalValue::from(2);
                                trace!("bne ok");
                            } else {
                                out = SignalValue::from(0);
                                trace!("bne failed");
                            }
                        } //bne
                        0b100 => {
                            let rs1: u32 = rs1.try_into().unwrap();
                            let rs2: u32 = rs2.try_into().unwrap();
                            if (rs1 as i32) < (rs2 as i32) {
                                out = SignalValue::from(2);
                                trace!("blt ok");
                            } else {
                                out = SignalValue::from(0);
                                trace!("blt failed")
                            }
                        } //blt
                        0b101 => {
                            let rs1: u32 = rs1.try_into().unwrap();
                            let rs2: u32 = rs2.try_into().unwrap();
                            if rs1 as i32 >= rs2 as i32 {
                                out = SignalValue::from(2);
                                trace!("bge ok");
                            } else {
                                out = SignalValue::from(0);
                                trace!("bge failed");
                            }
                        } //bge
                        0b110 => {
                            let rs1: u32 = rs1.try_into().unwrap();
                            let rs2: u32 = rs2.try_into().unwrap();
                            if rs1 < rs2 {
                                out = SignalValue::from(2);
                                trace!("bltu ok");
                            } else {
                                out = SignalValue::from(0);
                                trace!("bltu failed");
                            }
                        } //bltu
                        0b111 => {
                            let rs1: u32 = rs1.try_into().unwrap();
                            let rs2: u32 = rs2.try_into().unwrap();
                            if rs1 >= rs2 {
                                trace!("bgeu ok");
                                out = SignalValue::from(2);
                            } else {
                                trace!("bgeu failed");
                                out = SignalValue::from(0);
                            }
                        } //bgeu
                        0b011 => {
                            trace!("jalr ok");
                            out = SignalValue::from(1);
                        } //jalr
                        0b010 => {
                            trace!("jal ok");
                            out = SignalValue::from(2); //jal
                        }
                        _ => {
                            trace!("no control transfer");
                            out = SignalValue::from(0);
                        }
                    }
                }
            }
        } else {
            out = SignalValue::from(0); // pick pc+4 signal if disabled
        }
        trace!("BranchLogic Out:{:?}", out);
        simulator.set_out_value(&self.id, "out", out);
    }
}
#[cfg(test)]
mod test {
    use super::*;

    use std::rc::Rc;
    use syncrim::{
        common::{ComponentStore, Input, Simulator},
        components::ProbeOut,
    };

    #[test]
    fn test_beq() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("rs1")),
                Rc::new(ProbeOut::new("rs2")),
                Rc::new(ProbeOut::new("ctrl")),
                Rc::new(ProbeOut::new("enable")),
                Rc::new(BranchLogic {
                    id: "blu".to_string(),
                    pos: (0.0, 0.0),
                    rs1: Input::new("rs1", "out"),
                    rs2: Input::new("rs2", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    enable: Input::new("enable", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(&cs);
        assert_eq!(simulator.cycle, 1);

        // outputs
        let blu_out = &Input::new("blu", "out");

        simulator.set_out_value("ctrl", "out", 0b001);
        simulator.set_out_value("enable", "out", 0); //not enabled
        simulator.set_out_value("rs1", "out", 42);
        simulator.set_out_value("rs2", "out", 42);
        assert_eq!(simulator.get_input_value(blu_out), 0.into());

        println!("<setup for clock 2>");
        simulator.set_out_value("ctrl", "out", 0b001);
        simulator.set_out_value("enable", "out", 0); //not enabled
        simulator.set_out_value("rs1", "out", 42);
        simulator.set_out_value("rs2", "out", 1337);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 0.into());

        println!("<setup for clock 3>");
        simulator.set_out_value("ctrl", "out", 0b001);
        simulator.set_out_value("enable", "out", 1); //enabled
        simulator.set_out_value("rs1", "out", 42);
        simulator.set_out_value("rs2", "out", 1337);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 2.into());

        println!("<setup for clock 4>");
        simulator.set_out_value("ctrl", "out", 0b001);
        simulator.set_out_value("enable", "out", 1); //enabled
        simulator.set_out_value("rs1", "out", 42);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 0.into());
    }
    #[test]
    fn test_bne() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("rs1")),
                Rc::new(ProbeOut::new("rs2")),
                Rc::new(ProbeOut::new("ctrl")),
                Rc::new(ProbeOut::new("enable")),
                Rc::new(BranchLogic {
                    id: "blu".to_string(),
                    pos: (0.0, 0.0),
                    rs1: Input::new("rs1", "out"),
                    rs2: Input::new("rs2", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    enable: Input::new("enable", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(&cs);
        assert_eq!(simulator.cycle, 1);

        // outputs
        let blu_out = &Input::new("blu", "out");

        simulator.set_out_value("ctrl", "out", 0b000); //beq
        simulator.set_out_value("enable", "out", 0); //not enabled
        simulator.set_out_value("rs1", "out", 42);
        simulator.set_out_value("rs2", "out", 42);
        assert_eq!(simulator.get_input_value(blu_out), 0.into());

        println!("<setup for clock 2>");
        simulator.set_out_value("ctrl", "out", 0b000); //beq
        simulator.set_out_value("enable", "out", 0); //not enabled
        simulator.set_out_value("rs1", "out", 42);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 0.into());

        println!("<setup for clock 3>");
        simulator.set_out_value("ctrl", "out", 0b000); //beq
        simulator.set_out_value("enable", "out", 1); //enabled
        simulator.set_out_value("rs1", "out", 42);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 2.into());

        println!("<setup for clock 4>");
        simulator.set_out_value("ctrl", "out", 0b000); //beq
        simulator.set_out_value("enable", "out", 1); //enabled
        simulator.set_out_value("rs1", "out", 42);
        simulator.set_out_value("rs2", "out", 1337);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 0.into());
    }
    #[test]
    fn test_blt() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("rs1")),
                Rc::new(ProbeOut::new("rs2")),
                Rc::new(ProbeOut::new("ctrl")),
                Rc::new(ProbeOut::new("enable")),
                Rc::new(BranchLogic {
                    id: "blu".to_string(),
                    pos: (0.0, 0.0),
                    rs1: Input::new("rs1", "out"),
                    rs2: Input::new("rs2", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    enable: Input::new("enable", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(&cs);
        assert_eq!(simulator.cycle, 1);

        // outputs
        let blu_out = &Input::new("blu", "out");

        simulator.set_out_value("ctrl", "out", 0b100);
        simulator.set_out_value("enable", "out", 0); //not enabled
        simulator.set_out_value("rs1", "out", 42);
        simulator.set_out_value("rs2", "out", 42);
        assert_eq!(simulator.get_input_value(blu_out), 0.into());

        println!("<setup for clock 2>");
        simulator.set_out_value("ctrl", "out", 0b100);
        simulator.set_out_value("enable", "out", 0); //not enabled
        simulator.set_out_value("rs1", "out", 41);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 0.into());

        println!("<setup for clock 3>");
        simulator.set_out_value("ctrl", "out", 0b100);
        simulator.set_out_value("enable", "out", 1); //enabled
        simulator.set_out_value("rs1", "out", 41);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 2.into());

        println!("<setup for clock 4>");
        simulator.set_out_value("ctrl", "out", 0b100);
        simulator.set_out_value("enable", "out", 1); //enabled
        simulator.set_out_value("rs1", "out", 42);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 0.into());
        println!("<setup for clock 5>");
        simulator.set_out_value("ctrl", "out", 0b100);
        simulator.set_out_value("enable", "out", 1); //enabled
        simulator.set_out_value("rs1", "out", 43);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 0.into());
    }
    #[test]
    fn test_bge() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("rs1")),
                Rc::new(ProbeOut::new("rs2")),
                Rc::new(ProbeOut::new("ctrl")),
                Rc::new(ProbeOut::new("enable")),
                Rc::new(BranchLogic {
                    id: "blu".to_string(),
                    pos: (0.0, 0.0),
                    rs1: Input::new("rs1", "out"),
                    rs2: Input::new("rs2", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    enable: Input::new("enable", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(&cs);
        assert_eq!(simulator.cycle, 1);

        // outputs
        let blu_out = &Input::new("blu", "out");

        simulator.set_out_value("ctrl", "out", 0b101);
        simulator.set_out_value("enable", "out", 0); //not enabled
        simulator.set_out_value("rs1", "out", 42);
        simulator.set_out_value("rs2", "out", 42);
        assert_eq!(simulator.get_input_value(blu_out), 0.into());

        println!("<setup for clock 2>");
        simulator.set_out_value("ctrl", "out", 0b101);
        simulator.set_out_value("enable", "out", 0); //not enabled
        simulator.set_out_value("rs1", "out", 41);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 0.into());

        println!("<setup for clock 3>");
        simulator.set_out_value("ctrl", "out", 0b101);
        simulator.set_out_value("enable", "out", 1); //enabled
        simulator.set_out_value("rs1", "out", 42);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 2.into());

        println!("<setup for clock 4>");
        simulator.set_out_value("ctrl", "out", 0b101);
        simulator.set_out_value("enable", "out", 1); //enabled
        simulator.set_out_value("rs1", "out", 43);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 2.into());
        println!("<setup for clock 5>");
        simulator.set_out_value("ctrl", "out", 0b101);
        simulator.set_out_value("enable", "out", 1); //enabled
        simulator.set_out_value("rs1", "out", 41);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 0.into());
    }
    #[test]
    fn test_bltu() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("rs1")),
                Rc::new(ProbeOut::new("rs2")),
                Rc::new(ProbeOut::new("ctrl")),
                Rc::new(ProbeOut::new("enable")),
                Rc::new(BranchLogic {
                    id: "blu".to_string(),
                    pos: (0.0, 0.0),
                    rs1: Input::new("rs1", "out"),
                    rs2: Input::new("rs2", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    enable: Input::new("enable", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(&cs);
        assert_eq!(simulator.cycle, 1);

        // outputs
        let blu_out = &Input::new("blu", "out");

        simulator.set_out_value("ctrl", "out", 0b110);
        simulator.set_out_value("enable", "out", 0); //not enabled
        simulator.set_out_value("rs1", "out", 42);
        simulator.set_out_value("rs2", "out", 42);
        assert_eq!(simulator.get_input_value(blu_out), 0.into());

        println!("<setup for clock 2>");
        simulator.set_out_value("ctrl", "out", 0b110);
        simulator.set_out_value("enable", "out", 0); //not enabled
        simulator.set_out_value("rs1", "out", 41);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 0.into());

        println!("<setup for clock 3>");
        simulator.set_out_value("ctrl", "out", 0b110);
        simulator.set_out_value("enable", "out", 1); //enabled
        simulator.set_out_value("rs1", "out", 1);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 2.into());

        println!("<setup for clock 4>");
        simulator.set_out_value("ctrl", "out", 0b110);
        simulator.set_out_value("enable", "out", 1); //enabled
        simulator.set_out_value("rs1", "out", -1i32 as u32);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 0.into());
        println!("<setup for clock 5>");
        simulator.set_out_value("ctrl", "out", 0b110);
        simulator.set_out_value("enable", "out", 1); //enabled
        simulator.set_out_value("rs1", "out", 43);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 0.into());
    }
    #[test]
    fn test_bgeu() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("rs1")),
                Rc::new(ProbeOut::new("rs2")),
                Rc::new(ProbeOut::new("ctrl")),
                Rc::new(ProbeOut::new("enable")),
                Rc::new(BranchLogic {
                    id: "blu".to_string(),
                    pos: (0.0, 0.0),
                    rs1: Input::new("rs1", "out"),
                    rs2: Input::new("rs2", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    enable: Input::new("enable", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(&cs);
        assert_eq!(simulator.cycle, 1);

        // outputs
        let blu_out = &Input::new("blu", "out");

        simulator.set_out_value("ctrl", "out", 0b111);
        simulator.set_out_value("enable", "out", 0); //not enabled
        simulator.set_out_value("rs1", "out", 42);
        simulator.set_out_value("rs2", "out", 42);
        assert_eq!(simulator.get_input_value(blu_out), 0.into());

        println!("<setup for clock 2>");
        simulator.set_out_value("ctrl", "out", 0b111);
        simulator.set_out_value("enable", "out", 0); //not enabled
        simulator.set_out_value("rs1", "out", 41);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 0.into());

        println!("<setup for clock 3>");
        simulator.set_out_value("ctrl", "out", 0b111);
        simulator.set_out_value("enable", "out", 1); //enabled
        simulator.set_out_value("rs1", "out", 1);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 0.into());

        println!("<setup for clock 4>");
        simulator.set_out_value("ctrl", "out", 0b111);
        simulator.set_out_value("enable", "out", 1); //enabled
        simulator.set_out_value("rs1", "out", -1i32 as u32);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 2.into());
        println!("<setup for clock 5>");
        simulator.set_out_value("ctrl", "out", 0b111);
        simulator.set_out_value("enable", "out", 1); //enabled
        simulator.set_out_value("rs1", "out", 43);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 2.into());
        println!("<setup for clock 5>");
        simulator.set_out_value("ctrl", "out", 0b111);
        simulator.set_out_value("enable", "out", 1); //enabled
        simulator.set_out_value("rs1", "out", 42);
        simulator.set_out_value("rs2", "out", 42);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 2.into());
    }
    #[test]
    fn test_jalr() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("rs1")),
                Rc::new(ProbeOut::new("rs2")),
                Rc::new(ProbeOut::new("ctrl")),
                Rc::new(ProbeOut::new("enable")),
                Rc::new(BranchLogic {
                    id: "blu".to_string(),
                    pos: (0.0, 0.0),
                    rs1: Input::new("rs1", "out"),
                    rs2: Input::new("rs2", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    enable: Input::new("enable", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(&cs);
        assert_eq!(simulator.cycle, 1);

        // outputs
        let blu_out = &Input::new("blu", "out");

        simulator.set_out_value("ctrl", "out", 0b011);
        simulator.set_out_value("enable", "out", 0); //not enabled
        simulator.set_out_value("rs1", "out", SignalValue::Unknown);
        simulator.set_out_value("rs2", "out", SignalValue::Unknown);
        assert_eq!(simulator.get_input_value(blu_out), 0.into());

        println!("<setup for clock 2>");
        simulator.set_out_value("ctrl", "out", 0b011);
        simulator.set_out_value("enable", "out", 1); //enabled
        simulator.set_out_value("rs1", "out", SignalValue::Unknown);
        simulator.set_out_value("rs2", "out", SignalValue::Unknown);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 1.into());
    }
    #[test]
    fn test_jal() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("rs1")),
                Rc::new(ProbeOut::new("rs2")),
                Rc::new(ProbeOut::new("ctrl")),
                Rc::new(ProbeOut::new("enable")),
                Rc::new(BranchLogic {
                    id: "blu".to_string(),
                    pos: (0.0, 0.0),
                    rs1: Input::new("rs1", "out"),
                    rs2: Input::new("rs2", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    enable: Input::new("enable", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(&cs);
        assert_eq!(simulator.cycle, 1);

        // outputs
        let blu_out = &Input::new("blu", "out");

        simulator.set_out_value("ctrl", "out", 0b010);
        simulator.set_out_value("enable", "out", 0); //not enabled
        simulator.set_out_value("rs1", "out", SignalValue::Unknown);
        simulator.set_out_value("rs2", "out", SignalValue::Unknown);
        assert_eq!(simulator.get_input_value(blu_out), 0.into());

        println!("<setup for clock 2>");
        simulator.set_out_value("ctrl", "out", 0b010);
        simulator.set_out_value("enable", "out", 1); //enabled
        simulator.set_out_value("rs1", "out", SignalValue::Unknown);
        simulator.set_out_value("rs2", "out", SignalValue::Unknown);
        simulator.clock();
        assert_eq!(simulator.get_input_value(blu_out), 2.into());
    }
}
