use core::cell::RefCell;
// use log::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
#[cfg(feature = "gui-egui")]
use syncrim::common::EguiComponent;
use syncrim::common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator};

use crate::components::mips_mem_struct::{MemOpSize, MipsMem};
use crate::components::RegFile;
use crate::gui_egui::mips_mem_view_window::MemViewWindow;

pub const INSTR_MEM_PC_ID: &str = "pc";

pub const INSTR_MEM_INSTRUCTION_ID: &str = "instruction";

#[derive(Serialize, Deserialize, Clone)]
pub struct InstrMem {
    pub id: String,
    pub pos: (f32, f32),
    pub pc: Input,
    // should probably not skip mem rc here, since we still need them to point to the same MipsMem
    #[serde(skip)]
    pub mem: Rc<RefCell<MipsMem>>,
    pub mem_view: RefCell<MemViewWindow>,
}

impl InstrMem {
    pub fn new(
        id: String,
        pos: (f32, f32),
        pc_input: Input,
        mem: Rc<RefCell<MipsMem>>,
    ) -> InstrMem {
        let mem_view = MemViewWindow::new(
            id.clone(),
            "instruction memory view".into(),
            Rc::clone(&mem),
        )
        .set_code_view();
        InstrMem {
            id: id,
            pos: pos,
            pc: pc_input,
            mem: mem,
            mem_view: RefCell::new(mem_view),
        }
    }
    pub fn rc_new(
        id: String,
        pos: (f32, f32),
        pc_input: Input,
        mem: Rc<RefCell<MipsMem>>,
    ) -> Rc<InstrMem> {
        Rc::new(InstrMem::new(id, pos, pc_input, mem))
    }
    pub fn set_mem_view_reg(mut self, reg_rc: Rc<RegFile>) -> Self {
        self.mem_view.get_mut().update_regfile(reg_rc);
        self
    }
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
        let memref = Rc::new(RefCell::new(MipsMem::default()));
        Box::new(Rc::new(InstrMem {
            id: id.into(),
            pos: pos,
            pc: dummy_input,
            mem: memref.clone(),
            mem_view: RefCell::new(MemViewWindow::new(
                "dummy".into(),
                "IM dummy".into(),
                memref,
            )),
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

        // update dynamic symbol PC_IM
        self.mem_view.borrow_mut().set_dynamic_symbol("PC_IM", pc);

        // Get a word at PC with the size of 32bits, read as big endian,
        // sign extend doesn't mater since we have 32 bits so extending to 32bits does nothing
        match self.mem.borrow().get(pc, MemOpSize::Word, false, true) {
            Ok(instr) => {
                simulator.set_out_value(&self.id, INSTR_MEM_INSTRUCTION_ID, instr);
                // check if pc is at breakpoint
                match self.mem_view.borrow().is_break_point(&pc) {
                    true => Err(Condition::Halt(format!("Reached breakpoint at {:#0x}", pc))),
                    false => Ok(()),
                }
            }
            Err(_) => Err(Condition::Error(format!("Unaligned Read, PC = {:#0x}", pc))),
        }
    }
}
