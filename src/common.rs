use petgraph::Graph;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use vizia::prelude::*;

pub type Signal = u32;
pub type SignedSignal = i32;

#[derive(Lens, Clone)]
pub struct Simulator {
    pub id_start_index: IdStartIndex,

    // Components stored in topological evaluation order
    pub ordered_components: Components,
    pub sim_state: Vec<Signal>,
    pub history: Vec<Vec<Signal>>,
    pub component_ids: Vec<String>,
    pub graph: Graph<String, ()>,
}

type Components = Vec<Rc<dyn ViziaComponent>>;

#[derive(Serialize, Deserialize)]
pub struct ComponentStore {
    // pub path: String,
    pub store: Components,
}

// a mapping (id -> index)
// where index is the start index in the LensValues vector
// e.g., `mux1` starts at index 15, then its
// select input is index 15
// the first input is index 16
// the second input is index 17, etc.
pub type IdStartIndex = HashMap<String, usize>;

// Common functionality for all components
#[typetag::serde(tag = "type")]
pub trait Component {
    // placeholder
    fn to_(&self) {}

    /// returns the (id, Ports) of the component
    fn get_id_ports(&self) -> (String, Ports);

    /// evaluation function
    fn evaluate(&self, _simulator: &mut Simulator) {}
}

#[typetag::serde(tag = "type")]
pub trait ViziaComponent:Component {
    /// create Vizia view
    fn view(&self, _cx: &mut Context) {}
}

#[derive(Debug, Clone)]
pub struct Ports {
    pub inputs: Vec<Input>,
    pub out_type: OutputType,
    pub outputs: Vec<Output>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Input {
    pub id: String,
    pub index: usize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum OutputType {
    // Will be evaluated as a combinatorial function from inputs to outputs
    Combinatorial,
    // Will be evaluated as synchronous from input to output
    Sequential,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum Output {
    // Will be evaluated as a constant (function without inputs)
    Constant(Signal),
    // Will be evaluated as a function
    Function,
}
