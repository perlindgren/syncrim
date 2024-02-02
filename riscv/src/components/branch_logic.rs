use log::trace;
use serde::{Deserialize, Serialize};
#[cfg(feature = "gui-egui")]
use std::rc::Rc;
#[cfg(feature = "gui-egui")]
use syncrim::common::EguiComponent;
use syncrim::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, SignalValue, Simulator,
};

pub const BRANCH_LOGIC_RS1_ID: &str = "rs1";
pub const BRANCH_LOGIC_RS2_ID: &str = "rs2";
pub const BRANCH_LOGIC_CTRL_ID: &str = "ctrl";
pub const BRANCH_LOGIC_ENABLE_ID: &str = "enable";
pub const BRANCH_LOGIC_MRET_ID: &str = "mret";
pub const BRANCH_LOGIC_OUT_ID: &str = "out";

pub const BRANCH_LOGIC_HEIGHT: f32 = 60.0;
pub const BRANCH_LOGIC_WIDTH: f32 = 60.0;

#[derive(Serialize, Deserialize)]
pub struct BranchLogic {
    pub width: f32,
    pub height: f32,
    pub id: String,
    pub pos: (f32, f32),

    pub rs1: Input,
    pub rs2: Input,

    pub ctrl: Input,
    pub enable: Input,

    pub mret: Input,

    pub int: Input,
}

#[typetag::serde()]
impl Component for BranchLogic {
    fn to_(&self) {
        println!("BranchLogic");
    }

    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy = Input::new("dummy", "out");
        Box::new(Rc::new(BranchLogic {
            width: 60.0,
            height: 60.0,
            id: id.to_string(),
            pos: (pos.0, pos.1),
            rs1: dummy.clone(),
            rs2: dummy.clone(),
            ctrl: dummy.clone(),
            enable: dummy.clone(),
            mret: dummy.clone(),
            int: dummy.clone(),
        }))
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            BRANCH_LOGIC_RS1_ID => self.rs1 = new_input,
            BRANCH_LOGIC_RS2_ID => self.rs2 = new_input,
            BRANCH_LOGIC_CTRL_ID => self.ctrl = new_input,
            BRANCH_LOGIC_ENABLE_ID => self.enable = new_input,
            BRANCH_LOGIC_MRET_ID => self.mret = new_input,
            _ => (),
        }
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: BRANCH_LOGIC_RS1_ID.to_string(),
                        input: self.rs1.clone(),
                    },
                    &InputPort {
                        port_id: BRANCH_LOGIC_RS2_ID.to_string(),
                        input: self.rs2.clone(),
                    },
                    &InputPort {
                        port_id: BRANCH_LOGIC_CTRL_ID.to_string(),
                        input: self.ctrl.clone(),
                    },
                    &InputPort {
                        port_id: BRANCH_LOGIC_ENABLE_ID.to_string(),
                        input: self.enable.clone(),
                    },
                    &InputPort {
                        port_id: BRANCH_LOGIC_MRET_ID.to_string(),
                        input: self.mret.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![BRANCH_LOGIC_OUT_ID],
            ),
        )
    }

    #[allow(non_snake_case)]
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        let enable: u32 = simulator.get_input_value(&self.enable).try_into().unwrap();
        let out: SignalValue;
        let rs1: SignalValue = simulator.get_input_value(&self.rs1);
        let rs2: SignalValue = simulator.get_input_value(&self.rs2);
        let int: SignalValue = simulator.get_input_value(&self.int);
        let mret: SignalValue = simulator.get_input_value(&self.mret);
        if let SignalValue::Data(sig) = int {
            if sig == 1 {
                trace!("int");
                simulator.set_out_value(&self.id, "out", 3);
                return Ok(()); //if interrupt just return here.
            }
        }
        if let SignalValue::Data(sig) = mret {
            if sig == 1 {
                trace!("mret");
                simulator.set_out_value(&self.id, "out", 4);
                return Ok(()); //if mret just return here.
            }
        }
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
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
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
                Rc::new(ProbeOut::new("int")),
                Rc::new(ProbeOut::new("mret")),
                Rc::new(BranchLogic {
                    width: 0.0,
                    height: 0.0,
                    id: "blu".to_string(),
                    pos: (0.0, 0.0),
                    rs1: Input::new("rs1", "out"),
                    rs2: Input::new("rs2", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    enable: Input::new("enable", "out"),
                    int: Input::new("int", "out"),
                    mret: Input::new("mret", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(cs).unwrap();
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
                Rc::new(ProbeOut::new("int")),
                Rc::new(ProbeOut::new("mret")),
                Rc::new(BranchLogic {
                    width: 0.0,
                    height: 0.0,
                    id: "blu".to_string(),
                    pos: (0.0, 0.0),
                    rs1: Input::new("rs1", "out"),
                    rs2: Input::new("rs2", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    enable: Input::new("enable", "out"),
                    int: Input::new("int", "out"),
                    mret: Input::new("mret", "out"),
                }),
            ],
        };
        let mut simulator = Simulator::new(cs).unwrap();
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
                Rc::new(ProbeOut::new("int")),
                Rc::new(ProbeOut::new("mret")),
                Rc::new(BranchLogic {
                    width: 0.0,
                    height: 0.0,
                    id: "blu".to_string(),
                    pos: (0.0, 0.0),
                    rs1: Input::new("rs1", "out"),
                    rs2: Input::new("rs2", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    enable: Input::new("enable", "out"),
                    int: Input::new("int", "out"),
                    mret: Input::new("mret", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(cs).unwrap();
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
                Rc::new(ProbeOut::new("int")),
                Rc::new(ProbeOut::new("mret")),
                Rc::new(BranchLogic {
                    width: 0.0,
                    height: 0.0,
                    id: "blu".to_string(),
                    pos: (0.0, 0.0),
                    rs1: Input::new("rs1", "out"),
                    rs2: Input::new("rs2", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    enable: Input::new("enable", "out"),
                    int: Input::new("int", "out"),
                    mret: Input::new("mret", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(cs).unwrap();
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
                Rc::new(ProbeOut::new("int")),
                Rc::new(ProbeOut::new("mret")),
                Rc::new(BranchLogic {
                    width: 0.0,
                    height: 0.0,
                    id: "blu".to_string(),
                    pos: (0.0, 0.0),
                    rs1: Input::new("rs1", "out"),
                    rs2: Input::new("rs2", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    enable: Input::new("enable", "out"),
                    int: Input::new("int", "out"),
                    mret: Input::new("mret", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(cs).unwrap();
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
                Rc::new(ProbeOut::new("int")),
                Rc::new(ProbeOut::new("mret")),
                Rc::new(BranchLogic {
                    width: 0.0,
                    height: 0.0,
                    id: "blu".to_string(),
                    pos: (0.0, 0.0),
                    rs1: Input::new("rs1", "out"),
                    rs2: Input::new("rs2", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    enable: Input::new("enable", "out"),
                    int: Input::new("int", "out"),
                    mret: Input::new("mret", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(cs).unwrap();
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
                Rc::new(ProbeOut::new("int")),
                Rc::new(ProbeOut::new("mret")),
                Rc::new(BranchLogic {
                    width: 0.0,
                    height: 0.0,
                    id: "blu".to_string(),
                    pos: (0.0, 0.0),
                    rs1: Input::new("rs1", "out"),
                    rs2: Input::new("rs2", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    enable: Input::new("enable", "out"),
                    int: Input::new("int", "out"),
                    mret: Input::new("mret", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(cs).unwrap();
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
                Rc::new(ProbeOut::new("int")),
                Rc::new(ProbeOut::new("mret")),
                Rc::new(BranchLogic {
                    width: 0.0,
                    height: 0.0,
                    id: "blu".to_string(),
                    pos: (0.0, 0.0),
                    rs1: Input::new("rs1", "out"),
                    rs2: Input::new("rs2", "out"),
                    ctrl: Input::new("ctrl", "out"),
                    enable: Input::new("enable", "out"),
                    int: Input::new("int", "out"),
                    mret: Input::new("mret", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(cs).unwrap();
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
