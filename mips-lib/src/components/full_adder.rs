// use std::fmt::Alignment;
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;
#[cfg(feature = "gui-egui")]
use syncrim::common::EguiComponent;
use syncrim::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, SignalValue, Simulator,
};

pub const FULL_ADD_A_IN_ID: &str = "full_add_a_in";
pub const FULL_ADD_B_IN_ID: &str = "full_add_b_in";
pub const FULL_ADD_OP_IN_ID: &str = "full_add_op_in";
pub const FULL_ADD_OUT_ID: &str = "alu_out";
pub const FULL_ADD_OVERFLOW_OUT_ID: &str = "alu_overflow_out";

pub mod alu_op {
    pub const ADD: u32 = 0;
    pub const ADDU: u32 = 1;
    pub const SUB: u32 = 2;
    pub const SUBU: u32 = 3;
    pub const AND: u32 = 4;
    pub const OR: u32 = 5;
    pub const XOR: u32 = 6;
    pub const NOR: u32 = 7;
    pub const SLT: u32 = 8;
    pub const SLTU: u32 = 9;
    pub const SLL: u32 = 10;
    pub const SRL: u32 = 11;
    pub const SRA: u32 = 12;
    pub const LUI: u32 = 13;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FullAdd {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) a_in: Input,
    pub(crate) b_in: Input,
    pub(crate) op_in: Input,
}

#[typetag::serde]
impl Component for FullAdd {
    fn to_(&self) {
        trace!("full_adder");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, _id: &str, _pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(FullAdd {
            id: "dummy".to_string(),
            pos: (0.0, 0.0),
            a_in: dummy_input.clone(),
            b_in: dummy_input.clone(),
            op_in: dummy_input.clone(),
        }))
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: FULL_ADD_A_IN_ID.to_string(),
                        input: self.a_in.clone(),
                    },
                    &InputPort {
                        port_id: FULL_ADD_B_IN_ID.to_string(),
                        input: self.b_in.clone(),
                    },
                    &InputPort {
                        port_id: FULL_ADD_OP_IN_ID.to_string(),
                        input: self.op_in.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![FULL_ADD_OUT_ID, FULL_ADD_OVERFLOW_OUT_ID],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            FULL_ADD_A_IN_ID => self.a_in = new_input,
            FULL_ADD_B_IN_ID => self.b_in = new_input,
            FULL_ADD_OP_IN_ID => self.op_in = new_input,
            _ => {}
        }
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input values
        let a: u32 = simulator.get_input_value(&self.a_in).try_into().unwrap();
        let b: u32 = simulator.get_input_value(&self.b_in).try_into().unwrap();
        let op: u32 = simulator.get_input_value(&self.op_in).try_into().unwrap();

        let output: u32;
        let mut overflow: u32 = 0;

        match op {
            alu_op::ADD => {
                // output = a.wrapping_add(b);

                let tmp = (a as i32).checked_add(b as i32);
                overflow = match tmp {
                    Some(val) => {
                        output = val as u32;
                        0
                    }
                    None => {
                        output = a.wrapping_add(b);
                        1
                    }
                };
            }
            alu_op::ADDU => {
                output = a.wrapping_add(b);
            }
            alu_op::SUB => {
                let tmp = (a as i32).checked_sub(b as i32);
                overflow = match tmp {
                    Some(val) => {
                        output = val as u32;
                        0
                    }
                    None => {
                        output = a.wrapping_sub(b);
                        1
                    }
                }
            }
            alu_op::SUBU => {
                output = a.wrapping_sub(b);
            }
            alu_op::AND => {
                output = a & b;
            }
            alu_op::OR => {
                output = a | b;
            }
            alu_op::XOR => {
                output = a ^ b;
            }
            alu_op::NOR => {
                output = !(a | b);
            }
            alu_op::SLT => {
                output = ((a as i32) < (b as i32)) as u32;
            }
            alu_op::SLTU => {
                output = (a < b) as u32;
            }
            alu_op::SLL => {
                output = a << b;
            }
            alu_op::SRL => {
                output = a >> b;
            }
            alu_op::SRA => {
                output = ((a as i32) >> b) as u32;
            }
            alu_op::LUI => {
                output = (a & 0x0000_ffff) | (b << 16);
            }
            _ => {
                return Err(Condition::Error(
                    "undef alu operation or unimplemented instruction".to_string(),
                ));
            }
        }
        simulator.set_out_value(&self.id, FULL_ADD_OUT_ID, SignalValue::Data(output));
        simulator.set_out_value(
            &self.id,
            FULL_ADD_OVERFLOW_OUT_ID,
            SignalValue::Data(overflow),
        );
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl FullAdd {
    pub fn new(id: &str, pos: (f32, f32), a_in: Input, b_in: Input, op_in: Input) -> Self {
        FullAdd {
            id: id.to_string(),
            pos,
            a_in,
            b_in,
            op_in,
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), a_in: Input, b_in: Input, op_in: Input) -> Rc<Self> {
        Rc::new(FullAdd::new(id, pos, a_in, b_in, op_in))
    }
}
#[cfg(test)]
mod test {
    use super::*;

    use crate::{
        common::{ComponentStore, Input, Simulator},
        components::ProbeOut,
    };
    use std::rc::Rc;

    #[test]
    fn test_some_alu_op() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("op")),
                Rc::new(ProbeOut::new("a")),
                Rc::new(ProbeOut::new("b")),
                FullAdd::rc_new(
                    "ALU",
                    (0.0, 0.0),
                    Input::new("a", "out"),
                    Input::new("b", "out"),
                    Input::new("op", "out"),
                ),
            ],
        };
        let mut simulator = Simulator::new(cs).unwrap();

        assert_eq!(simulator.cycle, 1);

        // outputs
        let alu_val = &Input::new("ALU", "out");

        // reset
        assert_eq!(simulator.get_input_value(alu_val), (0 + 0).into());

        println!("<setup for clock 2>");
        simulator.set_out_value("a", "out", 42);
        simulator.set_out_value("b", "out", 1337);
        simulator.set_out_value("op", "out", alu_op::ADD);
        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(simulator.cycle, 2);
        assert_eq!(
            simulator.get_input_value(alu_val),
            (42 + 1337).into(),
            "testing add (1)"
        );

        println!("<setup for clock 3>");
        simulator.set_out_value("a", "out", -100i32 as u32);
        simulator.set_out_value("b", "out", 1337);
        simulator.set_out_value("op", "out", alu_op::ADD);
        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(
            simulator.get_input_value(alu_val),
            (1337 - 100).into(),
            "testing add (2)"
        );

        println!("<setup for clock 4>");
        simulator.set_out_value("a", "out", -100i32 as u32);
        simulator.set_out_value("b", "out", 1337);
        simulator.set_out_value("op", "out", alu_op::SUB);
        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(
            simulator.get_input_value(alu_val),
            ((-100i32 - 1337) as u32).into(),
            "testing sub"
        );

        println!("<setup for clock 5>");
        simulator.set_out_value("a", "out", -100i32 as u32);
        simulator.set_out_value("b", "out", 1337);
        simulator.set_out_value("op", "out", alu_op::SLT);
        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(
            simulator.get_input_value(alu_val),
            true.into(),
            "testing SLT"
        );

        println!("<setup for clock 5>");
        simulator.set_out_value("a", "out", -100i32 as u32);
        simulator.set_out_value("b", "out", 1337);
        simulator.set_out_value("op", "out", alu_op::SLTU);
        println!("sim_state {:?}", simulator.sim_state);
        println!("<clock>");
        simulator.clock();
        println!("sim_state {:?}", simulator.sim_state);
        assert_eq!(
            simulator.get_input_value(alu_val),
            false.into(),
            "testing SLT"
        );
    }
}
