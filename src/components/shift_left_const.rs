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

pub const SHIFT_SIGNAL_IN_ID: &str = "shift_in";

pub const SHIFT_OUT_ID: &str = "shift_left_const_out";

#[derive(Serialize, Deserialize, Clone)]
pub struct ShiftConst {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) signal_in: Input,
    pub shift_by: u32,
}

#[typetag::serde]
impl Component for ShiftConst {
    fn to_(&self) {
        trace!("shift");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, _id: &str, _pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(ShiftConst {
            id: "dummy".to_string(),
            pos: (0.0, 0.0),
            signal_in: dummy_input.clone(),
            shift_by: 0,
        }))
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![&InputPort {
                    port_id: SHIFT_SIGNAL_IN_ID.to_string(),
                    input: self.signal_in.clone(),
                }],
                OutputType::Combinatorial,
                vec![SHIFT_OUT_ID],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            SHIFT_SIGNAL_IN_ID => self.signal_in = new_input,
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

        let output: u32 = signal_in << self.shift_by;
        simulator.set_out_value(&self.id, SHIFT_OUT_ID, SignalValue::Data(output));
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ShiftConst {
    pub fn new(id: &str, pos: (f32, f32), signal_in: Input, shift_by: u32) -> Self {
        ShiftConst {
            id: id.to_string(),
            pos,
            signal_in,
            shift_by,
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), signal_in: Input, shift_by: u32) -> Rc<Self> {
        Rc::new(ShiftConst::new(id, pos, signal_in, shift_by))
    }
}
