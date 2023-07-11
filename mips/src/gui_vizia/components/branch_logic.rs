use crate::components::BranchLogic;
use syncrim::{
    common::ViziaComponent, gui_vizia::tooltip::new_component_tooltip, vizia::prelude::*,
};

#[typetag::serde]
impl ViziaComponent for BranchLogic {
    // create view
    fn view(&self, cx: &mut Context) {
        println!("---- Create BranchLogic View");
        View::build(BranchLogicView {}, cx, |cx| {
            Label::new(cx, "Branch")
                .left(Stretch(1.0))
                .right(Stretch(1.0))
                .top(Stretch(1.0))
                .bottom(Stretch(1.0));
        })
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - self.width / 2.0))
        .top(Pixels(self.pos.1 - self.height / 2.0))
        .width(Pixels(self.width))
        .height(Pixels(self.height))
        .background_color(Color::lightgray())
        .border_color(Color::black())
        .border_width(Pixels(1.0))
        .tooltip(|cx| new_component_tooltip(cx, self));
    }
}

pub struct BranchLogicView {}

impl View for BranchLogicView {
    fn element(&self) -> Option<&'static str> {
        Some("BranchLogic")
    }
}
