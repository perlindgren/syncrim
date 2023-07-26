use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, InputPort, OutputType, Ports, Simulator};

pub const INSTR_MEM_PC_ID: &str = "pc";

pub const INSTR_MEM_OUT_ID: &str = "out";

#[derive(Serialize, Deserialize)]
pub struct InstrMem {
    pub id: String,
    pub pos: (f32, f32),
    pub instr: Vec<u32>,
    pub pc: Input,
}

use log::*;

#[typetag::serde()]
impl Component for InstrMem {
    fn to_(&self) {
        trace!("InstrMem");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![InputPort {
                    port_id: INSTR_MEM_PC_ID.to_string(),
                    input: self.pc.clone(),
                }],
                out_type: OutputType::Combinatorial,
                outputs: vec![INSTR_MEM_OUT_ID.to_string()],
            },
        )
    }

    fn clock(&self, simulator: &mut Simulator) {
        // get instr at pc/4
        let pc: u32 = simulator.get_input_val(&self.pc).try_into().unwrap();

        trace!("--- evaluate instr mem: pc {:?}", pc);
        let instr = self.instr[(pc / 4) as usize];
        // set output
        trace!("--- output {}", instr);
        simulator.set_out_val(&self.id, "out", instr);
    }
}
