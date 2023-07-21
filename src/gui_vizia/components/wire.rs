use crate::{
    common::{Component, ViziaComponent},
    components::Wire,
    gui_vizia::{gui_components::popup::NewPopup, tooltip::new_component_tooltip},
};

use vizia::{
    prelude::*,
    vg::{Paint, Path},
};

use log::*;

#[typetag::serde]
impl ViziaComponent for Wire {
    // create view
    fn view(&self, cx: &mut Context) {
        trace!("---- Create Wire View");
        let surround = 5.0;
        View::build(WireView { surround }, cx, |cx| {
            NewPopup::new(cx, self.get_id_ports()).position_type(PositionType::SelfDirected);
        })
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - surround))
        .top(Pixels(self.pos.1 - surround))
        .width(Pixels(self.delta.0 + 2.0 * surround))
        .height(Pixels(self.delta.1 + 2.0 * surround))
        .on_press(|ex| ex.emit(PopupEvent::Switch))
        .tooltip(|cx| new_component_tooltip(cx, self));
    }
}

pub struct WireView {
    surround: f32,
}

impl View for WireView {
    fn element(&self) -> Option<&'static str> {
        Some("Wire")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        // trace!("Wire draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbaf(0.0, 0.0, 0.1, 0.5));
        paint.set_line_width(cx.logical_to_physical(1.0));

        let surround = self.surround * cx.scale_factor();
        path.move_to(
            bounds.left() + 0.5 + surround,
            bounds.top() + surround + 0.5,
        );
        path.line_to(
            bounds.left() + bounds.width() - surround + 0.5,
            bounds.top() + bounds.height() - surround + 0.5,
        );

        canvas.stroke_path(&path, &paint);
    }
}
