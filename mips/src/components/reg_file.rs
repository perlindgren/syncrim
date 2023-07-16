use log::*;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, Range};
use std::{cell::RefCell, rc::Rc};
use syncrim::common::{Component, Input, OutputType, Ports, Signal, Simulator};

#[derive(Serialize, Deserialize)]
pub struct RegFile {
    pub id: String,
    pub pos: (f32, f32),
    pub width: f32,
    pub height: f32,

    // ports
    pub read_addr1: Input,
    pub read_addr2: Input,
    pub write_data: Input,
    pub write_addr: Input,
    pub write_enable: Input,

    // data
    pub registers: RegStore,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegStore(pub Rc<RefCell<[u32; 32]>>);

impl RegStore {
    pub fn new() -> RegStore {
        RegStore(Rc::new(RefCell::new([0; 32])))
    }

    pub fn range() -> Range<u8> {
        Range { start: 0, end: 32 }
    }
}

impl Default for RegStore {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for RegStore {
    type Target = RefCell<[u32; 32]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RegFile {
    fn read_reg(&self, simulator: &Simulator, input: &Input) -> u32 {
        let read_addr = simulator.get_input_val(input) as usize;
        trace!("read_addr {}", read_addr);

        // mips always reads 0;
        if read_addr > 0 {
            self.registers.borrow()[read_addr]
        } else {
            0
        }
    }
}

#[typetag::serde()]
impl Component for RegFile {
    fn to_(&self) {
        trace!("RegFile");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.read_addr1.clone(), self.read_addr2.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec!["reg_a".into(), "reg_b".into()],
            },
        )
    }

    fn evaluate(&self, simulator: &mut Simulator) {
        if simulator.get_input_val(&self.write_enable) == true as Signal {
            let data = simulator.get_input_val(&self.write_data);
            trace!("data {}", data);
            let write_addr = simulator.get_input_val(&self.write_addr) as usize;
            trace!("write_addr {}", write_addr);
            self.registers.borrow_mut()[write_addr] = data;
        }

        // read after write
        let reg_value_a = self.read_reg(simulator, &self.read_addr1);
        trace!("reg_value {}", reg_value_a);
        simulator.set_out_val(&self.id, "reg_a", reg_value_a);

        let reg_value_b = self.read_reg(simulator, &self.read_addr2);
        trace!("reg_value {}", reg_value_b);
        simulator.set_out_val(&self.id, "reg_b", reg_value_b);
    }
}
