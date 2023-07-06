pub mod common;
pub mod component_store;
pub mod components;
pub mod simulator;

// Vizia frontend
#[cfg(feature = "gui-vizia")]
pub mod gui_vizia;

// Egui frontend
#[cfg(feature = "gui-egui")]
pub mod gui_egui;

// Re-export
#[cfg(feature = "gui-vizia")]
pub use vizia;
