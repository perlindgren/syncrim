use log::trace;
use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, OutputType, Ports, SignalValue, Simulator};

#[derive(Serialize, Deserialize)]
pub struct SZExt {
    pub id: String,
    pub pos: (f32, f32),

    pub data_i: Input,
    pub sel_i: Input,
}

#[typetag::serde()]
impl Component for SZExt {
    fn to_(&self) {
        println!("s_z_ext");
    }
    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.data_i.clone(), self.sel_i.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec!["out".into()],
            },
        )
    }
    #[allow(non_snake_case)]
    fn clock(&self, simulator: &mut Simulator) {
        //data is zero extended as default since its a 32 bit signal

        match simulator.get_input_val(&self.data_i) {
            //if there is data, sel should be defined, otherwise panic is good.
            SignalValue::Data(data) => {
                let mut data: u32 = data.try_into().unwrap();
                let sel: u32 = simulator.get_input_val(&self.sel_i).try_into().unwrap();
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
                    id: "szext".to_string(),
                    pos: (0.0, 0.0),
                    data_i: Input::new("input", "out"),
                    sel_i: Input::new("sel", "out"),
                }),
            ],
        };

        let mut simulator = Simulator::new(&cs);
        assert_eq!(simulator.cycle, 1);
        let szext = &Input::new("szext", "out");
        let val = 0b100000000000;

        simulator.set_out_value("input", "out", val);
        simulator.set_out_value("sel", "out", 0);
        simulator.clock();
        assert_eq!(
            simulator.get_input_val(szext),
            ((!0b11111111111) as u32).into()
        );

        simulator.set_out_value("input", "out", val);
        simulator.set_out_value("sel", "out", 1);
        simulator.clock();
        assert_eq!(simulator.get_input_val(szext), 0b100000000000.into());
    }
}
