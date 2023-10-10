use serde::{Deserialize, Serialize};
#[cfg(feature = "gui-egui")]
use std::rc::Rc;
#[cfg(feature = "gui-egui")]
use syncrim::common::EguiComponent;
use syncrim::{
    common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator},
    signal::SignalValue,
};
pub const LSB_ZERO_DATA_I_ID: &str = "data_i";

pub const LSB_ZERO_OUT_ID: &str = "out";

pub const LSB_ZERO_HEIGHT: f32 = 10.0;
pub const LSB_ZERO_WIDTH: f32 = 10.0;

#[derive(Serialize, Deserialize)]
pub struct LSBZero {
    pub height: f32,
    pub width: f32,
    pub id: String,
    pub pos: (f32, f32),

    pub data_i: Input,
}

#[typetag::serde()]
impl Component for LSBZero {
    fn to_(&self) {
        println!("LSBZero");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy = Input::new("dummy", "out");
        Box::new(Rc::new(LSBZero {
            height: LSB_ZERO_HEIGHT,
            width: LSB_ZERO_WIDTH,
            id: id.to_string(),
            pos: (pos.0, pos.1),
            data_i: dummy.clone(),
        }))
    }
    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            LSB_ZERO_DATA_I_ID => self.data_i = new_input,
            _ => (),
        }
    }
    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![&InputPort {
                    port_id: LSB_ZERO_DATA_I_ID.to_string(),
                    input: self.data_i.clone(),
                }],
                OutputType::Combinatorial,
                vec![LSB_ZERO_OUT_ID],
            ),
        )
    }
    #[allow(non_snake_case)]
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        match simulator.get_input_value(&self.data_i) {
            SignalValue::Data(mut data) => {
                let mask: u32 = !0b1;
                data &= mask;
                simulator.set_out_value(&self.id, "out", data);
            }
            _ => simulator.set_out_value(&self.id, "out", SignalValue::Unknown),
        }
        Ok(())
    }
}

mod test {
    #![allow(unused_imports)]
    use super::*;

    use std::rc::Rc;
    use syncrim::{
        common::{ComponentStore, Input, Simulator},
        components::ProbeOut,
    };
    #[test]
    fn lsb_zero_test() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("input")),
                Rc::new(LSBZero {
                    height: 0.0,
                    width: 0.0,
                    id: "lzero".to_string(),
                    pos: (0.0, 0.0),
                    data_i: Input::new("input", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(cs).unwrap();
        assert_eq!(simulator.cycle, 1);

        // outputs
        let lout = &Input::new("lzero", "out");
        for i in 0..100 {
            simulator.set_out_value("input", "out", i);
            simulator.clock();
            assert_eq!(simulator.get_input_value(lout), (i & (!0b1)).into());
        }
    }
}
