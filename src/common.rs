use petgraph::Graph;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[cfg(feature = "gui-vizia")]
use vizia::prelude::*;

pub type Signal = u32;
pub type SignedSignal = i32;

#[cfg(not(any(feature = "gui-vizia", feature = "gui-egui")))]
type Components = Vec<Rc<dyn Component>>;

#[cfg(feature = "gui-vizia")]
type Components = Vec<Rc<dyn ViziaComponent>>;

#[cfg(all(not(test), feature = "egui"))]
type Components = Vec<Rc<dyn EguiComponent>>;

#[cfg(feature = "gui-egui")]
#[derive(Clone)]
pub struct Simulator {
    pub id_start_index: IdStartIndex,

    // Components stored in topological evaluation order
    pub ordered_components: Components,
    pub sim_state: Vec<Signal>,
    pub history: Vec<Vec<Signal>>,
    pub component_ids: Vec<String>,
    pub graph: Graph<String, ()>,
}

#[cfg(feature = "gui-vizia")]
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

#[derive(Serialize, Deserialize)]
pub struct ComponentStore {
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

// Specific functionality for Vizia frontend
#[cfg(feature = "gui-vizia")]
#[typetag::serde(tag = "type")]
pub trait ViziaComponent: Component {
    /// create Vizia view
    fn view(&self, _cx: &mut vizia::context::Context) {}
}

// Specific functionality for EGui frontend
#[cfg(feature = "gui-egui")]
#[typetag::serde(tag = "type")]
pub trait EguiComponent: Component {
    fn render(&self, _ui: &mut egui::Ui, _simulator: Simulator, _start: egui::Vec2, _scale: f32) {}
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

impl Input {
    pub fn new(id: &str, index: usize) -> Self {
        Input {
            id: id.to_string(),
            index,
        }
    }
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
