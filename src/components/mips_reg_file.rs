// use std::fmt::Alignment;
#[cfg(feature = "gui-egui")]
use crate::common::EguiComponent;
use crate::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, SignalSigned, SignalUnsigned,
    SignalValue, Simulator,
};
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;

use super::register;

pub const REG_FILE_A1_IN_ID: &str = "reg_file_a1_in";
pub const REG_FILE_A2_IN_ID: &str = "reg_file_a2_in";
pub const REG_FILE_A3_IN_ID: &str = "reg_file_a3_in";
pub const REG_FILE_WD3_IN_ID: &str = "reg_file_wd3_in";
pub const REG_FILE_WE3_IN_ID: &str = "reg_file_we3_in";

pub const REG_FILE_RD1_OUT_ID: &str = "rd1_out";
pub const REG_FILE_RD2_OUT_ID: &str = "rd2_out";

#[derive(Serialize, Deserialize, Clone)]
pub struct RegFile {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) a1_in: Input,
    pub(crate) a2_in: Input,
    pub(crate) a3_in: Input,
    pub(crate) wd3_in: Input,
    pub(crate) we3_in: Input,
    registers: Vec<u32>,
}

#[typetag::serde]
impl Component for RegFile {
    fn to_(&self) {
        trace!("full_adder");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(RegFile {
            id: "dummy".to_string(),
            pos: (0.0, 0.0),
            a1_in: dummy_input.clone(),
            a2_in: dummy_input.clone(),
            a3_in: dummy_input.clone(),
            wd3_in: dummy_input.clone(),
            we3_in: dummy_input.clone(),
            registers: vec![0; 32],
        }))
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: REG_FILE_A1_IN_ID.to_string(),
                        input: self.a1_in.clone(),
                    },
                    &InputPort {
                        port_id: REG_FILE_A2_IN_ID.to_string(),
                        input: self.a2_in.clone(),
                    },
                    &InputPort {
                        port_id: REG_FILE_A3_IN_ID.to_string(),
                        input: self.a3_in.clone(),
                    },
                    &InputPort {
                        port_id: REG_FILE_WD3_IN_ID.to_string(),
                        input: self.wd3_in.clone(),
                    },
                    &InputPort {
                        port_id: REG_FILE_WE3_IN_ID.to_string(),
                        input: self.we3_in.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![REG_FILE_RD1_OUT_ID, REG_FILE_RD2_OUT_ID],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            REG_FILE_A1_IN_ID => self.a1_in = new_input,
            REG_FILE_A2_IN_ID => self.a2_in = new_input,
            REG_FILE_A3_IN_ID => self.a3_in = new_input,
            REG_FILE_WD3_IN_ID => self.wd3_in = new_input,
            REG_FILE_WE3_IN_ID => self.we3_in = new_input,
            _ => {}
        }
    }

    // propagate sign extension to output
    // TODO: always extend to Signal size? (it should not matter and should be slightly cheaper)
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input values
        let a1: u32 = simulator.get_input_value(&self.a1_in).try_into().unwrap();
        let a2: u32 = simulator.get_input_value(&self.a2_in).try_into().unwrap();
        let a3: u32 = simulator.get_input_value(&self.a3_in).try_into().unwrap();
        let wd3: u32 = simulator.get_input_value(&self.wd3_in).try_into().unwrap();
        let we3: u32 = simulator.get_input_value(&self.we3_in).try_into().unwrap();
        let registers: Vec<u32> = &mut simulator.get_input_value(&self.registers);

        if we3 == 1 {
            registers.get_mut(a3 as usize).unwrap() = wd3;
        }

        let a1_out = a1 >> 21;
        let a2_out = a2 >> 16;

        simulator.set_out_value(
            &self.id,
            REG_FILE_RD1_OUT_ID,
            SignalValue::Data(self.registers[a1_out as usize]),
        );
        simulator.set_out_value(
            &self.id,
            REG_FILE_RD2_OUT_ID,
            SignalValue::Data(self.registers[a2_out as usize]),
        );
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl RegFile {
    pub fn new(
        id: &str,
        pos: (f32, f32),
        a1_in: Input,
        a2_in: Input,
        a3_in: Input,
        wd3_in: Input,
        we3_in: Input,
    ) -> Self {
        RegFile {
            id: id.to_string(),
            pos,
            a1_in,
            a2_in,
            a3_in,
            wd3_in,
            we3_in,
        }
    }

    pub fn rc_new(
        id: &str,
        pos: (f32, f32),
        a1_in: Input,
        a2_in: Input,
        a3_in: Input,
        wd3_in: Input,
        we3_in: Input,
    ) -> Rc<Self> {
        Rc::new(RegFile::new(id, pos, a1_in, a2_in, a3_in, wd3_in, we3_in))
    }
}
