use crate::components::InstrSplit;
use syncrim::{
    common::ViziaComponent, gui_vizia::tooltip::new_component_tooltip, vizia::prelude::*,
};

#[typetag::serde]
impl ViziaComponent for InstrSplit {
    // create view
    fn view(&self, cx: &mut Context) {
        println!("---- Create InsrMem View");
        View::build(InstrSplitView {}, cx, |_cx| {})
            .position_type(PositionType::SelfDirected)
            .left(Pixels(self.pos.0 - self.width / 2.0))
            .top(Pixels(self.pos.1 - self.height / 2.0))
            .width(Pixels(self.width))
            .height(Pixels(self.height))
            .background_color(Color::lightgrey())
            .border_color(Color::black())
            .border_width(Pixels(1.0))
            .tooltip(|cx| new_component_tooltip(cx, self));
    }
}

struct InstrSplitView {}

impl View for InstrSplitView {
    fn element(&self) -> Option<&'static str> {
        Some("InstrSplit")
    }
}
