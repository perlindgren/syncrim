use serde::{Deserialize, Serialize};
use vizia::prelude::*;

#[derive(Lens, Debug, Clone)]
pub struct LensValues {
    pub values: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct ComponentStore {
    pub store: Vec<Box<dyn Component>>,
}

// Note: One can use Rc instead of Box, to get rid of lifetime
pub struct SimState<'a> {
    pub lens_values: LensValues,
    pub component_store: ComponentStore,
    pub eval: Vec<&'a Box<dyn Component>>,
}
// Common functionality for all components
#[typetag::serde()]
pub trait Component {
    // placeholder
    fn to_(&self) {}

    // returns the (id, Ports) of the component
    fn get_id_ports(&self) -> (String, Ports);

    // evaluation function
    fn evaluate(&self, sim_state: &mut SimState) {}
}

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
    // Will be evaluated as synchronous copy from input to output
    Sequential,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Output {
    // Will be evaluated as a constant (function without inputs)
    Constant(u32),
    // Will be evaluated as a function
    Function,
}
