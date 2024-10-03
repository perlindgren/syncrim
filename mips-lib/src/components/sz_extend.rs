// use std::fmt::Alignment;
use crate::components::cntr_unit_signals;
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;
use syncrim::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, SignalValue, Simulator,
};

pub const SIGNZEROEXTEND_SIGNAL_IN_ID: &str = "signzero_signal_in";
pub const SIGNZEROEXTEND_CTRL_IN_ID: &str = "signzero_ctrl_in";

pub const SIGNZEROEXTEND_OUT_ID: &str = "sz_out";

#[derive(Serialize, Deserialize, Clone)]
pub struct SignZeroExtend {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) signzero_signal_in: Input,
    pub(crate) signzero_ctrl_in: Input,
}

#[typetag::serde]
impl Component for SignZeroExtend {
    fn to_(&self) {
        trace!("pc+4");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: SIGNZEROEXTEND_SIGNAL_IN_ID.to_string(),
                        input: self.signzero_signal_in.clone(),
                    },
                    &InputPort {
                        port_id: SIGNZEROEXTEND_CTRL_IN_ID.to_string(),
                        input: self.signzero_ctrl_in.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![SIGNZEROEXTEND_OUT_ID],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            SIGNZEROEXTEND_SIGNAL_IN_ID => self.signzero_signal_in = new_input,
            SIGNZEROEXTEND_CTRL_IN_ID => self.signzero_ctrl_in = new_input,
            _ => {}
        }
    }

    // propagate sign extension to output
    // TODO: always extend to Signal size? (it should not matter and should be slightly cheaper)
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input values
        let mut signal: u32 = simulator
            .get_input_value(&self.signzero_signal_in)
            .try_into()
            .unwrap();

        let ctrl: u32 = simulator
            .get_input_value(&self.signzero_ctrl_in)
            .try_into()
            .unwrap();

        if ctrl == cntr_unit_signals::EXTEND_SIGNED && (signal >> 15) == 1 {
            signal |= 0xffff_0000;
        }

        simulator.set_out_value(&self.id, SIGNZEROEXTEND_OUT_ID, SignalValue::Data(signal));
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl SignZeroExtend {
    pub fn new(
        id: &str,
        pos: (f32, f32),
        signzero_signal_in: Input,
        signzero_ctrl_in: Input,
    ) -> Self {
        SignZeroExtend {
            id: id.to_string(),
            pos,
            signzero_signal_in,
            signzero_ctrl_in,
        }
    }

    pub fn rc_new(
        id: &str,
        pos: (f32, f32),
        signzero_signal_in: Input,
        signzero_ctrl_in: Input,
    ) -> Rc<Self> {
        Rc::new(SignZeroExtend::new(
            id,
            pos,
            signzero_signal_in,
            signzero_ctrl_in,
        ))
    }
}
