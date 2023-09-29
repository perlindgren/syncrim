use log::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use syncrim::common::{Component, Condition, Id, Input, OutputType, Ports, Simulator};
#[derive(Serialize, Deserialize)]
pub struct Sysclk {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),

    clock: RefCell<u32>,
}

#[typetag::serde]
impl Component for Sysclk {
    fn to_(&self) {
        trace!("register");
    }
    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                // Vector of inputs
                vec![],
                OutputType::Combinatorial,
                vec!["clock"],
            ),
        )
    }

    // propagate input value to output
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // get input value
        let clock = self.clock.borrow().clone();
        self.clock.replace(clock + 1);
        // set output
        simulator.set_out_value(&self.id, "clock", self.clock.borrow().clone());
        trace!("clock: {}", self.clock.borrow().clone());
        Ok(())
    }
}

impl Sysclk {
    pub fn new(id: &str, pos: (f32, f32)) -> Self {
        Sysclk {
            id: id.to_string(),
            pos,
            clock: RefCell::new(0),
        }
    }
}
