use log::trace;
use serde::{Deserialize, Serialize};
#[cfg(feature = "gui-egui")]
use std::rc::Rc;
#[cfg(feature = "gui-egui")]
use syncrim::common::EguiComponent;
use syncrim::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, SignalValue, Simulator,
};
pub const SIGN_ZERO_EXT_DATA_I_ID: &str = "data_i";
pub const SIGN_ZERO_EXT_SEL_I_ID: &str = "sel_i";

pub const SIGN_ZERO_EXT_OUT_ID: &str = "out";

pub const SIGN_ZERO_EXT_HEIGHT: f32 = 30.0;
pub const SIGN_ZERO_EXT_WIDTH: f32 = 60.0;

#[derive(Serialize, Deserialize)]
pub struct SZExt {
    pub height: f32,
    pub width: f32,
    pub id: String,
    pub pos: (f32, f32),

    pub data_i: Input,
    pub sel_i: Input,
}

#[typetag::serde()]
impl Component for SZExt {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_(&self) {
        println!("s_z_ext");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy = Input::new("dummy", "out");
        Box::new(Rc::new(SZExt {
            height: SIGN_ZERO_EXT_HEIGHT,
            width: SIGN_ZERO_EXT_WIDTH,
            id: id.to_string(),
            pos: (pos.0, pos.1),
            data_i: dummy.clone(),
            sel_i: dummy.clone(),
        }))
    }
    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            SIGN_ZERO_EXT_DATA_I_ID => self.data_i = new_input,
            SIGN_ZERO_EXT_SEL_I_ID => self.sel_i = new_input,
            _ => (),
        }
    }
    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: SIGN_ZERO_EXT_DATA_I_ID.to_string(),
                        input: self.data_i.clone(),
                    },
                    &InputPort {
                        port_id: SIGN_ZERO_EXT_SEL_I_ID.to_string(),
                        input: self.sel_i.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![SIGN_ZERO_EXT_OUT_ID],
            ),
        )
    }
    #[allow(non_snake_case)]
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        //data is zero extended as default since its a 32 bit signal

        match simulator.get_input_value(&self.data_i) {
            //if there is data, sel should be defined, otherwise panic is good.
            SignalValue::Data(mut data) => {
                let sel: u32 = simulator.get_input_value(&self.sel_i).try_into().unwrap();
                //println!("SZEDATA:{:x}", data);
                match sel {
                    0 => {
                        trace!("Sign extending");
                        if data >> 11 == 1 {
                            let mask: u32 = 0xFFFFF000;
                            data |= mask;
                            //println!("sign was one, data:{:x}", data);
                        }
                    }
                    1 => {
                        trace!("Zero extending");
                    }
                    _ => {
                        panic!("Invalid sel on SZExt:{}", sel)
                    }
                }
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
    fn test_szext() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("input")),
                Rc::new(ProbeOut::new("sel")),
                Rc::new(SZExt {
                    height: 0.0,
                    width: 0.0,
                    id: "szext".to_string(),
                    pos: (0.0, 0.0),
                    data_i: Input::new("input", "out"),
                    sel_i: Input::new("sel", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(cs).unwrap();
        assert_eq!(simulator.cycle, 1);
        let szext = &Input::new("szext", "out");
        let val = 0b100000000000;

        simulator.set_out_value("input", "out", val);
        simulator.set_out_value("sel", "out", 0);
        simulator.clock();
        assert_eq!(
            simulator.get_input_value(szext),
            ((!0b11111111111) as u32).into()
        );

        simulator.set_out_value("input", "out", val);
        simulator.set_out_value("sel", "out", 1);
        simulator.clock();
        assert_eq!(simulator.get_input_value(szext), 0b100000000000.into());
    }
}
