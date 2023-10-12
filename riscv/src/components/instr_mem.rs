use asm_riscv::{self};
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap, HashSet},
    ops::Range,
    rc::Rc,
};

use log::trace;
use riscv_asm_strings::Stringify;
use serde::{Deserialize, Serialize};
#[cfg(feature = "gui-egui")]
use syncrim::common::EguiComponent;
use syncrim::common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator};
pub const INSTR_MEM_PC_ID: &str = "pc";

pub const INSTR_MEM_INSTRUCTION_ID: &str = "instruction";

pub const INSTR_MEM_HEIGHT: f32 = 100.0;
pub const INSTR_MEM_WIDTH: f32 = 200.0;

#[derive(Serialize, Deserialize, Clone)]
pub struct InstrMem {
    pub width: f32,
    pub height: f32,
    pub id: String,
    pub pos: (f32, f32),
    pub bytes: BTreeMap<usize, u8>,
    pub pc: Input,
    pub range: Range<usize>,
    pub breakpoints: Rc<RefCell<HashSet<usize>>>,
    pub symbols: HashMap<usize, String>,
    pub le: bool,
}

#[typetag::serde()]
impl Component for InstrMem {
    fn to_(&self) {
        println!("InstrMem");
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
        }))
    }
    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            INSTR_MEM_PC_ID => self.pc = new_input,
            _ => (),
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
        let instr = if !self.le{ (*self.bytes.get(&((pc) as usize)).unwrap() as u32) << 24
            | (*self.bytes.get(&((pc + 1) as usize)).unwrap() as u32) << 16
            | (*self.bytes.get(&((pc + 2) as usize)).unwrap() as u32) << 8
            | (*self.bytes.get(&((pc + 3) as usize)).unwrap() as u32)}
        else{(*self.bytes.get(&((pc) as usize)).unwrap() as u32)
            | (*self.bytes.get(&((pc + 1) as usize)).unwrap() as u32) << 8
            | (*self.bytes.get(&((pc + 2) as usize)).unwrap() as u32) << 16
            | (*self.bytes.get(&((pc + 3) as usize)).unwrap() as u32) << 24};
        //the asm_riscv crate incorrectly panics when trying from instead of
        //returning Err, catch it and handle instead
        let instruction_fmt = {
            format!(
                "{:?}",
                match asm_riscv::I::try_from(instr) {
                    Ok(i) => i.to_string(),
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
        let mut instr_mem = BTreeMap::new();
        for i in 0u32..6u32 {
            let bytes = i.to_be_bytes();
            instr_mem.insert((i * 4) as usize, bytes[0]);
            instr_mem.insert((i * 4 + 1) as usize, bytes[1]);
            instr_mem.insert((i * 4 + 2) as usize, bytes[2]);
            instr_mem.insert((i * 4 + 3) as usize, bytes[3]);
        }
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("pc")),
                Rc::new(InstrMem {
                    width: 0.0,
                    height: 0.0,
                    id: "imem".to_string(),
                    pos: (0.0, 0.0),
                    pc: Input::new("pc", "out"),
                    bytes: instr_mem,
                    range: Range {
                        start: 0,
                        end: 0x1000,
                    },
                    breakpoints: Rc::new(RefCell::new(HashSet::new())),
                    symbols: HashMap::new(),
                    le: true,
                }),
            ],
        };

        let mut simulator = Simulator::new(cs).unwrap();
        assert_eq!(simulator.cycle, 1);

        // outputs
        let imem_out = &Input::new("imem", "instruction");
        for i in 0..6 {
            simulator.set_out_value("pc", "out", i * 4);
            simulator.clock();
            assert_eq!(simulator.get_input_value(imem_out), i.into());
        }
    }
}
