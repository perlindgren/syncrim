use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, InputPort, OutputType, Ports, Simulator};

#[derive(Serialize, Deserialize)]
pub struct InstrMem {
    pub id: String,
    pub pos: (f32, f32),
    pub pc: InputPort,
    pub instr: Vec<u32>,
}

use log::*;

impl InstrMem {
    pub fn new(id: &str, pos: (f32, f32), pc: Input, instr: Vec<u32>) -> Self {
        InstrMem {
            id: id.to_string(),
            pos,
            pc: InputPort {
                port_id: String::from("in"),
                input: pc,
            },
            instr,
        }
    }
}
#[typetag::serde()]
impl Component for InstrMem {
    fn to_(&self) {
        trace!("InstrMem");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.pc.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec!["out".into()],
            },
        )
    }

    fn clock(&self, simulator: &mut Simulator) {
        // get instr at pc/4
        let pc: u32 = simulator.get_input_val(&self.pc.input).try_into().unwrap();

        trace!("--- evaluate instr mem: pc {:?}", pc);
        let instr = self.instr[(pc / 4) as usize];
        // set output
        trace!("--- output {}", instr);
        simulator.set_out_val(&self.id, "out", instr);
    }
}
