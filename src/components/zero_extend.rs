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

pub const ZEROEXTEND_SIGNAL_IN_ID: &str = "signal_in";

pub const ZEROEXTEND_OUT_ID: &str = "zero_extend_out";

#[derive(Serialize, Deserialize, Clone)]
pub struct ZeroExtend {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) signal_in: Input,
}

#[typetag::serde]
impl Component for ZeroExtend {
    fn to_(&self) {
        trace!("zero_extend");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, _id: &str, _pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(ZeroExtend {
            id: "dummy".to_string(),
            pos: (0.0, 0.0),
            signal_in: dummy_input.clone(),
        }))
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![&InputPort {
                    port_id: ZEROEXTEND_SIGNAL_IN_ID.to_string(),
                    input: self.signal_in.clone(),
                }],
                OutputType::Combinatorial,
                vec![ZEROEXTEND_OUT_ID],
            ),
        )
    }

    #[allow(clippy::single_match)]
    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            ZEROEXTEND_SIGNAL_IN_ID => self.signal_in = new_input,
            _ => {}
        }
    }

    // propagate sign extension to output
    // TODO: always extend to Signal size? (it should not matter and should be slightly cheaper)
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input values
        let signal_in: u32 = simulator
            .get_input_value(&self.signal_in)
            .try_into()
            .unwrap();

        let output: u32 = signal_in & 0x0000_FFFF; // already zero extended

        simulator.set_out_value(&self.id, ZEROEXTEND_OUT_ID, SignalValue::Data(output));
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ZeroExtend {
    pub fn new(id: &str, pos: (f32, f32), signal_in: Input) -> Self {
        ZeroExtend {
            id: id.to_string(),
            pos,
            signal_in,
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), signal_in: Input) -> Rc<Self> {
        Rc::new(ZeroExtend::new(id, pos, signal_in))
    }
}
