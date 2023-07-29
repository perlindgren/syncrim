
use crate::common::Component;
use log::*;
use vizia::prelude::*;

#[cfg(feature = "components")]
pub mod components;

mod grid;
mod gui;
pub mod hover;
mod keymap;
mod menu;
pub mod popup;
pub mod tooltip;
mod transport;

pub use gui::*;

pub struct V;
impl View for V {}

impl V {
    pub fn new<'a, H>(
        cx: &'a mut Context,
        component: &dyn Component,
        content: impl FnOnce(&mut Context) -> Handle<'_, H>,
    ) -> Handle<'a, V> {
        Self {}
            .build(cx, move |cx| {
                trace!("V build");
                content(cx).hoverable(false);
                crate::gui_vizia::popup::build_popup(cx, component.get_id_ports()).hoverable(true);
            })
            .size(Auto)
    }
}

// Specific functionality for Vizia frontend
#[typetag::serde(tag = "type")]
pub trait ViziaComponent: Component {
    /// create left Vizia view
    fn left_view(&self, _cx: &mut vizia::context::Context) {}

    /// create Vizia view
    fn view<'a>(&self, cx: &'a mut Context) -> Handle<'a, V> {
        V {}.build(cx, move |_| {})
    }
}
