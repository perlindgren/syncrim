use serde::{Deserialize, Serialize};
use std::rc::Rc;
use syncrim::common::{
    Component, Condition, Input, InputPort, OutputType, Ports, SignalUnsigned, SignalValue,
    Simulator,
};

pub const INSTR_MEM_PC_ID: &str = "pc";

pub const INSTR_MEM_OUT_ID: &str = "out";

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
                inputs: vec![InputPort {
                    port_id: INSTR_MEM_PC_ID.to_string(),
                    input: self.pc.clone(),
                }],
                out_type: OutputType::Combinatorial,
                outputs: vec![INSTR_MEM_OUT_ID.to_string()],
            },
        )
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        let instr: SignalValue =
            match TryInto::<SignalUnsigned>::try_into(simulator.get_input_value(&self.pc)) {
                Ok(pc) => {
                    trace!("--- evaluate instr mem: pc {:?}", pc);
                    // get instr at pc/4
                    match self.instr.get((pc / 4) as usize) {
                        Some(instr) => (*instr).into(),
                        _ => SignalValue::Unknown,
                    }
                }
                _ => SignalValue::Unknown,
            };

        // set output
        trace!("--- output {:?}", instr);
        simulator.set_out_value(&self.id, INSTR_MEM_OUT_ID, instr);
        Ok(())
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
