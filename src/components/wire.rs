use crate::common::{Component, Output, OutputType, Ports, SimState, Simulator};
use serde::{Deserialize, Serialize};
use vizia::prelude::*;
use vizia::vg::{Paint, Path};

#[derive(Serialize, Deserialize)]
pub struct Wire {
    pub id: String,
    pub pos: (f32, f32),
    pub size: (f32, f32),
}

#[typetag::serde]
impl Component for Wire {
    fn to_(&self) {
        println!("Wire");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                // Wires do not take any inputs
                inputs: vec![],
                out_type: OutputType::Combinatorial,
                // No output value
                outputs: vec![],
            },
        )
    }

    fn evaluate(&self, simulator: &Simulator, sim_state: &mut SimState) {}

    // create view
    fn view(&self, cx: &mut Context, _state: Wrapper<crate::gui::gui_derived_lenses::state>) {
        println!("---- Create Wire View");
        View::build(WireView {}, cx, |cx| {})
            .position_type(PositionType::SelfDirected)
            // .min_size(Pixels(20.0))
            .left(Pixels(self.pos.0))
            .top(Pixels(self.pos.1))
            .width(Pixels(self.size.0))
            .height(Pixels(self.size.1));
    }
}

pub struct WireView {}

impl View for WireView {
    fn element(&self) -> Option<&'static str> {
        Some("Wire")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        println!("Wire draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbaf(0.0, 0.0, 0.1, 0.5));
        paint.set_line_width(cx.logical_to_physical(1.0));

        path.move_to(bounds.left() + 0.5, bounds.top() + 0.5);
        path.line_to(
            bounds.left() + bounds.width() + 0.5,
            bounds.top() + bounds.height() + 0.5,
        );

        canvas.stroke_path(&mut path, &paint);
    }
}
