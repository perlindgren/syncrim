pub mod common;
pub mod component_store;
pub mod fern;
pub mod mips_helper_functions;
pub mod signal;
pub mod simulator;

// Default provided components
#[cfg(feature = "components")]
pub mod components;

// Vizia frontend
#[cfg(feature = "gui-vizia")]
pub mod gui_vizia;

// Egui frontend
#[cfg(feature = "gui-egui")]
pub mod gui_egui;

// Re-export
#[cfg(feature = "gui-vizia")]
pub use vizia;
