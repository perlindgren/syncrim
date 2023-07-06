use crate::{
    common::{Component, ViziaComponent},
    components::Add,
    gui_vizia::{popup::NewPopup, tooltip::new_component_tooltip},
};

use vizia::{
    prelude::*,
    vg::{Paint, Path},
};

#[typetag::serde]
impl ViziaComponent for Add {
    // create view
    fn view(&self, cx: &mut Context) {
        println!("---- Create Add View");

        View::build(AddView {}, cx, move |cx| {
            Label::new(cx, "+")
                .left(Percentage(50.0))
                .top(Pixels(40.0 - 10.0))
                .hoverable(false);
            NewPopup::new(cx, self.get_id_ports()).position_type(PositionType::SelfDirected);
        })
        .left(Pixels(self.pos.0 - 20.0))
        .top(Pixels(self.pos.1 - 40.0))
        .width(Pixels(40.0))
        .height(Pixels(80.0))
        .on_press(|ex| ex.emit(PopupEvent::Switch))
        .tooltip(|cx| new_component_tooltip(cx, self));
    }
}

pub struct AddView {}

impl View for AddView {
    fn element(&self) -> Option<&'static str> {
        Some("Add")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        //println!("Add draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbf(1.0, 0.0, 0.0));
        paint.set_line_width(cx.logical_to_physical(1.0));

        let height = bounds.height();
        let width = bounds.width();
        let top = bounds.top();
        let left = bounds.left();
        let right = bounds.right();
        let bottom = bounds.bottom();

        // top left
        path.move_to(left + 0.5, top + 0.5);

        // top right corner
        path.line_to(left + width * 0.5 + 0.5, top + 0.5);
        path.line_to(right + 0.5, top + height * 0.25 + 0.5);

        // bottom right corner
        path.line_to(right + 0.5, bottom - height * 0.25 + 0.5);
        path.line_to(left + width * 0.5 + 0.5, bottom + 0.5);
        path.line_to(left + 0.5, bottom + 0.5);

        // left outtake
        path.line_to(left + 0.5, bottom - 0.25 * height + 0.5);
        path.line_to(left + width * 0.25 + 0.5, top + 0.5 * height + 0.5);
        path.line_to(left + 0.5, top + 0.25 * height + 0.5);
        path.line_to(left + 0.5, top + 0.5);

        canvas.stroke_path(&path, &paint);
    }
}
