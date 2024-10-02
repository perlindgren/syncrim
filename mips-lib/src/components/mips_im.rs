use core::cell::RefCell;
// use log::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
#[cfg(feature = "gui-egui")]
use syncrim::common::EguiComponent;
use syncrim::common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator};

use crate::components::physical_mem::MemOpSize;
#[cfg(feature = "gui-egui")]
use crate::gui_egui::mips_mem_view_window::MemViewWindow;

use super::PhysicalMem;

pub const INSTR_MEM_PC_ID: &str = "pc";

pub const INSTR_MEM_INSTRUCTION_ID: &str = "instruction";

#[derive(Serialize, Deserialize, Clone)]
pub struct InstrMem {
    pub id: String,
    pub pos: (f32, f32),
    pub pc: Input,
    // All components who deal with memory acess this
    pub phys_mem_id: String,
    pub regfile_id: String,
    #[cfg(feature = "gui-egui")]
    pub mem_view: RefCell<MemViewWindow>,
}

impl InstrMem {
    pub fn new(
        id: String,
        pos: (f32, f32),
        pc_input: Input,
        phys_mem_id: String,
        regfile_id: String,
    ) -> InstrMem {
        #[cfg(feature = "gui-egui")]
        let mem_view =
            MemViewWindow::new(id.clone(), "instruction memory view".into()).set_code_view(None);
        InstrMem {
            id,
            pos,
            pc: pc_input,
            phys_mem_id,
            #[cfg(feature = "gui-egui")]
            mem_view: RefCell::new(mem_view),
            regfile_id,
        }
    }
    pub fn rc_new(
        id: String,
        pos: (f32, f32),
        pc_input: Input,
        phys_mem_id: String,
        regfile_id: String,
    ) -> Rc<InstrMem> {
        Rc::new(InstrMem::new(id, pos, pc_input, phys_mem_id, regfile_id))
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
        Box::new(Rc::new(InstrMem {
            id: id.into(),
            pos,
            pc: dummy_input,
            phys_mem_id: "dummy".into(),
            #[cfg(feature = "gui-egui")]
            mem_view: RefCell::new(MemViewWindow::new("dummy".into(), "IM dummy".into())),
            regfile_id: "dummy".into(),
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

        // this is inside a {} to make sure our simulator borrow is returned before its used to set signal
        #[allow(clippy::expect_fun_call)]
        let option_instr = {
            let v = &simulator.ordered_components;
            let comp = v
                .iter()
                .find(|x| x.get_id_ports().0 == self.phys_mem_id)
                .expect(&format!("cant find {} in simulator", self.phys_mem_id));
            // deref to get Rc
            // deref again to get &dyn EguiComponent
            let comp_any = (**comp).as_any();
            let phys_mem: &PhysicalMem = comp_any
                .downcast_ref()
                .expect("can't downcast to physical memory");
            phys_mem
                .mem
                .borrow_mut()
                .get(pc, MemOpSize::Word, false, true)
        };

        // update dynamic symbol PC_IM
        #[cfg(feature = "gui-egui")]
        self.mem_view.borrow_mut().set_dynamic_symbol("PC_IM", pc);

        // Get a word at PC with the size of 32bits, read as big endian,
        // sign extend doesn't mater since we have 32 bits so extending to 32bits does nothing
        match option_instr {
            Ok(instr) => {
                simulator.set_out_value(&self.id, INSTR_MEM_INSTRUCTION_ID, instr);
                // check if pc is at breakpoint
                #[cfg(feature = "gui-egui")]
                match self.mem_view.borrow().is_break_point(&pc) {
                    true => Err(Condition::Halt(format!("Reached breakpoint at {:#0x}", pc))),
                    false => Ok(()),
                }
                #[cfg(not(feature = "gui-egui"))]
                Ok(())
            }
            Err(_) => Err(Condition::Error(format!("Unaligned Read, PC = {:#0x}", pc))),
        }
    }
}
