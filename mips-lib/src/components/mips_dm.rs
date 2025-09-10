#[cfg(feature = "gui-egui")]
use crate::gui_egui::mips_mem_view_window::MemViewWindow;
use core::cell::RefCell;
use std::cell::RefMut;
// use log::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
#[cfg(feature = "gui-egui")]
use syncrim::common::EguiComponent;
use syncrim::common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator};

use crate::components::physical_mem::{MemOpSize, MemWriteReturn, MipsMem};

use super::PhysicalMem;

pub mod data_op {
    pub const NO_OP: u32 = 0;
    pub const LOAD_BYTE: u32 = 1;
    pub const LOAD_BYTE_U: u32 = 2;
    pub const LOAD_HALF: u32 = 3;
    pub const LOAD_HALF_U: u32 = 4;
    pub const LOAD_WORD: u32 = 5;

    pub const STORE_BYTE: u32 = 6;
    pub const STORE_HALF: u32 = 7;
    pub const STORE_WORD: u32 = 8;
}

pub const DATA_MEM_A_IN_ID: &str = "data_mem_address_in";
pub const DATA_MEM_OP_IN_ID: &str = "data_mem_op_in";
pub const DATA_MEM_WRITE_ENABLE_ID: &str = "data_mem_write_enable";
pub const DATA_MEM_WD_IN_ID: &str = "data_mem_write_data_in";

pub const DATA_MEM_READ_DATA_OUT_ID: &str = "data_out";

#[derive(Serialize, Deserialize, Clone)]
pub struct DataMem {
    pub id: String,
    pub pos: (f32, f32),
    pub address_input: Input,
    pub data_input: Input,
    pub op_input: Input,
    pub write_enable_input: Input,
    pub phys_mem_id: String,
    pub regfile_id: String,

    #[cfg(feature = "gui-egui")]
    pub mem_view: RefCell<MemViewWindow>,

    pub dynamic_symbols: RefCell<HashMap<String, (u32, bool)>>,

    #[serde(skip)]
    pub input_address_history: RefCell<Vec<u32>>,
}

impl DataMem {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: String,
        pos: (f32, f32),
        address_input: Input,
        data_input: Input,
        op_input: Input,
        write_enable_input: Input,
        phys_mem_id: String,
        regfile_id: String,
    ) -> Self {
        #[cfg(feature = "gui-egui")]
        let mem_view =
            MemViewWindow::new(id.clone(), "Data memory view".into()).set_data_view(None);
        DataMem {
            id,
            pos,
            phys_mem_id,
            address_input,
            data_input,
            op_input,
            write_enable_input,
            #[cfg(feature = "gui-egui")]
            mem_view: RefCell::new(mem_view),

            regfile_id,
            dynamic_symbols: RefCell::new(HashMap::new()),
            input_address_history: RefCell::new(vec![]),
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn rc_new(
        id: String,
        pos: (f32, f32),
        address_input: Input,
        data_input: Input,
        op_input: Input,
        write_enable_input: Input,
        phys_mem_id: String,
        regfile_id: String,
    ) -> Rc<Self> {
        Rc::new(DataMem::new(
            id,
            pos,
            address_input,
            data_input,
            op_input,
            write_enable_input,
            phys_mem_id,
            regfile_id,
        ))
    }
    /// This gets a &PhysicalMem from the component named self.phys_mem_id
    ///
    /// # Panics
    ///
    /// Panics if This functions panics if phys_mem_id is not found in simulator
    /// or phys_mem_id is not of type PhysicalMem
    #[allow(clippy::expect_fun_call)]
    fn get_phys_mem<'a>(&self, sim: &'a Simulator) -> &'a PhysicalMem {
        let v = &sim.ordered_components;
        let comp = v
            .iter()
            .find(|x| x.get_id_ports().0 == self.phys_mem_id)
            .expect(&format!("cant find {} in simulator", self.phys_mem_id));
        // deref to get &dyn EguiComponent
        let comp_any = (*comp).as_any();
        let phys_mem: &PhysicalMem = comp_any
            .downcast_ref()
            .expect("can't downcast to physical memory");
        phys_mem
    }

    fn get_mut_mem<'a>(&self, sim: &'a Simulator) -> RefMut<'a, MipsMem> {
        self.get_phys_mem(sim).mem.borrow_mut()
    }

    fn up_hist(&self, sim: &Simulator, op: MemWriteReturn, cycle: usize) {
        self.get_phys_mem(sim)
            .history
            .borrow_mut()
            .insert(cycle, op);
    }
    fn up_cycle(&self, sim: &Simulator) {
        let cycle = sim.cycle;
        let _ = self.get_phys_mem(sim).cycle.replace(cycle);
    }

    // update DM_ADRS to the given new_adress
    fn update_dynamic_symbols(&self, new_adress: u32) {
        let mut new_dynamic_symbols = self.dynamic_symbols.borrow_mut().clone();
        if new_dynamic_symbols.contains_key("DM_ADRS") {
            new_dynamic_symbols.insert(
                "DM_ADRS".to_string(),
                (
                    new_adress,
                    new_dynamic_symbols.get_key_value("DM_ADRS").unwrap().1 .1,
                ),
            );
        }
        *self.dynamic_symbols.borrow_mut() = new_dynamic_symbols;
    }
}

#[typetag::serde()]
impl Component for DataMem {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn to_(&self) {}
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, id: &str, pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        let dummy_input = Input::new("dummy", "out");
        Box::new(DataMem::rc_new(
            id.to_string(),
            pos,
            dummy_input.clone(),
            dummy_input.clone(),
            dummy_input.clone(),
            dummy_input,
            "dummy".into(),
            "dummy".into(),
        ))
    }

    fn set_id_port(&mut self, target_port_id: Id, new_input: Input) {
        match target_port_id.as_str() {
            DATA_MEM_A_IN_ID => self.address_input = new_input,
            DATA_MEM_OP_IN_ID => self.op_input = new_input,
            DATA_MEM_WRITE_ENABLE_ID => self.op_input = new_input,
            DATA_MEM_WD_IN_ID => self.write_enable_input = new_input,
            _ => {}
        }
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports::new(
                vec![
                    &InputPort {
                        port_id: DATA_MEM_A_IN_ID.to_string(),
                        input: self.address_input.clone(),
                    },
                    &InputPort {
                        port_id: DATA_MEM_OP_IN_ID.to_string(),
                        input: self.op_input.clone(),
                    },
                    &InputPort {
                        port_id: DATA_MEM_WD_IN_ID.to_string(),
                        input: self.data_input.clone(),
                    },
                    &InputPort {
                        port_id: DATA_MEM_WRITE_ENABLE_ID.to_string(),
                        input: self.write_enable_input.clone(),
                    },
                ],
                OutputType::Combinatorial,
                vec![DATA_MEM_READ_DATA_OUT_ID],
            ),
        )
    }

    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        let cycle = simulator.cycle;
        self.up_cycle(simulator);
        // get instr at pc/4s
        let address: u32 = simulator
            .get_input_value(&self.address_input)
            .try_into()
            .unwrap();

        // is operation load or store ect
        let mem_op: u32 = simulator
            .get_input_value(&self.op_input)
            .try_into()
            .unwrap();

        let write_enable: bool = simulator
            .get_input_value(&self.write_enable_input)
            .try_into()
            .unwrap();

        let data: u32 = simulator
            .get_input_value(&self.data_input)
            .try_into()
            .unwrap();
        self.input_address_history
            .borrow_mut()
            .push(self.dynamic_symbols.borrow().get("DM_ADRS").unwrap().0);
        self.update_dynamic_symbols(address);

        // check if write enable and mem op match
        let is_write_enable_valid = match mem_op {
            data_op::LOAD_BYTE
            | data_op::LOAD_BYTE_U
            | data_op::LOAD_HALF
            | data_op::LOAD_HALF_U
            | data_op::LOAD_WORD => !write_enable,
            data_op::STORE_BYTE | data_op::STORE_HALF | data_op::STORE_WORD => write_enable,
            _ => true,
        };

        if !is_write_enable_valid {
            return Err(Condition::Error(format!(
                "write_enable ({}) doesn't match mem_op ({})",
                write_enable, mem_op
            )));
        }
        const SIGNED: bool = true;
        const UNSIGNED: bool = false;

        let ret: Result<(), Condition> = match mem_op {
            data_op::NO_OP => {
                simulator.set_out_value(&self.id, DATA_MEM_READ_DATA_OUT_ID, 0);
                Ok(())
            }
            data_op::LOAD_BYTE => {
                let val = self.get_mut_mem(simulator).get_unaligned(
                    address,
                    MemOpSize::Byte,
                    SIGNED,
                    true,
                );
                simulator.set_out_value(&self.id, DATA_MEM_READ_DATA_OUT_ID, val);
                Ok(())
            }
            data_op::LOAD_BYTE_U => {
                let val = self.get_mut_mem(simulator).get_unaligned(
                    address,
                    MemOpSize::Byte,
                    UNSIGNED,
                    true,
                );
                simulator.set_out_value(&self.id, DATA_MEM_READ_DATA_OUT_ID, val);
                Ok(())
            }
            data_op::LOAD_HALF => {
                let l_ret = self
                    .get_mut_mem(simulator)
                    .get(address, MemOpSize::Half, SIGNED, true);
                match l_ret {
                    Ok(val) => {
                        simulator.set_out_value(&self.id, DATA_MEM_READ_DATA_OUT_ID, val);
                        Ok(())
                    }
                    Err(_) => Err(Condition::Error(format!(
                        "Tried to read unaligned half word, address {:#0x}",
                        address
                    ))),
                }
            }
            data_op::LOAD_HALF_U => {
                let l_ret =
                    self.get_mut_mem(simulator)
                        .get(address, MemOpSize::Half, UNSIGNED, true);
                match l_ret {
                    Ok(val) => {
                        simulator.set_out_value(&self.id, DATA_MEM_READ_DATA_OUT_ID, val);
                        Ok(())
                    }
                    Err(_) => Err(Condition::Error(format!(
                        "Tried to read unaligned half word, address {:#0x}",
                        address
                    ))),
                }
            }
            data_op::LOAD_WORD => {
                let l_ret =
                    self.get_mut_mem(simulator)
                        .get(address, MemOpSize::Word, UNSIGNED, true);
                match l_ret {
                    Ok(val) => {
                        simulator.set_out_value(&self.id, DATA_MEM_READ_DATA_OUT_ID, val);
                        Ok(())
                    }
                    Err(_) => Err(Condition::Error(format!(
                        "Tried to read unaligned word, address {:#0x}",
                        address
                    ))),
                }
            }
            data_op::STORE_BYTE => {
                let ret = self
                    .get_mut_mem(simulator)
                    .write(address, data, MemOpSize::Byte, true);
                self.up_hist(simulator, ret, cycle);
                simulator.set_out_value(&self.id, DATA_MEM_READ_DATA_OUT_ID, 0);
                Ok(())
            }
            data_op::STORE_HALF => {
                let w_ret =
                    self.get_mut_mem(simulator)
                        .write_aligned(address, data, MemOpSize::Half, true);
                match w_ret {
                    Ok(ret) => {
                        self.up_hist(simulator, ret, cycle);
                        simulator.set_out_value(&self.id, DATA_MEM_READ_DATA_OUT_ID, 0);
                        Ok(())
                    }
                    Err(_) => Err(Condition::Error(format!(
                        "Tried to write unaligned half word, address {:#0x}",
                        address
                    ))),
                }
            }
            data_op::STORE_WORD => {
                let w_ret =
                    self.get_mut_mem(simulator)
                        .write_aligned(address, data, MemOpSize::Word, true);
                match w_ret {
                    Ok(ret) => {
                        self.up_hist(simulator, ret, cycle);
                        simulator.set_out_value(&self.id, DATA_MEM_READ_DATA_OUT_ID, 0);
                        Ok(())
                    }
                    Err(_) => Err(Condition::Error(format!(
                        "Tried to write unaligned word, address {:#0x}",
                        address
                    ))),
                }
            }
            _ => Err(Condition::Error(format!("unknown mem op {}", mem_op))),
        };
        // test breakpoint
        match ret {
            Ok(_) => match mem_op {
                data_op::NO_OP => Ok(()),
                _ => {
                    #[cfg(feature = "gui-egui")]
                    if self.mem_view.borrow().is_break_point(&(address & !0b11)) {
                        Err(Condition::Halt(format!(
                            "Read or write at breakpoint address {:#0x}",
                            address
                        )))
                    } else {
                        Ok(())
                    }
                    #[cfg(not(feature = "gui-egui"))]
                    Ok(())
                }
            },
            Err(_) => ret,
        }
    }
    // set input_adress to what it was the previous cycle
    fn un_clock(&self, _simulator: &Simulator) {
        let previous_adress: u32 = self.input_address_history.borrow_mut().pop().unwrap();
        self.update_dynamic_symbols(previous_adress);
    }
    // if the simulator is reset and input_adress_history isn't empty: move over dynamic_symbol settings
    // while resetting values and adresses
    fn reset(&self) {
        if self.input_address_history.borrow().len() > 0 {
            let start_adress = self.input_address_history.borrow()[0];
            let current_symbol_keys: Vec<String> =
                self.dynamic_symbols.borrow().keys().cloned().collect();

            let mut new_symbols: HashMap<String, (u32, bool)> = HashMap::new();
            for symbol_name in current_symbol_keys {
                new_symbols.insert(
                    symbol_name.clone(),
                    (
                        start_adress,
                        self.dynamic_symbols.borrow().get(&symbol_name).unwrap().1,
                    ),
                );
            }
            *self.dynamic_symbols.borrow_mut() = new_symbols;
            self.input_address_history.borrow_mut().clear();
        }
    }
}
