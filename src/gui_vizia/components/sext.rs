// use std::fmt::Alignment;
use crate::{
    common::{Component, ViziaComponent},
    components::Sext,
    gui_vizia::{popup::NewPopup, tooltip::new_component_tooltip},
};

use vizia::{
    prelude::*,
    vg::{Paint, Path},
};

use log::*;

#[typetag::serde]
impl ViziaComponent for Sext {
    // create viewI
    fn view(&self, cx: &mut Context) {
        trace!("---- Create Sext View");
        assert!(self.in_size < self.out_size);

        View::build(SextView {}, cx, move |cx| {
            Label::new(cx, "SXT")
                .width(Pixels(80.0))
                .top(Pixels(20.0))
                .text_align(TextAlign::Center)
                .hoverable(false);
            NewPopup::new(cx, self.get_id_ports()).position_type(PositionType::SelfDirected);
        })
        .left(Pixels(self.pos.0 - 40.0))
        .top(Pixels(self.pos.1 - 20.0))
        .width(Pixels(80.0))
        .height(Pixels(40.0))
        .on_press(|ex| ex.emit(PopupEvent::Switch))
        .tooltip(|cx| new_component_tooltip(cx, self));
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
