use std::{collections::HashMap, cell::RefCell};

use serde::{Deserialize, Serialize};
use syncrim::common::{Component, Input, OutputType, Ports, Simulator};

#[derive(Serialize, Deserialize)]
pub struct CSR {
    pub id: String,
    pub pos: (f32, f32),
    pub registers: Registers,
    pub address: Input,
    pub data: Input,
    pub we:Input,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Registers{
    registers: RefCell<HashMap<usize, (u32, CSRPriv)>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum CSRPriv{
    MRO,
    MRW,
}

impl Default for Registers{
    fn default() -> Self{
        let mut map = HashMap::new();
        map.insert(0xF11, (0x0, CSRPriv::MRO)); //mvendorid may be 0x0 
        map.insert(0xF12,(0x0, CSRPriv::MRO)); //marchid may be 0x0
        map.insert(0xF13, (0x0, CSRPriv::MRO)); //mimpid may be 0x0
        map.insert(0xF14, (0x0, CSRPriv::MRO)); //mhartid, since we only have one hart, this must be 0

        map.insert(0x300, (0b11<<11, CSRPriv::MRW)); //mstatus, mpp should be = 3
        map.insert(0x301, (1<<8, CSRPriv::MRW)); //misa, we support RV32I base ISA
        map.insert(0x302, (0x0, CSRPriv::MRW)); //medeleg
        map.insert(0x303, (0x0, CSRPriv::MRW)); //mideleg we only support machine mode, so delegating doesn't make sense
        map.insert(0x304, (0x0, CSRPriv::MRW)); //mie no interrupts enabled as default
        map.insert(0x305, (0x0, CSRPriv::MRW)); //mtvec init from SW
        map.insert(0x306, (0x0, CSRPriv::MRW)); //mcounteren counters are not implemented, so this is inconsequential

        map.insert(0x340, (0x0, CSRPriv::MRW)); //mscratch this is just a data register
        map.insert(0x341, (0x0, CSRPriv::MRW)); //mepc this is 0x0 on init
        map.insert(0x342, (0x0, CSRPriv::MRW)); //mcause same here
        map.insert(0x343, (0x0, CSRPriv::MRW)); //mtval same here
        map.insert(0x344, (0x0, CSRPriv::MRW)); //mip this is technically read only with only machine mode available

        //PMP CSRs unimplemented
        //Machine Counter/Timers unimplemented
        //Machine Counter Setup unimplemented
        //Debug registers unimplemented



        Registers { registers: RefCell::new(map), }
    }
}

#[typetag::serde()]
impl Component for CSR {
    fn to_(&self) {
        println!("CSR");
    }
    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.address.clone(),
                self.data.clone(),
                self.we.clone(),
                ],
                out_type: OutputType::Combinatorial,
                outputs: vec!["output".into()],
            },
        )
    }

    fn clock(&self, simulator: &mut Simulator) {
        // get instr at pc/4
        let _we = simulator.get_input_val(&self.we);

        //simulator.set_out_val(&self.id, "instruction", we);
    }
}
