use log::trace;
use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, OutputType, Ports, Simulator};

#[derive(Serialize, Deserialize)]
pub struct InstrMem {
    pub id: String,
    pub pos: (f32, f32),
    pub instr: Vec<u32>,
    pub pc: Input,
}

#[typetag::serde()]
impl Component for InstrMem {
    fn to_(&self) {
        println!("InstrMem");
    }
    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.pc.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec!["instruction".into()],
            },
        )
    }

    fn clock(&self, simulator: &mut Simulator) {
        // get instr at pc/4
        let pc: u32 = simulator.get_input_val(&self.pc).try_into().unwrap();

        let instr = self.instr[(pc / 4) as usize];
        trace!("instruction: {:032b}", instr);
        trace!("pc:0x{:08x}", pc);
        // set output
        simulator.set_out_value(&self.id, "instruction", instr);
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
    fn test_inst_mem() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("pc")),
                Rc::new(InstrMem {
                    id: "imem".to_string(),
                    pos: (0.0, 0.0),
                    pc: Input::new("pc", "out"),
                    instr: vec![0x0, 0x1, 0x2, 0x3, 0x4, 0x5],
                }),
            ],
        };

        let mut simulator = Simulator::new(&cs);
        assert_eq!(simulator.cycle, 1);

        // outputs
        let imem_out = &Input::new("imem", "instruction");
        for i in 0..6 {
            simulator.set_out_value("pc", "out", i * 4);
            simulator.clock();
            assert_eq!(simulator.get_input_val(imem_out), i.into());
        }
    }
}
