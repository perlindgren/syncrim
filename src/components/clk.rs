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

pub const CLK_IN_ID: &str = "clk_in";

pub const CLK_OUT_ID: &str = "out";

#[derive(Serialize, Deserialize, Clone)]
pub struct MIPSCLK {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) clk_in: Input,
}

#[typetag::serde]
impl Component for MIPSCLK {
    fn to_(&self) {
        trace!("pc+4");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, _id: &str, _pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(MIPSCLK {
            id: "dummy".to_string(),
            pos: (0.0, 0.0),
            clk_in: dummy_input.clone(),
        }))
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![&InputPort {
                    port_id: CLK_IN_ID.to_string(),
                    input: self.clk_in.clone(),
                }],
                OutputType::Combinatorial,
                vec![CLK_OUT_ID],
            ),
        )
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            CLK_A_IN_ID => self.clk_in = new_input,
            _ => {}
        }
    }

    // propagate sign extension to output
    // TODO: always extend to Signal size? (it should not matter and should be slightly cheaper)
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input values
        let start_time: u32 = simulator.get_input_value(&self.clk_in).try_into().unwrap();

        simulator.set_out_value(
            &self.id,
            CLK_OUT_ID,
            SignalValue::Data(start_time.wrapping_add(4)),
        );
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MIPSCLK {
    pub fn new(id: &str, pos: (f32, f32), clk_in: Input) -> Self {
        MIPSCLK {
            id: id.to_string(),
            pos,
            clk_in,
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), clk_in: Input) -> Rc<Self> {
        Rc::new(MIPSCLK::new(id, pos, clk_in))
    }
}
