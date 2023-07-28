use crate::{
    common::{Component, ViziaComponent},
    components::Constant,
    gui_vizia::{popup::NewPopup, tooltip::new_component_tooltip},
};

use vizia::prelude::*;

use log::*;

#[typetag::serde]
impl ViziaComponent for Constant {
    // create view
    fn view(&self, cx: &mut Context) {
        trace!("---- Create Constant View");
        View::build(ConstantView {}, cx, |cx| {
            Label::new(cx, &format!("{}", self.value)).hoverable(false);
            NewPopup::new(cx, self.get_id_ports()).position_type(PositionType::SelfDirected);
        })
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - 10.0))
        .top(Pixels(self.pos.1 - 10.0))
        //.width(Pixels(20.0)) // TODO, max width?
        .width(Auto)
        .height(Pixels(20.0))
        .background_color(Color::lightblue())
        // TODO: do we want/need tooltip/popup for constants
        .on_press(|ex| ex.emit(PopupEvent::Switch))
        .tooltip(|cx| new_component_tooltip(cx, self));
    }
}
pub struct ConstantView {}

impl View for ConstantView {
    fn element(&self) -> Option<&'static str> {
        Some("Constant")
    }
}
