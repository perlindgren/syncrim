use crate::components::RegFile;
use syncrim::{
    common::ViziaComponent,
    gui_vizia::{tooltip::new_component_tooltip, GuiData},
    vizia::{
        prelude::*,
        vg::{Color, Paint, Path},
    },
};

#[typetag::serde]
impl ViziaComponent for RegFile {
    // create view
    fn view(&self, cx: &mut Context) {
        println!("---- Create RegFile View");
        View::build(RegFileView {}, cx, |cx| {
            let input = self.id.clone();

            Binding::new(cx,GuiData::clock, move |cx, _| {
                Label::new(cx, &{
                    let simulator = GuiData::simulator.get(cx);
                    simulator.c_by_id_str(&input)
                })
                .hoverable(false);
            });
        })
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - 50.0))
        .top(Pixels(self.pos.1 - 100.0))
        .width(Pixels(250.0))
        .height(Pixels(750.0))
        .tooltip(|cx| new_component_tooltip(cx, self));
    }
}

pub struct RegFileView {}

impl View for RegFileView {
    fn element(&self) -> Option<&'static str> {
        Some("RegFile")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();

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
