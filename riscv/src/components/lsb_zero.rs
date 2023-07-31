use serde::{Deserialize, Serialize};
use syncrim::{
    common::{Component, Condition, Input, OutputType, Ports, Simulator},
    signal::SignalValue,
};

#[derive(Serialize, Deserialize)]
pub struct LSBZero {
    pub id: String,
    pub pos: (f32, f32),

    pub data_i: Input,
}

#[typetag::serde()]
impl Component for LSBZero {
    fn to_(&self) {
        println!("LSBZero");
    }
    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.data_i.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec!["out".into()],
            },
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
                    id: "lzero".to_string(),
                    pos: (0.0, 0.0),
                    data_i: Input::new("input", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(&cs);
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
