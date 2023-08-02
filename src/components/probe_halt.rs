use crate::common::{
    Component, Condition, Id, Input, OutputType, Ports, Signal, SignalExpr, Simulator,
};
use log::*;
use serde::{Deserialize, Serialize};
use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct ProbeHalt {
    pub(crate) id: Id,
    pub(crate) pos: (f32, f32),
    pub(crate) inputs: Vec<Input>,
    pub(crate) signal_expr: SignalExpr,
}

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct TextSignal {
//     pub text: String,
//     pub signal: SignalExpr,
// }

#[typetag::serde]
impl Component for ProbeHalt {
    fn to_(&self) {
        trace!("ProbeHalt");
    }

    fn get_id_ports(&self) -> (Id, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: self.inputs.clone(),
                out_type: OutputType::Combinatorial,
                outputs: vec![],
            },
        )
    }

    // propagate editable value
    fn clock(&self, simulator: &mut Simulator) -> Result<(), Condition> {
        // let mut history = self.edit_history.write().unwrap();
        // trace!("{} history {:?}", self.id, history);
        // let current = history.last().unwrap().clone();
        // // set output to current value
        // simulator.set_out_value(&self.id, "out", current.signal.get_value());
        // // push to prepare data for next;
        // history.push(current);
        Ok(())
    }

    // reverse simulation, notice does not touch simulator state, its just internal
    fn un_clock(&self) {
        // let mut edit_history = self.edit_history.write().unwrap();
        // trace!("{} history {:?}", self.id, edit_history);
        // let _next = edit_history.pop().unwrap(); // pop the next editable value
        // let _current = edit_history.pop().unwrap(); // pop current editable value
        // let prev = edit_history.pop().unwrap(); // pop the prev editable value
        // trace!("next {:?}", _next);
        // trace!("current {:?}", _current);
        // trace!("prev {:?}", prev);
        // edit_history.push(prev.clone()); // push as current
        // edit_history.push(prev); // push as next (to be edited)
    }
}

impl ProbeHalt {
    pub fn new(id: &str, pos: (f32, f32), inputs: Vec<Input>, signal_expr: SignalExpr) -> Self {
        ProbeHalt {
            id: id.into(),
            pos,
            inputs,
            signal_expr,
        }
    }

    pub fn rc_new(
        id: &str,
        pos: (f32, f32),
        inputs: Vec<Input>,
        signal_expr: SignalExpr,
    ) -> Rc<Self> {
        Rc::new(ProbeHalt::new(id, pos, inputs, signal_expr))
    }
}
