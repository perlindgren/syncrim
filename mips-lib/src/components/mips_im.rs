use core::cell::RefCell;
use std::collections::HashMap;
// use log::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
#[cfg(feature = "gui-egui")]
use syncrim::common::EguiComponent;
use syncrim::common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator};

use crate::components::{physical_mem::MemOpSize, MemLoadError};
#[cfg(feature = "gui-egui")]
use crate::gui_egui::mips_mem_view_window::MemViewWindow;

use super::PhysicalMem;

pub const INSTR_MEM_PC_ID: &str = "pc";

pub const INSTR_MEM_INSTRUCTION_ID: &str = "instruction";

#[derive(Serialize, Deserialize)]
pub struct InstrMem {
    pub id: String,
    pub pos: (f32, f32),
    pub pc: Input,
    // All components who deal with memory acess this
    pub phys_mem_id: String,
    pub regfile_id: String,
    #[cfg(feature = "gui-egui")]
    pub mem_view: RefCell<MemViewWindow>,
    #[cfg(feature = "gui-egui")]
    #[serde(skip)]
    pub load_err: RefCell<Option<MemLoadError>>,
    #[serde(skip)]
    pub pc_dm_history: RefCell<Vec<u32>>,
    pub dynamic_symbols: RefCell<HashMap<String, (u32, bool)>>,
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
            regfile_id,
            #[cfg(feature = "gui-egui")]
            mem_view: RefCell::new(mem_view),
            #[cfg(feature = "gui-egui")]
            load_err: RefCell::new(None),
            pc_dm_history: RefCell::new(vec![]),
            dynamic_symbols: RefCell::new(HashMap::new()),
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
    pub fn clock_dynamic_symbols(&self, new_pc: u32) {
        let mut dynamic_symbols = self.dynamic_symbols.borrow_mut();
        let mut pc_dm_history = self.pc_dm_history.borrow_mut();

        if dynamic_symbols.contains_key("PC_DM")
            && dynamic_symbols.contains_key("PC_EX")
            && dynamic_symbols.contains_key("PC_DE")
        {
            // Store previous PC_DM, because unclocking doesn't provide info about PC_DM-stage
            pc_dm_history.push(dynamic_symbols.get_mut("PC_DM").unwrap().0);

            dynamic_symbols.get_mut("PC_DM").unwrap().0 = dynamic_symbols.get("PC_EX").unwrap().0;
            dynamic_symbols.get_mut("PC_EX").unwrap().0 = dynamic_symbols.get("PC_DE").unwrap().0;
            dynamic_symbols.get_mut("PC_DE").unwrap().0 = dynamic_symbols.get("PC_IM").unwrap().0;
        }
        dynamic_symbols.get_mut("PC_IM").unwrap().0 = new_pc;
    }

    pub fn unclock_dynamic_symbols(&self, new_pc: u32) {
        let mut dynamic_symbols = self.dynamic_symbols.borrow_mut();
        dynamic_symbols.get_mut("PC_IM").unwrap().0 = new_pc;
        if dynamic_symbols.contains_key("PC_DM")
            && dynamic_symbols.contains_key("PC_EX")
            && dynamic_symbols.contains_key("PC_DE")
        {
            dynamic_symbols.get_mut("PC_DE").unwrap().0 = dynamic_symbols.get("PC_EX").unwrap().0;
            dynamic_symbols.get_mut("PC_EX").unwrap().0 = dynamic_symbols.get("PC_DM").unwrap().0;
            dynamic_symbols.get_mut("PC_DM").unwrap().0 =
                self.pc_dm_history.borrow_mut().pop().unwrap();
        }
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
            mem_view: RefCell::new(MemViewWindow::new("dummy".into(), "IM dummy".into())),
            regfile_id: "dummy".into(),
            load_err: RefCell::new(None),
            pc_dm_history: RefCell::new(vec![]),
            dynamic_symbols: RefCell::new(HashMap::new()),
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

        self.clock_dynamic_symbols(pc);

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
    // set component to what it was the previous cycle
    fn un_clock(&self, simulator: &Simulator) {
        let pc: u32 = simulator.get_input_value(&self.pc).try_into().unwrap();
        self.unclock_dynamic_symbols(pc);
    }
    // if the simulator is reset and pc_dm_history isn't empty: move over dynamic_symbol settings
    // while resetting adresses
    fn reset(&self) {
        if self.pc_dm_history.borrow().len() > 0 {
            let start_pc = self.pc_dm_history.borrow()[0];
            let symbol_keys: Vec<String> = self.dynamic_symbols.borrow().keys().cloned().collect();

            let mut dynamic_symbols = self.dynamic_symbols.borrow_mut();
            for symbol_name in symbol_keys {
                dynamic_symbols.get_mut(&symbol_name).unwrap().0 = start_pc;
            }
        }
    }
}
