use crate::components::LSBZero;
use log::trace;
use syncrim::{
    gui_vizia::{tooltip::new_component_tooltip, ViziaComponent, V},
    vizia::{
        prelude::*,
        vg::{Color, Paint, Path},
    },
};

#[typetag::serde]
impl ViziaComponent for LSBZero {
    // create view
    fn view<'a>(&self, cx: &'a mut Context) -> Handle<'a, V> {
        V::new(cx, self, move |cx| {
            trace!("---- Create LSBZero View");
            View::build(LSBZeroView {}, cx, |cx| {
                Label::new(cx, "")
                    .left(Percentage(0.0))
                    .top(Percentage(0.0));
            })
        })
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - self.width / 2f32))
        .top(Pixels(self.pos.1 - self.height / 2f32))
        .width(Pixels(self.width))
        .height(Pixels(self.height))
        .tooltip(|cx| new_component_tooltip(cx, self))
    }
}

pub struct LSBZeroView {}

impl View for LSBZeroView {
    fn element(&self) -> Option<&'static str> {
        Some("InstMem")
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
