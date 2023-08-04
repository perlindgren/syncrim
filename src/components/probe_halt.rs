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
        let res = self.signal_expr.eval(simulator);
        trace!("signal_expr = {:?}", res);

        match res {
            Ok(true) => {
                // we hit a breakpoint
                Err(Condition::Halt(format!("{:?}", self.signal_expr)))
            }
            _ => Ok(()),
        }
    }

    // reverse simulation, notice does not touch simulator state, its just internal
    fn un_clock(&self) {}
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
