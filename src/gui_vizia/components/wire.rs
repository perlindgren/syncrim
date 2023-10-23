use crate::{
    common::Component,
    components::Wire,
    gui_vizia::{popup::build_popup, tooltip::new_component_tooltip, ViziaComponent, V},
};

use log::*;
use std::{cell::RefCell, rc::Rc};
use vizia::{
    prelude::*,
    vg::{Paint, Path},
};

#[typetag::serde]
impl ViziaComponent for Wire {
    // create view
    fn view<'a>(&self, cx: &'a mut Context) -> Handle<'a, V> {
        V {}.build(cx, |cx| {
            trace!("---- Create Wire View");
            let surround = 5.0;

            for (i, pos) in self.pos[1..].iter().enumerate() {
                View::build(
                    WireView {
                        surround,
                        drawn: Rc::new(RefCell::new(false)),
                    },
                    cx,
                    |cx| {
                        build_popup(cx, self.get_id_ports());
                    },
                )
                .position_type(PositionType::SelfDirected)
                .left(Pixels(f32::min(pos.0, self.pos[i].0) - surround))
                .top(Pixels(f32::min(pos.1, self.pos[i].1) - surround))
                .width(Pixels(f32::abs(pos.0 - self.pos[i].0) + 2.0 * surround))
                .height(Pixels(f32::abs(pos.1 - self.pos[i].1) + 2.0 * surround))
                .on_press(|ex| ex.emit(PopupEvent::Switch))
                .tooltip(|cx| new_component_tooltip(cx, self));
            }
        })
        .hoverable(false)
        .width(Pixels(0.0))
        .height(Pixels(0.0))
    }
}

pub struct WireView {
    surround: f32,
    drawn: Rc<RefCell<bool>>,
}

impl View for WireView {
    fn element(&self) -> Option<&'static str> {
        Some("Wire")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        //let drawn = self.drawn.borrow().clone();
        //if !(drawn){
        let bounds = cx.bounds();
        trace!("Wire draw {:?}", bounds);

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
        self.drawn.replace(true);
        canvas.stroke_path(&path, &paint);

        //}
    }
}
