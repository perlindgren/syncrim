use crate::components::Sysclk;
use log::*;
use syncrim::gui_vizia::{ViziaComponent, V};
use syncrim::vizia::{
    prelude::*,
    vg::{Color, Paint, Path},
};

#[typetag::serde]
impl ViziaComponent for Sysclk {
    // create view
    fn view<'a>(&self, cx: &'a mut Context) -> Handle<'a, V> {
        V::new(cx, self, |cx| {
            trace!("---- Create Sysclk View ");
            SysclkView {}.build(cx, |_cx| {})
        })
        .left(Pixels(self.pos.0 - 10.0))
        .top(Pixels(self.pos.1 - 15.0))
        .width(Pixels(20.0))
        .height(Pixels(30.0))
    }
}

pub struct SysclkView {}

impl View for SysclkView {
    fn element(&self) -> Option<&'static str> {
        Some("Sysclk")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        // trace!("Register draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(Color::rgbf(0.0, 0.0, 0.0));
        paint.set_line_width(cx.logical_to_physical(1.0));

        let left = bounds.left();
        let right = bounds.right();
        let top = bounds.top();
        let bottom = bounds.bottom();
        let width = bounds.width();
        let height = bounds.height();

        // draw box
        path.move_to(left + 0.5, top + 0.5);
        path.line_to(right + 0.5, top + 0.5);
        path.line_to(right + 0.5, bottom + 0.5);
        path.line_to(left + 0.5, bottom + 0.5);
        path.line_to(left + 0.5, top + 0.5);

        canvas.stroke_path(&path, &paint);
    }
}
