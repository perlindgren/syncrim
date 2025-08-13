use crate::common::{Component, Id, Input, InputPort, OutputType, Ports};
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::rc::Rc;

pub const WIRE_INPUT_ID: &str = "in";

#[derive(Serialize, Deserialize, Clone)]
pub struct Wire {
    pub(crate) id: Id,
    pub pos: Vec<(f32, f32)>,
    pub(crate) input: Input,
    // this is colour32, using [u8;4] instead of color
    // because we are not sure if egui feature is present
    // if this field does not exist in wire call basic color to generate it
    // skip serializing color if color matches basic color
    #[serde(
        default = "Wire::basic_color",
        skip_serializing_if = "Wire::is_color_basic"
    )]
    pub(crate) color_rgba: [u8; 4],
}

#[typetag::serde]
impl Component for Wire {
    fn to_(&self) {
        trace!("Wire");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                // Wires take one input
                vec![&InputPort {
                    port_id: WIRE_INPUT_ID.to_string(),
                    input: self.input.clone(),
                }],
                OutputType::Combinatorial,
                // No output value
                vec![],
            ),
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Wire {
    pub fn new(id: &str, pos: Vec<(f32, f32)>, input: Input) -> Self {
        Wire {
            id: id.to_string(),
            pos,
            input,
            color_rgba: Wire::basic_color(),
        }
    }

    pub fn rc_new(id: &str, pos: Vec<(f32, f32)>, input: Input) -> Rc<Wire> {
        Rc::new(Wire::new(id, pos, input))
    }

    pub const fn basic_color() -> [u8; 4] {
        [0x00, 0x00, 0x00, 0xff]
    }

    pub fn is_color_basic(x: &[u8; 4]) -> bool {
        &Wire::basic_color() == x
    }
}
