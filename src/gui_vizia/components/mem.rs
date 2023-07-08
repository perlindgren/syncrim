use crate::{
    common::{Component, ViziaComponent},
    components::Mem,
    gui_vizia::{popup::NewPopup, tooltip::new_component_tooltip},
};

use vizia::{
    prelude::*,
    vg::{Paint, Path},
};

#[typetag::serde]
impl ViziaComponent for Mem {
    // create view
    fn view(&self, cx: &mut Context) {
        println!("---- Create Mem View ");

        View::build(MemView {}, cx, |cx| {
            Label::new(cx, "DataMemory")
                .hoverable(false)
                .left(Pixels(10.0))
                .top(Pixels(10.0));
            NewPopup::new(cx, self.get_id_ports()).position_type(PositionType::SelfDirected);
        })
        .position_type(PositionType::SelfDirected)
        .background_color(Color::blueviolet())
        .left(Pixels(self.pos.0 - self.width / 2.0))
        .top(Pixels(self.pos.1 - self.height / 2.0))
        .width(Pixels(self.width))
        .height(Pixels(self.height))
        .on_press(|ex| ex.emit(PopupEvent::Switch))
        .tooltip(|cx| new_component_tooltip(cx, self));
    }
}

struct MemView {}

impl View for MemView {
    fn element(&self) -> Option<&'static str> {
        Some("Memory")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        // println!("Memory draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbf(0.0, 0.0, 0.0));
        paint.set_line_width(cx.logical_to_physical(1.0));

        let left = bounds.left();
        let right = bounds.right();
        let top = bounds.top();
        let bottom = bounds.bottom();
        let _width = bounds.width();
        let _height = bounds.height();

        // draw box
        path.move_to(left + 0.5, top + 0.5);
        path.line_to(right + 0.5, top + 0.5);
        path.line_to(right + 0.5, bottom + 0.5);
        path.line_to(left + 0.5, bottom + 0.5);
        path.line_to(left + 0.5, top + 0.5);

        canvas.stroke_path(&path, &paint);
    }
}
