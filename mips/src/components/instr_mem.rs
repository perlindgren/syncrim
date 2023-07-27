use serde::{Deserialize, Serialize};
use std::rc::Rc;
use syncrim::common::{Component, Input, OutputType, Ports, SignalData, SignalUnsigned, Simulator};

#[derive(Serialize, Deserialize)]
pub struct InstrMem {
    pub(crate) id: String,
    pub(crate) pos: (f32, f32),
    pub(crate) pc: Input,
    pub(crate) instr: Vec<u32>,
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
                inputs: vec![self.pc.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec!["out".into()],
            },
        )
    }

    fn clock(&self, simulator: &mut Simulator) {
        let instr: SignalData =
            match TryInto::<SignalUnsigned>::try_into(simulator.get_input_val(&self.pc)) {
                Ok(pc) => {
                    trace!("--- evaluate instr mem: pc {:?}", pc);
                    // get instr at pc/4
                    match self.instr.get((pc / 4) as usize) {
                        Some(instr) => (*instr).into(),
                        _ => SignalData::Unknown,
                    }
                }
                _ => SignalData::Unknown,
            };

        // set output
        trace!("--- output {:?}", instr);
        simulator.set_out_val(&self.id, "out", instr);
    }
}

impl InstrMem {
    pub fn new(id: &str, pos: (f32, f32), pc: Input, instr: Vec<u32>) -> Self {
        InstrMem {
            id: id.to_string(),
            pos,
            pc,
            instr,
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32), pc: Input, instr: Vec<u32>) -> Rc<Self> {
        Rc::new(InstrMem::new(id, pos, pc, instr))
    }
}
