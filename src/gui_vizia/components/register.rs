use crate::{
    common::{Component, ViziaComponent},
    components::Register,
    gui_vizia::{popup::NewPopup, tooltip::new_component_tooltip},
};

use vizia::{
    prelude::*,
    vg::{Paint, Path},
};

use log::*;

#[typetag::serde]
impl ViziaComponent for Register {
    // create view
    fn view(&self, cx: &mut Context) {
        trace!("---- Create Register View ");

        View::build(RegisterView {}, cx, |cx| {
            NewPopup::new(cx, self.get_id_ports()).position_type(PositionType::SelfDirected);
        })
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - 10.0))
        .top(Pixels(self.pos.1 - 15.0))
        .width(Pixels(20.0))
        .height(Pixels(30.0))
        .on_press(|ex| ex.emit(PopupEvent::Switch))
        .tooltip(|cx| new_component_tooltip(cx, self));
    }
}

pub struct RegisterView {}

impl View for RegisterView {
    fn element(&self) -> Option<&'static str> {
        Some("Register")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        // trace!("Register draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbf(0.0, 0.0, 0.0));
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

        // draw cut out
        path.line_to(left + width * 0.5 + 0.5, top + height * 0.25 + 0.5);
        path.line_to(right + 0.5, top + 0.5);

        canvas.stroke_path(&path, &paint);
    }
}
