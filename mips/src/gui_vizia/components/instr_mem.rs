use crate::components::InstrMem;
use syncrim::{
    gui_vizia::{ViziaComponent, V},
    vizia::{
        prelude::*,
        vg::{Color, Paint, Path},
    },
};

use log::*;

#[typetag::serde]
impl ViziaComponent for InstrMem {
    // create view
    fn left_view(&self, cx: &mut Context) {
        trace!("---- Create Left Instr View");

        View::build(InstMemLeft { display: false }, cx, |cx| {
            Label::new(cx, "Inst Mem Left");
        });
    }

    // create view
    fn view<'a>(&self, cx: &'a mut Context) -> Handle<'a, V> {
        trace!("---- Create InsrMem View");
        V::new(cx, self, |cx| {
            InstMem {}.build(cx, |cx| {
                Label::new(cx, "Inst Mem")
                    .left(Percentage(20.0))
                    .top(Percentage(45.0))
                    .hoverable(false);
            })
        })
        .left(Pixels(self.pos.0 - 50.0))
        .top(Pixels(self.pos.1 - 100.0))
        .width(Pixels(100.0))
        .height(Pixels(200.0))
    }
}

#[derive(Lens, Clone)]
pub struct InstMemLeft {
    display: bool,
}

impl View for InstMemLeft {
    fn element(&self) -> Option<&'static str> {
        Some("InstMem")
    }

    // TODO, what to show here
}

pub struct InstMem {}

impl View for InstMem {
    fn element(&self) -> Option<&'static str> {
        Some("InstMem")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        // trace!("InstMem draw {:?}", bounds);

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
