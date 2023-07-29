use crate::{
    common::{Component, ViziaComponent, V},
    components::Constant,
    gui_vizia::{popup::build_popup, tooltip::new_component_tooltip},
};

use vizia::prelude::*;

use log::*;

#[typetag::serde]
impl ViziaComponent for Constant {
    // create view
    fn view<'a>(&'a self, cx: &'a mut Context) -> Handle<'a, V> {
        V {}.build(cx, move |cx| {
            trace!("---- Create Constant View");

            View::build(ConstantView {}, cx, |cx| {
                Label::new(cx, &format!("{}", self.value)).hoverable(false);
            })
            .left(Pixels(self.pos.0 - 10.0))
            .top(Pixels(self.pos.1 - 10.0))
            //.width(Pixels(20.0)) // TODO, max width?
            .width(Auto)
            .height(Pixels(20.0))
            .background_color(Color::lightblue())
            // TODO: do we want/need tooltip/popup for constants
            .tooltip(|cx| new_component_tooltip(cx, self));
            build_popup(cx, self.get_id_ports());
        })
    }
}
pub struct ConstantView {}

impl View for ConstantView {
    fn element(&self) -> Option<&'static str> {
        Some("Constant")
    }
}
