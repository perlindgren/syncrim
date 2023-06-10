use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use vizia::prelude::*;

#[derive(Lens, Debug, Clone)]
pub struct SimState {
    pub lens_values: Vec<u32>,
}

// pub struct MyLens<A> {
//     a: A,
// }

// impl<A> MyLens<A> {
//     pub fn new(a: A) -> Self
//     where
//         A: Lens,
//     {
//         Self { a }
//     }
// }

// impl<A> Lens for MyLens<A>
// where
//     A: Lens,
// {
//     type Source = A::Source;
//     type Target = A::Target;

//     fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &Self::Source, map: F) -> O {
//         self.a.view(source, map)
//     }
// }

// impl<T: Clone> Clone for MyLens<T> {
//     fn clone(&self) -> Self {
//         Self { a: self.a.clone() }
//     }
// }

// type MyLensType = Lens<Source = (), Target<u32>>;

#[derive(Lens)]
pub struct Simulator {
    pub id_start_index: IdStartIndex,

    // Components stored in topological evaluation order
    pub ordered_components: Components,
}

type Components = Vec<Rc<dyn Component>>;

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

    // returns the (id, Ports) of the component
    fn get_id_ports(&self) -> (String, Ports);

    // evaluation function
    fn evaluate(&self, _simulator: &Simulator, _sim_state: &mut SimState) {}

    // create view
    //fn view(&self, cx: &mut Context, fake_lens: &dyn Fn(&Context) -> Vec<u32>) {}
    fn view(&self, cx: &mut Context, _state: Wrapper<crate::gui::gui_derived_lenses::state>) {}
    // fn view(&self, cx: &mut Context) {}
    // fn view(&self, cx: &mut Context, p: Rc<SimState>) {}
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
