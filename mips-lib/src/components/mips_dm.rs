use core::cell::RefCell;
use log::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, rc::Rc};
#[cfg(feature = "gui-egui")]
use syncrim::common::EguiComponent;
use syncrim::{
    common::{Component, Condition, Id, Input, InputPort, OutputType, Ports, Simulator},
    gui_egui::mips_mem_view_window::MemViewWindow,
};

use syncrim::components::{
    mips_mem_struct::{MemOpSize, MemWriteReturn, MipsMem},
    RegFile,
};

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
    // FIXME should probably not skip mem rc here, since we still need them to point to the same MipsMem
    #[serde(skip)]
    pub mem: Rc<RefCell<MipsMem>>,
    pub mem_view: RefCell<MemViewWindow>,

    #[serde(skip)]
    history: RefCell<HashMap<usize, MemWriteReturn>>,
    // used for the un_clock(), this is because the simulator is not passed in un clock and we dont know what cycle we un clock to
    #[serde(skip)]
    cycle: RefCell<usize>,
}

impl DataMem {
    pub fn new(
        id: String,
        pos: (f32, f32),
        address_input: Input,
        data_input: Input,
        op_input: Input,
        write_enable_input: Input,
        mem: Rc<RefCell<MipsMem>>,
    ) -> Self {
        let mem_view = MemViewWindow::new(id.clone(), "Data memory view".into(), Rc::clone(&mem))
            .set_data_view();
        DataMem {
            id: id,
            pos: pos,
            mem: mem,
            address_input: address_input,
            data_input: data_input,
            op_input: op_input,
            write_enable_input: write_enable_input,
            mem_view: RefCell::new(mem_view),
            history: RefCell::new(HashMap::new()),
            cycle: RefCell::default(), // this doesn't mater will be overwritten when clock is called
        }
    }
    pub fn rc_new(
        id: String,
        pos: (f32, f32),
        address_input: Input,
        data_input: Input,
        op_input: Input,
        write_enable_input: Input,
        mem: Rc<RefCell<MipsMem>>,
    ) -> Rc<Self> {
        Rc::new(DataMem::new(
            id,
            pos,
            address_input,
            data_input,
            op_input,
            write_enable_input,
            mem,
        ))
    }
    pub fn set_mem_view_reg(mut self, reg_rc: Rc<RegFile>) -> Self {
        self.mem_view.get_mut().update_regfile(reg_rc);
        self
    }
    fn up_hist(&self, op: MemWriteReturn) {
        self.history.borrow_mut().insert(*self.cycle.borrow(), op);
    }
}

#[typetag::serde()]
impl Component for DataMem {
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
        Box::new(DataMem::rc_new(
            id.to_string(),
            pos,
            dummy_input.clone(),
            dummy_input.clone(),
            dummy_input.clone(),
            dummy_input,
            memref,
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
        *self.cycle.borrow_mut() = simulator.cycle;
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

        // update dynamic symbol PC_IM
        self.mem_view
            .borrow_mut()
            .set_dynamic_symbol("DM_ADRS", address);

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
                let val = self
                    .mem
                    .borrow()
                    .get_unaligned(address, MemOpSize::Byte, SIGNED, true);
                simulator.set_out_value(&self.id, DATA_MEM_READ_DATA_OUT_ID, val);
                Ok(())
            }
            data_op::LOAD_BYTE_U => {
                let val = self
                    .mem
                    .borrow()
                    .get_unaligned(address, MemOpSize::Byte, UNSIGNED, true);
                simulator.set_out_value(&self.id, DATA_MEM_READ_DATA_OUT_ID, val);
                Ok(())
            }
            data_op::LOAD_HALF => {
                match self
                    .mem
                    .borrow()
                    .get(address, MemOpSize::Half, SIGNED, true)
                {
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
                match self
                    .mem
                    .borrow()
                    .get(address, MemOpSize::Half, UNSIGNED, true)
                {
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
                match self
                    .mem
                    .borrow()
                    .get(address, MemOpSize::Word, UNSIGNED, true)
                {
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
                self.up_hist(
                    self.mem
                        .borrow_mut()
                        .write(address, data, MemOpSize::Byte, true),
                );
                simulator.set_out_value(&self.id, DATA_MEM_READ_DATA_OUT_ID, 0);
                Ok(())
            }
            data_op::STORE_HALF => {
                match self
                    .mem
                    .borrow_mut()
                    .write_aligned(address, data, MemOpSize::Half, true)
                {
                    Ok(ret) => {
                        self.up_hist(ret);
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
                match self
                    .mem
                    .borrow_mut()
                    .write_aligned(address, data, MemOpSize::Word, true)
                {
                    Ok(ret) => {
                        self.up_hist(ret);
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
                    if self.mem_view.borrow().is_break_point(&(address & !0b11)) {
                        Err(Condition::Halt(format!(
                            "Read or write at breakpoint address {:#0x}",
                            address
                        )))
                    } else {
                        Ok(())
                    }
                }
            },
            Err(_) => ret,
        }
    }

    fn un_clock(&self) {
        *self.cycle.borrow_mut() -= 1;
        if let Some(op) = self.history.borrow_mut().remove(&*self.cycle.borrow()) {
            self.mem.borrow_mut().revert(op);
        };
    }

    fn reset(&self) {
        // dont need to reset cycle, since cycle is updated in clock

        let mut hist_vec: Vec<(usize, MemWriteReturn)> =
            self.history.borrow_mut().drain().collect();
        // sort vec with largest first
        hist_vec.sort_by(|(a, _), (b, _)| a.cmp(b).reverse());
        let mut mem = self.mem.borrow_mut();
        for (_, op) in hist_vec {
            mem.revert(op);
        }
    }
}
