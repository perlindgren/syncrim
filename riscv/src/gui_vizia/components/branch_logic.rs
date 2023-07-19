use crate::components::BranchLogic;
use syncrim::{
    common::ViziaComponent,
    gui_vizia::tooltip::new_component_tooltip,
    vizia::{
        prelude::*,
        vg::{Color, Paint, Path},
    },
};

#[typetag::serde]
impl ViziaComponent for BranchLogic {
    // create view
    fn view(&self, cx: &mut Context) {
        println!("---- Create BranchLogic View");
        View::build(BranchLogicView {}, cx, |cx| {
            Label::new(cx, "BranchLogic")
                .left(Percentage(0.0))
                .top(Percentage(0.0));
        })
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - 50.0))
        .top(Pixels(self.pos.1 - 100.0))
        .width(Pixels(50.0))
        .height(Pixels(20.0))
        .tooltip(|cx| new_component_tooltip(cx, self));
    }
}

pub struct BranchLogicView {}

impl View for BranchLogicView {
    fn element(&self) -> Option<&'static str> {
        Some("BranchLogic")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        // println!("InstMem draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(Color::rgbf(0.0, 1.0, 1.0));
        paint.set_line_width(cx.logical_to_physical(1.0));

        path.move_to(bounds.left() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.top() + 0.5);

        canvas.fill_path(&path, &paint);
    }
}
