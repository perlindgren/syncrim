use petgraph::Graph;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, rc::Rc};

#[cfg(feature = "gui-vizia")]
use vizia::prelude::*;

pub use crate::signal::*;

use log::*;

// #[cfg(not(any(feature = "gui-vizia", feature = "gui-egui")))]
// type Components = Vec<Rc<dyn Component>>;

// #[cfg(feature = "gui-vizia")]
// type Components = Vec<Rc<dyn ViziaComponent>>;

// #[cfg(feature = "gui-egui")]
// type Components = Vec<Rc<dyn EguiComponent>>;

#[typetag::serde(tag = "type")]
pub trait GuiComponent: Component + ViziaComponent {}

pub type Components = Vec<Rc<dyn GuiComponent>>;

#[cfg_attr(feature = "gui-vizia", derive(Lens))]
#[derive(Clone)]
pub struct Simulator {
    pub cycle: usize,
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

    /// evaluate component based on current internal state
    fn clock(&self, _simulator: &mut Simulator) {}

    /// update component internal state
    fn un_clock(&self) {}
}

// Specific functionality for Vizia frontend
// #[cfg(feature = "gui-vizia")]
#[typetag::serde(tag = "type")]
pub trait ViziaComponent: Component {
    /// create left Vizia view
    fn left_view(&self, _cx: &mut vizia::context::Context) {}

    /// create Vizia view
    fn view<'a>(&self, cx: &'a mut Context) -> Handle<'a, V> {
        V {}.build(cx, move |_| {})
    }
}

pub struct V;
impl View for V {}

// impl V {
//     pub fn new<H>(
//         cx: &mut Context,
//         id_ports: (Id, Ports),
//         content: impl FnOnce(&mut Context) -> Handle<'_, H>,
//     ) -> Handle<V> {
//         Self {}
//             .build(cx, |cx| {
//                 trace!("V build");
//                 content(cx).hoverable(false);
//                 crate::gui_vizia::popup::build_popup(cx, id_ports).hoverable(true);
//             })
//             .size(Auto)
//     }
// }

impl V {
    pub fn new<'a, H>(
        cx: &'a mut Context,
        component: &dyn Component,
        content: impl FnOnce(&mut Context) -> Handle<'_, H>,
    ) -> Handle<'a, V> {
        // let id_idport = component.get_id_ports();
        Self {}
            .build(cx, move |cx| {
                trace!("V build");
                content(cx).hoverable(false);
                crate::gui_vizia::popup::build_popup(cx, component.get_id_ports()).hoverable(true);
            })
            .size(Auto)
    }
}

// Specific functionality for EGui frontend
#[cfg(feature = "gui-egui")]
#[typetag::serde(tag = "type")]
pub trait EguiComponent {
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
    pub field: Id,
}

impl Input {
    pub fn new(id: &str, field: &str) -> Self {
        Input {
            id: id.into(),
            field: field.into(),
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
