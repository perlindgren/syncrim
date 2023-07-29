use crate::{
    components::Sext,
    gui_vizia::{ViziaComponent, V},
};
use log::*;
use vizia::{
    prelude::*,
    vg::{Paint, Path},
};

#[typetag::serde]
impl ViziaComponent for Sext {
    // create view
    fn view<'a>(&self, cx: &'a mut Context) -> Handle<'a, V> {
        V::new(cx, self, |cx| {
            trace!("---- Create Sext View");
            assert!(self.in_size < self.out_size);

            SextView {}.build(cx, move |cx| {
                Label::new(cx, "SXT")
                    .width(Pixels(80.0))
                    .top(Pixels(20.0))
                    .text_align(TextAlign::Center)
                    .hoverable(false);
            })
        })
        .left(Pixels(self.pos.0 - 40.0))
        .top(Pixels(self.pos.1 - 20.0))
        .width(Pixels(80.0))
        .height(Pixels(40.0))
    }
}

pub struct SextView {}

impl View for SextView {
    fn element(&self) -> Option<&'static str> {
        Some("Sext")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        //trace!("Sext draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbf(1.0, 0.0, 0.0));
        paint.set_line_width(cx.logical_to_physical(1.0));

        let height = bounds.height();
        let _width = bounds.width();
        let top = bounds.top();
        let left = bounds.left();
        let right = bounds.right();
        let bottom = bounds.bottom();

        path.move_to(left + 0.5, top + height / 2.0 + 0.5);
        path.line_to(right + 0.5, top + 0.5);
        path.line_to(right + 0.5, bottom + 0.5);
        path.line_to(left + 0.5, bottom + 0.5);
        path.line_to(left + 0.5, top + height / 2.0 + 0.5);

        canvas.stroke_path(&path, &paint);
    }
}
