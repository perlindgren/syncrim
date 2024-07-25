#[cfg(feature = "gui-egui")]
use crate::common::EguiComponent;
use crate::common::{
    Component, Condition, Id, Input, InputPort, OutputType, Ports, SignalSigned, SignalUnsigned,
    SignalValue, Simulator,
};
use core::cell::RefCell;
use core::ops::Range;
use log::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

pub const INSTR_MEM_PC_ID: &str = "pc";

pub const INSTR_MEM_INSTRUCTION_ID: &str = "instruction";

pub const INSTR_MEM_HEIGHT: f32 = 100.0;
pub const INSTR_MEM_WIDTH: f32 = 100.0;

#[derive(Serialize, Deserialize, Clone)]
pub struct InstrMem {
    pub width: f32,
    pub height: f32,
    pub id: String,
    pub pos: (f32, f32),
    #[serde(skip)]
    pub bytes: BTreeMap<usize, u8>,
    pub pc: Input,
    pub range: Range<usize>,
    #[serde(skip)]
    pub breakpoints: Rc<RefCell<HashSet<usize>>>,
    #[serde(skip)]
    pub symbols: HashMap<usize, String>,
    pub le: bool,
}

#[typetag::serde()]
impl Component for InstrMem {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_(&self) {
        //println!("InstrMem");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(Rc::new(InstrMem {
            width: INSTR_MEM_WIDTH,
            height: INSTR_MEM_HEIGHT,
            id: id.to_string(),
            pos: (pos.0, pos.1),
            bytes: BTreeMap::new(),
            pc: dummy_input,
            range: Range {
                start: 0,
                end: 0x1000,
            },
            breakpoints: Rc::new(RefCell::new(HashSet::new())),
            symbols: HashMap::new(),
            le: true,
        }))
    }
    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        if target_port_id.as_str() == INSTR_MEM_PC_ID {
            self.pc = new_input;
        }
    }
    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![&InputPort {
                    port_id: INSTR_MEM_PC_ID.to_string(),
                    input: self.pc.clone(),
                }],
                OutputType::Combinatorial,
                vec![INSTR_MEM_INSTRUCTION_ID],
            ),
        )
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get instr at pc/4
        let pc: u32 = simulator.get_input_value(&self.pc).try_into().unwrap();
        let instr = if !self.le {
            (*self.bytes.get(&((pc) as usize)).unwrap() as u32) << 24
                | (*self.bytes.get(&((pc + 1) as usize)).unwrap() as u32) << 16
                | (*self.bytes.get(&((pc + 2) as usize)).unwrap() as u32) << 8
                | (*self.bytes.get(&((pc + 3) as usize)).unwrap() as u32)
        } else {
            (*self.bytes.get(&((pc) as usize)).unwrap() as u32)
                | (*self.bytes.get(&((pc + 1) as usize)).unwrap() as u32) << 8
                | (*self.bytes.get(&((pc + 2) as usize)).unwrap() as u32) << 16
                | (*self.bytes.get(&((pc + 3) as usize)).unwrap() as u32) << 24
        };
        //the asm_riscv crate incorrectly panics when trying from instead of
        //returning Err, catch it and handle instead
        let instruction_fmt = {
            format!(
                "{:?}",
                match asm_riscv::I::try_from(instr) {
                    Ok(i) => riscv_asm_strings::StringifyUpperHex::to_string(&i),
                    Err(_) => "Unknown instruction".to_string(),
                }
            )
        };
        trace!("instruction: {}", instruction_fmt);
        trace!("pc:0x{:08x}", pc);
        // set output
        simulator.set_out_value(&self.id, "instruction", instr);
        if !self.breakpoints.borrow().contains(&(pc as usize)) {
            Ok(())
        } else {
            Err(Condition::Halt(format!("Breakpoint at {}", pc)))
        }
    }
}
