use crate::{
    common::{Component, ViziaComponent},
    components::ProbeStim,
    gui_vizia::{popup::NewPopup, tooltip::new_component_tooltip},
};

use vizia::{
    prelude::*,
    vg::{Paint, Path},
};

use log::*;

#[typetag::serde]
impl ViziaComponent for ProbeStim {
    // create view
    fn view(&self, cx: &mut Context) {
        trace!("---- Create ProbeStim View");
        View::build(ProbeStimView {}, cx, |cx| {
            // Label::new(cx, &format!("{:?}", self.value)).hoverable(false);
            NewPopup::new(cx, self.get_id_ports()).position_type(PositionType::SelfDirected);
        })
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - 10.0))
        .top(Pixels(self.pos.1 - 10.0))
        .width(Pixels(20.0))
        .height(Pixels(20.0))
        // TODO: do we want/need tooltip/popup for constants
        .on_press(|ex| ex.emit(PopupEvent::Switch))
        .tooltip(|cx| new_component_tooltip(cx, self));
    }
}
pub struct ProbeStimView {}

impl View for ProbeStimView {
    fn element(&self) -> Option<&'static str> {
        Some("ProbeStim")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        // trace!("Constant draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbf(0.0, 1.0, 0.0));
        paint.set_line_width(cx.logical_to_physical(1.0));

        path.move_to(bounds.left() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.top() + 0.5);

        canvas.fill_path(&path, &paint);
    }
}
