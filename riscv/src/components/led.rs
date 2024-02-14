use serde::{Deserialize, Serialize};
#[cfg(feature = "gui-egui")]
use std::rc::Rc;
#[cfg(feature = "gui-egui")]
use syncrim::common::EguiComponent;
use syncrim::{
    common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator},
    signal::SignalValue,
};

pub const LED_I_ID: &str = "input";
pub const LED_HEIGHT: f32 = 20.0;
pub const LED_WIDTH: f32 = 20.0;

#[derive(Serialize, Deserialize)]
pub struct LED {
    pub height: f32,
    pub width: f32,
    pub id: String,
    pub pos: (f32, f32),

    pub input: Input,
}

#[typetag::serde()]
impl Component for LED {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_(&self) {
        println!("LED");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy = Input::new("dummy", "out");
        Box::new(Rc::new(LED {
            height: LED_HEIGHT,
            width: LED_WIDTH,
            id: id.to_string(),
            pos: (pos.0, pos.1),
            input: dummy.clone(),
        }))
    }
    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        if target_port_id.as_str() == LED_I_ID {
            self.input = new_input;
        }
    }
    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![&InputPort {
                    port_id: LED_I_ID.to_string(),
                    input: self.input.clone(),
                }],
                OutputType::Combinatorial,
                vec![],
            ),
        )
    }
    #[allow(non_snake_case)]
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        Ok(())
    }
}
