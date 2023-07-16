use crate::components::RegFile;
use std::{cell::Cell, rc::Rc};
use syncrim::{
    common::ViziaComponent,
    gui_vizia::tooltip::new_component_tooltip,
    vizia::{
        self,
        prelude::*,
        vg::{Paint, Path},
    },
};

#[typetag::serde]
impl ViziaComponent for RegFile {
    // create view
    fn view(&self, cx: &mut Context) {
        println!("---- Create RegFile View");
        View::build(
            RegFileView {
                registers: self.registers.clone(),
            },
            cx,
            |cx| {
                Label::new(cx, "Register File")
                    .left(Pixels(10.0))
                    .top(Pixels(10.0));
                List::new(cx, RegFileView::registers.map(|x| *(*x)), |cx, _, item| {
                    // Label::new(cx, item);
                    Label::new(cx, "-");
                });
                // Binding::new(cx, RegFileView::registers, |cx, wrapper_regs| {
                //     let regs = wrapper_regs.get(cx);
                //     List::new(cx, regs, |cx, _, item| {
                //         //     // Label::new(cx, item);
                //         //     Label::new(cx, "-");
                //     });
                // });
            },
        )
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - self.width / 2.0))
        .top(Pixels(self.pos.1 - self.height / 2.0))
        .background_color(Color::lightgrey())
        .border_width(Pixels(1.0))
        .border_color(Color::black())
        .width(Pixels(self.width))
        .height(Pixels(self.height))
        .tooltip(|cx| new_component_tooltip(cx, self));
    }
}

#[derive(Lens, Clone)]
pub struct RegFileView {
    registers: Rc<Vec<Cell<u32>>>,
}

impl View for RegFileView {
    fn element(&self) -> Option<&'static str> {
        Some("RegView")
    }

    // fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
    //     let bounds = cx.bounds();
    //     // println!("InstMem draw {:?}", bounds);

    //     let mut path = Path::new();
    //     let mut paint = Paint::color(vizia::vg::Color::rgbf(0.0, 1.0, 1.0));
    //     paint.set_line_width(cx.logical_to_physical(1.0));

    //     path.move_to(bounds.left() + 0.5, bounds.top() + 0.5);
    //     path.line_to(bounds.right() + 0.5, bounds.top() + 0.5);
    //     path.line_to(bounds.right() + 0.5, bounds.bottom() + 0.5);
    //     path.line_to(bounds.left() + 0.5, bounds.bottom() + 0.5);
    //     path.line_to(bounds.left() + 0.5, bounds.top() + 0.5);

    //     canvas.fill_path(&path, &paint);
    // }
}
