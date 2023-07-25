use petgraph::Graph;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    convert::{From, TryFrom},
    rc::Rc,
};

#[cfg(feature = "gui-vizia")]
use vizia::prelude::*;

// pub type Signal = u32;
// pub type SignedSignal = i32;
pub type Id = String;

pub type SignalUnsigned = u32;
pub type SignalSigned = i32;
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Signal {
    Uninitialized,
    Unknown,
    DontCare,
    Data(SignalUnsigned), // Maybe we should have something even more generic here
}

impl TryFrom<Signal> for SignalUnsigned {
    type Error = String;

    fn try_from(signal: Signal) -> Result<Self, Self::Error> {
        if let Signal::Data(data) = signal {
            Ok(data)
        } else {
            Err(format!("could not convert {:?} into u32", signal))
        }
    }
}

impl From<SignalUnsigned> for Signal {
    fn from(data: u32) -> Signal {
        Signal::Data(data)
    }
}

impl From<bool> for Signal {
    fn from(b: bool) -> Signal {
        Signal::Data(b as SignalUnsigned)
    }
}

#[cfg(not(any(feature = "gui-vizia", feature = "gui-egui")))]
type Components = Vec<Rc<dyn Component>>;

#[cfg(feature = "gui-vizia")]
type Components = Vec<Rc<dyn ViziaComponent>>;

#[cfg(feature = "gui-egui")]
pub type Components = Vec<Rc<dyn EguiComponent>>;

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

    fn set_id_port(&mut self, _target_port_id: Id, _new_input: Input) {}

    /// evaluate component based on current internal state
    fn clock(&self, _simulator: &mut Simulator) {}

    /// update component internal state
    fn un_clock(&self) {}
}

// Specific functionality for Vizia frontend
#[cfg(feature = "gui-vizia")]
#[typetag::serde(tag = "type")]
pub trait ViziaComponent: Component {
    /// create left Vizia view
    fn left_view(&self, _cx: &mut vizia::context::Context) {}

    /// create Vizia view
    fn view(&self, _cx: &mut vizia::context::Context) {}
}

// Specific structs for egui
#[cfg(feature = "gui-egui")]
pub enum SnapPriority {
    Default,
    Wire,
}

#[cfg(feature = "gui-egui")]
#[derive(Debug, Clone, Copy)]
pub enum EditorMode {
    Default,
    Wire,
    Input,
}

#[cfg(feature = "gui-egui")]
pub struct EditorRenderReturn {
    pub delete: bool,
    pub resp: Option<Vec<egui::Response>>,
}

// Specific functionality for EGui frontend
#[cfg(feature = "gui-egui")]
#[typetag::serde(tag = "type")]
pub trait EguiComponent: Component {
    fn render(
        &self,
        _ui: &mut egui::Ui,
        _simulator: Option<&mut Simulator>,
        _offset: egui::Vec2,
        _scale: f32,
        _clip_rect: egui::Rect,
        _editor_mode: EditorMode,
    ) -> Option<Vec<egui::Response>> {
        None
    }

    fn render_editor(
        &mut self,
        _ui: &mut egui::Ui,
        _simulator: Option<&mut Simulator>,
        _offset: egui::Vec2,
        _scale: f32,
        _clip_rect: egui::Rect,
        _id_ports: &Vec<(Id, Ports)>,
        _editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        EditorRenderReturn {
            delete: false,
            resp: None,
        }
    }

    fn size(&self) -> egui::Rect {
        egui::Rect::NOTHING
    }

    /// Get ports location relative to self, (inputs, outputs)
    fn ports_location(&self) -> Vec<(Id, egui::Pos2)> {
        vec![]
    }

    fn snap_priority(&self) -> SnapPriority {
        SnapPriority::Default
    }
    fn set_pos(&mut self, _pos: (f32, f32)) {}

    fn set_id_tmp(&mut self) {}
}

#[cfg(feature = "gui-egui")]
#[derive(Serialize, Deserialize, Clone)]
pub struct EguiExtra {
    pub properties_window: bool,
    pub id_tmp: String,
}

#[cfg(feature = "gui-egui")]
impl Default for EguiExtra {
    fn default() -> Self {
        EguiExtra {
            properties_window: false,
            id_tmp: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ports {
    pub inputs: Vec<InputPort>,
    pub out_type: OutputType,
    pub outputs: Vec<Id>,
}

impl Ports {
    pub fn new(inputs: Vec<&InputPort>, out_type: OutputType, outputs: Vec<&str>) -> Self {
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InputPort {
    pub port_id: Id,
    pub input: Input,
}

impl InputPort {
    pub fn new(id_self: &str, id: &str, field: &str) -> Self {
        InputPort {
            port_id: id_self.into(),
            input: Input::new(id, field),
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

// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
// pub enum Output {
//     // Will be evaluated as a constant (function without inputs)
//     Constant(Signal),
//     // Will be evaluated as a function
//     Function,
// }
