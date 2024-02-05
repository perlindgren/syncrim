use serde::{Deserialize, Serialize};
#[cfg(feature = "gui-egui")]
use std::rc::Rc;

#[cfg(feature = "gui-egui")]
use syncrim::common::EguiComponent;
use syncrim::common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator};
pub const WB_CTL_INTR_IN_ID: &str = "clic_i";
pub const WB_CTL_DEC_IN_ID: &str = "dec_i";

pub const WB_CTL_WE_OUT_ID: &str = "we_o";
pub const WB_CTL_MUX_CTL_O_ID: &str = "mux_sel";

pub const WB_CTL_HEIGHT: f32 = 20.0;
pub const WB_CTL_WIDTH: f32 = 20.0;

#[derive(Serialize, Deserialize)]
pub struct WBCtl {
    pub height: f32,
    pub width: f32,
    pub id: String,
    pub pos: (f32, f32),

    pub clic_i: Input,
    pub dec_i: Input,
}

#[typetag::serde()]
impl Component for WBCtl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_(&self) {
        println!("WBCtl");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy = Input::new("dummy", "out");
        Box::new(Rc::new(WBCtl {
            height: WB_CTL_HEIGHT,
            width: WB_CTL_WIDTH,
            id: id.to_string(),
            pos: (pos.0, pos.1),
            clic_i: dummy.clone(),
            dec_i: dummy.clone(),
        }))
    }
    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        if target_port_id.as_str() == WB_CTL_DEC_IN_ID {
            self.dec_i = new_input;
        } else if target_port_id.as_str() == WB_CTL_INTR_IN_ID {
            self.clic_i = new_input;
        }
    }
    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: WB_CTL_INTR_IN_ID.to_string(),
                        input: self.clic_i.clone(),
                    },
                    &InputPort {
                        port_id: WB_CTL_DEC_IN_ID.to_string(),
                        input: self.dec_i.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![WB_CTL_WE_OUT_ID, WB_CTL_MUX_CTL_O_ID],
            ),
        )
    }
    #[allow(non_snake_case)]
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        let dec_we: u32 = simulator
            .get_input_value(&self.dec_i)
            .try_into()
            .unwrap_or(0);
        let clic_we: u32 = simulator
            .get_input_value(&self.clic_i)
            .try_into()
            .unwrap_or(0);
        //assert_ne!(dec_we, clic_we);
        let mux_ctl = if dec_we != 0 { 0 } else { 1 };
        let we = if dec_we == 1 || clic_we == 1 { 1 } else { 0 };
        simulator.set_out_value(&self.id, WB_CTL_MUX_CTL_O_ID, mux_ctl);
        simulator.set_out_value(&self.id, WB_CTL_WE_OUT_ID, we);
        Ok(())
    }
}

mod test {
    #![allow(unused_imports)]
    use super::*;

    use crate::components::LSBZero;
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
