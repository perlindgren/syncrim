use crate::common::{Component, Input, Output, OutputType, Ports, Signal, Simulator};
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

use std::{cell::RefCell, collections::HashMap, convert::TryFrom};

#[derive(Serialize, Deserialize)]
pub struct Mem {
    pub id: String,
    pub pos: (f32, f32),
    pub width: f32,
    pub height: f32,

    // ports
    pub data: Input,
    pub addr: Input,
    pub ctrl: Input,
    pub size: Input,

    // memory
    pub memory: Memory,
    // later history... tbd
}

type Memory = RefCell<HashMap<usize, u8>>;

#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)] // Unfortunately Rust does not allow Signal here, we need to cast manually
pub enum DataSize {
    U8,
    U16,
    U32,
    U64,
}

#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)] // Unfortunately Rust does not allow Signal here, we need to cast manually
pub enum MemCtrl {
    None,
    Read,
    Write,
}

#[typetag::serde()]
impl Component for Mem {
    fn to_(&self) {
        println!("Mem");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.data.clone(), self.addr.clone(), self.ctrl.clone()],
                out_type: OutputType::Combinatorial,
                // out 0 data
                // out 1 alignment error
                outputs: vec![Output::Function; 2],
            },
        )
    }

    fn evaluate(&self, simulator: &mut Simulator) {
        let data = simulator.get_input_val(&self.data);
        let addr = simulator.get_input_val(&self.addr);
        let ctrl = simulator.get_input_val(&self.ctrl);
        let ctrl = MemCtrl::try_from(ctrl as u8).unwrap();
        let size = simulator.get_input_val(&self.size);
        let size = DataSize::try_from(size as u8).unwrap();

        match ctrl {
            MemCtrl::Read => {
                println!("read addr {:?} size {:?}", addr, size);
                let data = *self.memory.borrow().get(&(addr as usize)).unwrap_or(&0);
                simulator.set_id_index(&self.id, 0, data as Signal)
            }
            MemCtrl::Write => {
                println!("write addr {:?} size {:?}", addr, size);
                self.memory.borrow_mut().insert(addr as usize, data as u8);
            }
            MemCtrl::None => {
                println!("no read/write");
            }
        }
        println!("memory {:?}", self.memory);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::ComponentStore;
    use crate::components::ProbeOut;
    use std::rc::Rc;

    #[test]
    fn test_mem() {
        let cs = ComponentStore {
            store: vec![
                Rc::new(ProbeOut::new("data")),
                Rc::new(ProbeOut::new("addr")),
                Rc::new(ProbeOut::new("ctrl")),
                Rc::new(ProbeOut::new("size")),
                Rc::new(Mem {
                    id: "mem".to_string(),
                    pos: (0.0, 0.0),
                    width: 0.0,
                    height: 0.0,

                    // ports
                    data: Input::new("data", 0),
                    addr: Input::new("addr", 0),
                    ctrl: Input::new("ctrl", 0),
                    size: Input::new("size", 0),

                    // memory
                    memory: RefCell::new(HashMap::new()),
                    // later history... tbd
                }),
            ],
        };

        let mut clock = 0;
        let mut simulator = Simulator::new(&cs, &mut clock);

        assert_eq!(clock, 1);

        // outputs
        let out = &Input::new("mem", 0);
        let err = &Input::new("mem", 1);

        // reset
        assert_eq!(simulator.get_input_val(out), 0);
        assert_eq!(simulator.get_input_val(err), false as Signal);

        println!("<setup for write 42 to addr 4>");

        simulator.set_id_index("data", 0, 42);
        simulator.set_id_index("addr", 0, 4);
        simulator.set_id_index("ctrl", 0, MemCtrl::Write as Signal);
        simulator.set_id_index("size", 0, DataSize::U8 as Signal);

        println!("sim_state {:?}", simulator.sim_state);

        println!("<clock>");
        simulator.clock(&mut clock);
        println!("sim_state {:?}", simulator.sim_state);

        assert_eq!(clock, 2);
        assert_eq!(simulator.get_input_val(out), 0);
        assert_eq!(simulator.get_input_val(err), false as Signal);

        println!("<setup for read byte from addr 4>");

        simulator.set_id_index("ctrl", 0, MemCtrl::Read as Signal);
        simulator.set_id_index("size", 0, DataSize::U8 as Signal);

        simulator.clock(&mut clock);

        assert_eq!(clock, 3);
        assert_eq!(simulator.get_input_val(out), 42);
        assert_eq!(simulator.get_input_val(err), false as Signal);
    }
}
