use petgraph::Graph;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[cfg(feature = "gui-vizia")]
use vizia::prelude::*;

pub type Signal = u32;
pub type SignedSignal = i32;
pub type Id = String;

#[cfg(not(any(feature = "gui-vizia", feature = "gui-egui")))]
type Components = Vec<Rc<dyn Component>>;

#[cfg(feature = "gui-vizia")]
type Components = Vec<Rc<dyn ViziaComponent>>;

#[cfg(feature = "gui-egui")]
type Components = Vec<Rc<dyn EguiComponent>>;

#[cfg_attr(feature = "gui-vizia", derive(Lens))]
#[derive(Clone)]
pub struct Simulator {
    pub id_start_index: IdStartIndex,

    // Components stored in topological evaluation order
    pub ordered_components: Components,
    pub sim_state: Vec<Signal>,
    pub id_nr_outputs: IdNrOutputs,
    pub id_field_index: IdFieldIndex,
    pub history: Vec<Vec<Signal>>,
    pub component_ids: Vec<Id>,
    pub graph: Graph<Id, ()>,
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
pub type IdStartIndex = HashMap<Id, usize>;

pub type IdNrOutputs = HashMap<Id, usize>;

pub type IdFieldIndex = HashMap<(Id, Id), usize>;

// Common functionality for all components
#[typetag::serde(tag = "type")]
pub trait Component {
    // placeholder
    fn to_(&self) {}

    /// returns the (id, Ports) of the component
    fn get_id_ports(&self) -> (Id, Ports);

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
    fn render(
        &self,
        _ui: &mut egui::Ui,
        _simulator: Simulator,
        _start: egui::Vec2,
        _scale: f32,
        _clip_rect: egui::Rect,
    ) {
    }
}

#[derive(Debug, Clone)]
pub struct Ports {
    pub inputs: Vec<Input>,
    pub out_type: OutputType,
    pub outputs: Vec<Id>,
}

impl Ports {
    pub fn new(inputs: Vec<&Input>, out_type: OutputType, outputs: Vec<&str>) -> Self {
        Ports {
            inputs: inputs.into_iter().cloned().collect(),
            out_type,
            outputs: outputs.into_iter().map(|s| s.into()).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Input {
    pub id: Id,
    pub index: usize,
}

impl Input {
    pub fn new(id: &str, index: usize) -> Self {
        Input {
            id: id.into(),
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
