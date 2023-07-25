use crate::common::{Component, Id, OutputType, Ports, Signal, Simulator};
use log::*;
use serde::{Deserialize, Serialize};
use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct ProbeEdit {
    pub id: Id,
    pub pos: (f32, f32),
    pub edit_history: Arc<RwLock<Vec<TextSignal>>>, // will contain the next editable value
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TextSignal {
    pub text: String,
    pub signal: Signal,
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
        let mut history = self.edit_history.write().unwrap();
        trace!("{} history {:?}", self.id, history);
        let current = history.last().unwrap().clone();
        // set output to current value
        simulator.set_out_val(&self.id, "out", current.signal);
        // push to prepare data for next;
        history.push(current);
    }

    // reverse simulation, notice does not touch simulator state, its just internal
    fn un_clock(&self) {
        let mut edit_history = self.edit_history.write().unwrap();
        trace!("{} history {:?}", self.id, edit_history);
        let _next = edit_history.pop().unwrap(); // pop the next editable value
        let _current = edit_history.pop().unwrap(); // pop current editable value
        let prev = edit_history.pop().unwrap(); // pop the prev editable value
        trace!("next {:?}", _next);
        trace!("current {:?}", _current);
        trace!("prev {:?}", prev);
        edit_history.push(prev.clone()); // push as current
        edit_history.push(prev); // push as next (to be edited)
    }
}

impl ProbeEdit {
    pub fn new(id: &str, pos: (f32, f32)) -> Self {
        ProbeEdit {
            id: id.into(),
            pos,
            // initiate internal history
            edit_history: Arc::new(RwLock::new(vec![TextSignal {
                text: "0".to_string(),
                signal: Signal::Data(0),
            }])),
        }
    }

    pub fn rc_new(id: &str, pos: (f32, f32)) -> Rc<Self> {
        Rc::new(ProbeEdit::new(id, pos))
    }
}
