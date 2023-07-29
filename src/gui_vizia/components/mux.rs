use crate::{
    common::{Input, SignalUnsigned, ViziaComponent, V},
    components::Mux,
    gui_vizia::GuiData,
};

use vizia::{
    prelude::*,
    vg::{Paint, Path},
};

use log::*;

#[typetag::serde]
impl ViziaComponent for Mux {
    // create view
    fn view<'a>(&self, cx: &'a mut Context) -> Handle<'a, V> {
        V::new(cx, self, |cx| {
            trace!("---- Create Mux View");
            MuxView {
                select: self.select.clone(),
                select_max: self.m_in.len() as u8,
            }
            .build(cx, |_cx| {})
        })
        .left(Pixels(self.pos.0 - 20.0))
        .top(Pixels(self.pos.1 - 10.0 * self.m_in.len() as f32 - 10.0))
        .width(Pixels(40.0))
        .height(Pixels(20.0 * self.m_in.len() as f32 + 20.0))
    }
}

pub struct MuxView {
    select: Input,
    select_max: u8,
}

impl View for MuxView {
    fn element(&self) -> Option<&'static str> {
        Some("Mux")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        let scale = cx.scale_factor();
        // trace!("Mux draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbf(0.0, 0.0, 0.0));
        paint.set_line_width(cx.logical_to_physical(1.0));

        let height = bounds.height();
        let width = bounds.width();
        let top = bounds.top();
        let left = bounds.left();
        let right = bounds.right();
        let bottom = bounds.bottom();

        // top left
        path.move_to(left + 0.5, top + 0.5);

        // top and right corner
        path.line_to(left + width * 0.5 + 0.5, top + 0.5);
        path.line_to(right + 0.5, top + height * 0.25 + 0.5);

        // bottom and right corner
        path.line_to(bounds.right() + 0.5, bottom - height * 0.25 + 0.5);
        path.line_to(left + width * 0.5 + 0.5, bottom + 0.5);
        path.line_to(left + 0.5, bottom + 0.5);

        // left side
        path.line_to(left + 0.5, top + 0.5);

        canvas.stroke_path(&path, &paint);

        // selector
        let simulator = GuiData::simulator.get(cx);

        let select: Result<SignalUnsigned, String> =
            simulator.get_input_value(&self.select).try_into();

        trace!("----- select = {:?}", select);
        if let Ok(select) = select {
            if select < self.select_max as u32 {
                paint = Paint::color(vizia::vg::Color::rgbf(1.0, 0.0, 0.0));
                let mut path = Path::new();

                path.move_to(
                    left + 0.5,
                    top + 0.5 + (20.0 + select as f32 * 20.0) * scale,
                );
                path.line_to(right + 0.5, top + height * 0.5 + 0.5);
                canvas.stroke_path(&path, &paint);
            }
        }
    }
}
