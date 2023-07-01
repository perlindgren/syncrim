use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use vizia::prelude::*;

#[derive(Lens, Data, Clone)]
pub struct Simulator {
    pub id_start_index: IdStartIndex,

    // Components stored in topological evaluation order
    pub ordered_components: Components,
    pub sim_state: Vec<u32>,
    pub history: Vec<Vec<u32>>,
    pub component_ids: Vec<String>,
}

type Components = Vec<Rc<dyn Component>>;

#[derive(Serialize, Deserialize)]
pub struct ComponentStore {
    pub path: String,
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

    /// create view
    fn view(&self, _cx: &mut Context) {}
}

// Note: view uses the concrete type of the derived lens to allow object creation.
// Perhaps we can find a better way (e.g., through type erasure).

#[derive(Debug)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum OutputType {
    // Will be evaluated as a combinatorial function from inputs to outputs
    Combinatorial,
    // Will be evaluated as synchronous from input to output
    Sequential,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Output {
    // Will be evaluated as a constant (function without inputs)
    Constant(u32),
    // Will be evaluated as a function
    Function,
}
