use crate::common::{Component, Id, OutputType, Ports, Signal, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};

#[derive(Serialize, Deserialize, Clone)]
pub struct ProbeEdit {
    pub id: Id,
    pub pos: (f32, f32),
    pub history: Rc<RefCell<Vec<Signal>>>, // will contain the next editable value
}

#[typetag::serde]
impl Component for ProbeEdit {
    fn to_(&self) {
        trace!("ProbeEdit");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports::new(
                // Constants do not take any inputs
                vec![],
                OutputType::Combinatorial,
                // Single output value
                vec!["out"],
            ),
        )
    }

    // propagate editable value
    fn clock(&self, simulator: &mut Simulator) {
        let history = self.history.borrow();
        error!("{} history {:?}", self.id, history);
        let current = *history.last().unwrap();

        // set output to current value
        simulator.set_out_val(&self.id, "out", current);
        drop(history);
        // push to prepare data for next;
        self.history.borrow_mut().push(current)
    }

    // reverse simulation, notice does not touch simulator state, its just internal
    fn un_clock(&self, simulator: &mut Simulator) {
        let mut history = self.history.borrow_mut();
        error!("{} history {:?}", self.id, history);
        let _current = history.pop().unwrap(); // pop the current editable value
        error!("current {:?}", _current);
        drop(history);
        let history = self.history.borrow();
        error!("new {} history {:?}", self.id, history);
    }
}

impl ProbeEdit {
    pub fn new(id: &str, pos: (f32, f32)) -> Self {
        ProbeEdit {
            id: id.into(),
            pos,
            // initiate internal history
            history: Rc::new(RefCell::new(vec![0])),
        }
    }
}
