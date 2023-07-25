use num_enum::IntoPrimitive;
use petgraph::Graph;
use serde::{Deserialize, Serialize};
use std::{
    ascii::escape_default,
    collections::HashMap,
    convert::{From, TryFrom},
    fmt,
    rc::Rc,
};

#[cfg(feature = "gui-vizia")]
use vizia::prelude::*;

// pub type Signal = u32;
// pub type SignedSignal = i32;
pub type Id = String;

pub type SignalUnsigned = u32;
pub type SignalSigned = i32;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub struct Signal {
    data: SignalData,
    fmt: SignalFmt,
}

impl Signal {
    /// set data field
    pub fn set_data(&mut self, data: SignalData) {
        self.data = data
    }
    /// set fmt field
    pub fn set_fmt(&mut self, fmt: SignalFmt) {
        self.fmt = fmt
    }
    /// get data field
    pub fn get_data(&self) -> SignalData {
        self.data
    }
    /// get fmt field
    pub fn get_fmt(&self) -> SignalFmt {
        self.fmt
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum SignalData {
    Uninitialized,
    Unknown,
    DontCare,
    Data(SignalUnsigned), // Maybe we should have something even more generic here
}

impl TryFrom<Signal> for SignalUnsigned {
    type Error = String;

    fn try_from(signal: Signal) -> Result<Self, Self::Error> {
        if let SignalData::Data(data) = signal.data {
            Ok(data)
        } else {
            Err(format!(
                "Could not convert {:?} into SignalUnsigned",
                signal
            ))
        }
    }
}

impl TryFrom<SignalData> for SignalUnsigned {
    type Error = String;

    fn try_from(data: SignalData) -> Result<Self, Self::Error> {
        if let SignalData::Data(data) = data {
            Ok(data)
        } else {
            Err(format!("Could not convert {:?} into SignalUnsigned", data))
        }
    }
}

impl From<SignalData> for Signal {
    fn from(data: SignalData) -> Signal {
        Signal {
            data,
            fmt: SignalFmt::Hex(SignalSize::_32),
        }
    }
}

impl From<SignalUnsigned> for Signal {
    fn from(data: u32) -> Signal {
        Signal {
            data: SignalData::Data(data),
            fmt: SignalFmt::Hex(SignalSize::_32),
        }
    }
}

impl From<bool> for Signal {
    fn from(b: bool) -> Signal {
        Signal {
            data: SignalData::Data(b as SignalUnsigned),
            fmt: SignalFmt::Hex(SignalSize::_32),
        }
    }
}

impl From<SignalUnsigned> for SignalData {
    fn from(data: u32) -> SignalData {
        SignalData::Data(data)
    }
}

impl From<bool> for SignalData {
    fn from(b: bool) -> SignalData {
        SignalData::Data(b as SignalUnsigned)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub enum SignalFmt {
    Ascii(SignalSize),
    Unsigned(SignalSize),
    Signed(SignalSize),
    Hex(SignalSize),
    Binary(u8), // just to set a limit to the number of bits
    Bool,       // treats it as true/false
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone, IntoPrimitive)]
#[repr(u8)]
pub enum SignalSize {
    _8 = 1,
    _16 = 2,
    _32 = 4,
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.data {
            SignalData::Data(value) => match self.fmt {
                SignalFmt::Ascii(signal_size) => {
                    let s: u8 = signal_size.into();

                    let bytes = &value.to_be_bytes()[0..s as usize];
                    // let x: Vec<u8> = bytes.iter().map(|b| escape_default(*b)).flatten().collect();
                    // write!(f, "{:?}", self.bytes),
                    unimplemented!()
                }
                SignalFmt::Binary(u) => unimplemented!(),
                SignalFmt::Unsigned(_) => todo!(),
                SignalFmt::Signed(_) => todo!(),
                SignalFmt::Hex(_) => todo!(),
                SignalFmt::Bool => todo!(),
                // _ => write!(f, "{:?}", self.data),
            },
            _ => write!(f, "{:?}", self.data),
        }
    }
}

#[cfg(not(any(feature = "gui-vizia", feature = "gui-egui")))]
type Components = Vec<Rc<dyn Component>>;

#[cfg(feature = "gui-vizia")]
type Components = Vec<Rc<dyn ViziaComponent>>;

#[cfg(feature = "gui-egui")]
type Components = Vec<Rc<dyn EguiComponent>>;

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
#[cfg(feature = "gui-vizia")]
#[typetag::serde(tag = "type")]
pub trait ViziaComponent: Component {
    /// create left Vizia view
    fn left_view(&self, _cx: &mut vizia::context::Context) {}

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
