use crate::components::CLIC;
use log::trace;
use syncrim::{
    gui_vizia::{tooltip::new_component_tooltip, ViziaComponent, V},
    vizia::{
        prelude::*,
        vg::{Color, Paint, Path},
    },
};

#[typetag::serde]
impl ViziaComponent for CLIC {
    // create view
    fn view<'a>(&self, cx: &'a mut Context) -> Handle<'a, V> {
        V::new(cx, self, move |cx| {
            trace!("---- Create CLIC View");

            View::build(CLICView {}, cx, |cx| {
                Label::new(cx, "CLIC")
                    .left(Percentage(20.0))
                    .top(Percentage(45.0));
            })
        })
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - 50.0))
        .top(Pixels(self.pos.1 - 100.0))
        .width(Pixels(100.0))
        .height(Pixels(200.0))
        .tooltip(|cx| new_component_tooltip(cx, self))
    }
}

pub struct CLICView {}

impl View for CLICView {
    fn element(&self) -> Option<&'static str> {
        Some("CLIC")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        // println!("InstMem draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(Color::rgbf(1.0, 0.9, 0.9));
        paint.set_line_width(cx.logical_to_physical(1.0));

        path.move_to(bounds.left() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.top() + 0.5);

        canvas.fill_path(&path, &paint);
    }
}
