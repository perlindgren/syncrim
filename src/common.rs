use petgraph::Graph;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::{collections::HashMap, rc::Rc};

#[cfg(feature = "gui-egui")]
use crate::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions, SnapPriority};

#[cfg(feature = "gui-vizia")]
use vizia::prelude::*;

#[cfg(feature = "gui-vizia")]
use crate::gui_vizia::ViziaComponent;

pub use crate::signal::*;

#[cfg(not(any(feature = "gui-vizia", feature = "gui-egui")))]
type Components = Vec<Rc<dyn Component>>;

#[cfg(feature = "gui-vizia")]
type Components = Vec<Rc<dyn ViziaComponent>>;

#[cfg(feature = "gui-egui")]
pub type Components = Vec<Rc<dyn EguiComponent>>;

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
    // Running state, (do we need it accessible from other crates?)
    pub(crate) running: bool,
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

    // #default
    // fn get_pos(&self) -> (Position);

    fn set_id_port(&mut self, _target_port_id: Id, _new_input: Input) {
        todo!("Set set_id_port for this Component");
    }
    #[cfg(feature = "gui-egui")]
    fn dummy(&self, _id: &str, _pos: (f32, f32)) -> Box<Rc<dyn EguiComponent>> {
        todo!("implement dummy component factory for this component")
    }
    /// evaluate component based on current internal state
    fn clock(&self, _simulator: &mut Simulator) -> Result<(), Condition> {
        Ok(())
    }
    /// update component internal state
    fn un_clock(&self) {}
    /// reset component internal state to initial value
    fn reset(&self) {}
    /// any
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Condition {
    Warning(String),
    Error(String),
    Assert(String),
    Halt(String),
}

#[cfg(feature = "gui-egui")]
use crate::gui_egui::gui::EguiExtra;

// Specific functionality for EGui frontend
#[cfg(feature = "gui-egui")]
#[typetag::serde(tag = "type")]
pub trait EguiComponent: Component {
    #[allow(clippy::too_many_arguments)]
    fn render(
        &self,
        _ui: &mut egui::Ui,
        _context: &mut EguiExtra,
        _simulator: Option<&mut Simulator>,
        _offset: egui::Vec2,
        _scale: f32,
        _clip_rect: egui::Rect,
        _editor_mode: EditorMode,
    ) -> Option<Vec<egui::Response>> {
        None
    }

    #[allow(clippy::too_many_arguments)]
    fn render_editor(
        &mut self,
        _ui: &mut egui::Ui,
        _context: &mut EguiExtra,
        _simulator: Option<&mut Simulator>,
        _offset: egui::Vec2,
        _scale: f32,
        _clip_rect: egui::Rect,
        _id_ports: &[(Id, Ports)],
        _grid: &GridOptions,
        _editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        EditorRenderReturn {
            delete: false,
            resp: None,
        }
    }

    fn get_port_location(&self, _id: Input) -> Option<(f32, f32)> {
        None
    }

    fn top_padding(&self) -> f32 {
        todo!("Create top_padding for this EguiComponent");
    }

    /// Get ports location relative to self, (inputs, outputs)
    fn ports_location(&self) -> Vec<(Id, egui::Pos2)> {
        todo!("Create ports_location for this EguiComponent");
    }

    fn snap_priority(&self) -> SnapPriority {
        SnapPriority::Default
    }

    fn set_pos(&mut self, _pos: (f32, f32)) {
        todo!("Create set_pos for this EguiComponent");
    }

    fn get_pos(&self) -> (f32, f32) {
        todo!("Create get_pos for this EguiComponent");
    }

    fn set_id_tmp(&self, context: &mut EguiExtra) {
        context.id_tmp.clone_from(&self.get_id_ports().0);
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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
