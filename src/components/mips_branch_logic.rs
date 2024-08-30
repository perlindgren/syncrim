// use std::fmt::Alignment;
#[cfg(feature = "gui-egui")]
use crate::common::EguiComponent;
use crate::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, SignalValue, Simulator,
};
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;

pub const BRANCH_OP_ID: &str = "branch_op_in";
pub const BRANCH_RT_ID: &str = "branch_rt_in";
pub const BRANCH_FUNCT_ID: &str = "branch_funct_in";

pub const BRANCH_RS_VALUE_ID: &str = "branch_rs_vlaue_id";
pub const BRANCH_RT_VALUE_ID: &str = "branch_rt_value_id";

pub const BRANCH_OUT_ID: &str = "branch_out";

pub const BRANCH_OFFSET: u32 = 0;
pub const BRANCH_REGISTER: u32 = 1;
pub const BRANCH_TARGET: u32 = 2;
pub const BRANCH_ADD4: u32 = 3;

#[derive(Serialize, Deserialize, Clone)]
pub struct BranchLogic {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) op_in: Input,
    pub(crate) rt_in: Input,
    pub(crate) funct_in: Input,
    pub(crate) rs_value: Input,
    pub(crate) rt_value: Input,
}

#[typetag::serde]
impl Component for BranchLogic {
    fn to_(&self) {
        trace!("branch_logic");
    }
    // #[cfg(feature = "gui-egui")]
    // fn dummy(&self, _id: &str, _pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
    //     let dummy_input = Input::new("dummy", "out");
    //     Box::new(Rc::new(BranchLogic {
    //         id: "dummy".to_string(),
    //         pos: (0.0, 0.0),
    //         clk_in: dummy_input.clone(),
    //     }))
    // }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: BRANCH_OP_ID.to_string(),
                        input: self.op_in.clone(),
                    },
                    &InputPort {
                        port_id: BRANCH_RT_ID.to_string(),
                        input: self.rt_in.clone(),
                    },
                    &InputPort {
                        port_id: BRANCH_FUNCT_ID.to_string(),
                        input: self.funct_in.clone(),
                    },
                    &InputPort {
                        port_id: BRANCH_RS_VALUE_ID.to_string(),
                        input: self.rs_value.clone(),
                    },
                    &InputPort {
                        port_id: BRANCH_RT_VALUE_ID.to_string(),
                        input: self.rt_value.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![BRANCH_OUT_ID],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            BRANCH_OP_ID => self.op_in = new_input,
            BRANCH_RT_ID => self.rt_in = new_input,
            BRANCH_FUNCT_ID => self.funct_in = new_input,
            BRANCH_RS_VALUE_ID => self.rs_value = new_input,
            BRANCH_RT_VALUE_ID => self.rt_value = new_input,
            _ => {}
        }
    }

    // propagate sign extension to output
    // TODO: always extend to Signal size? (it should not matter and should be slightly cheaper)
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input values
        let op: u32 = simulator.get_input_value(&self.op_in).try_into().unwrap();
        let rt: u32 = simulator.get_input_value(&self.rt_in).try_into().unwrap();
        let funct: u32 = simulator
            .get_input_value(&self.funct_in)
            .try_into()
            .unwrap();
        let rs_value: u32 = simulator
            .get_input_value(&self.rs_value)
            .try_into()
            .unwrap();
        let rt_value: u32 = simulator
            .get_input_value(&self.rt_value)
            .try_into()
            .unwrap();

        let out: u32;

        match op {
            0 => {
                if (funct != 8) & (funct != 9) {
                    out = BRANCH_ADD4;
                } else {
                    // JR, JARL
                    out = BRANCH_REGISTER;
                }
            }
            1 => {
                if (rt != 0) & (rt != 16) {
                    // not BLTZ, BLTZAL
                    if (rt != 1) & (rt != 17) {
                        //not BGEZ, BGEZAL
                        // error
                        out = BRANCH_ADD4;
                    } else if (rs_value as i32) >= 0 {
                        // BGEZ, BGEZAL
                        out = BRANCH_OFFSET;
                    } else {
                        out = BRANCH_ADD4;
                    }
                } else if (rs_value as i32) < 0 {
                    // BLTZ, BLTZAL
                    out = BRANCH_OFFSET;
                } else {
                    out = BRANCH_ADD4;
                }
            }
            2 => {
                // J
                out = BRANCH_TARGET;
            }
            3 => {
                // JAL
                out = BRANCH_TARGET;
            }
            4 => {
                if rs_value == rt_value {
                    // BEQ
                    out = BRANCH_OFFSET;
                } else {
                    out = BRANCH_ADD4;
                }
            }
            5 => {
                if rs_value == rt_value {
                    // BNE
                    out = BRANCH_ADD4;
                } else {
                    out = BRANCH_OFFSET;
                }
            }
            6 => {
                if (rs_value as i32) <= 0 {
                    // BLEZ
                    out = BRANCH_OFFSET;
                } else {
                    out = BRANCH_ADD4;
                }
            }
            7 => {
                if (rs_value as i32) > 0 {
                    // BGTZ
                    out = BRANCH_OFFSET;
                } else {
                    out = BRANCH_ADD4;
                }
            }
            _ => {
                out = BRANCH_ADD4;
            }
        }

        simulator.set_out_value(&self.id, BRANCH_OUT_ID, SignalValue::Data(out));
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl BranchLogic {
    pub fn new(
        id: &str,
        pos: (f32, f32),
        op_in: Input,
        rt_in: Input,
        funct_in: Input,
        rs_value: Input,
        rt_value: Input,
    ) -> Self {
        BranchLogic {
            id: id.to_string(),
            pos,
            op_in,
            rt_in,
            funct_in,
            rs_value,
            rt_value,
        }
    }

    pub fn rc_new(
        id: &str,
        pos: (f32, f32),
        op_in: Input,
        rt_in: Input,
        funct_in: Input,
        rs_value: Input,
        rt_value: Input,
    ) -> Rc<Self> {
        Rc::new(BranchLogic::new(
            id, pos, op_in, rt_in, funct_in, rs_value, rt_value,
        ))
    }
}
