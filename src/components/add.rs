use crate::common::{Component, Input, Output, OutputType, Ports, SimState, Simulator};
use serde::{Deserialize, Serialize};
use vizia::prelude::*;
use vizia::vg::{Paint, Path};

#[derive(Serialize, Deserialize)]
pub struct Add {
    pub id: String,
    pub pos: (f32, f32),
    pub a_in: Input,
    pub b_in: Input,
}

#[typetag::serde]
impl Component for Add {
    fn to_(&self) {
        println!("add");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                inputs: vec![self.a_in.clone(), self.b_in.clone()],
                out_type: OutputType::Combinatorial,
                outputs: vec![Output::Function],
            },
        )
    }

    // propagate addition to output
    fn evaluate(&self, simulator: &Simulator, sim_state: &mut SimState) {
        // get input values
        let a_in = simulator.get_input_val(sim_state, &self.a_in);
        let b_in = simulator.get_input_val(sim_state, &self.b_in);

        // compute addition (notice will panic on overflow)
        let value = a_in + b_in;

        // set output
        simulator.set_id_index(sim_state, &self.id, 0, value);

        println!(
            "eval: add id {} in {} {} out {}",
            self.id, a_in, b_in, value
        );
    }

    // create view
    fn view(&self, cx: &mut Context, _state: Wrapper<crate::gui::gui_derived_lenses::state>) {
        println!("---- Create Add View");
        View::build(AddView {}, cx, |cx| {
            // Label::new(cx, &format!("{:?}", self.value));
        })
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - 20.0))
        .top(Pixels(self.pos.1 - 40.0))
        .width(Pixels(40.0))
        .height(Pixels(80.0));
    }
}

pub struct AddView {}

impl View for AddView {
    fn element(&self) -> Option<&'static str> {
        Some("Add")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        println!("Add draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbf(1.0, 0.0, 0.0));
        paint.set_line_width(cx.logical_to_physical(1.0));

        let h = bounds.height();
        let w = bounds.width();
        let t = bounds.top();
        let l = bounds.left();
        let r = bounds.right();

        // top left
        path.move_to(l + 0.5, t + 0.5);

        // top right corner
        path.line_to(l + w * 0.5 + 0.5, t + 0.5);
        path.line_to(r + 0.5, t + h * 0.25 + 0.5);

        // bottom right corner
        path.line_to(bounds.right() + 0.5, bounds.bottom() - h * 0.25 + 0.5);
        path.line_to(bounds.left() + w * 0.5 + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.bottom() + 0.5);

        // left outtake
        path.line_to(bounds.left() + 0.5, bounds.bottom() - 0.25 * h + 0.5);
        path.line_to(l + w * 0.25 + 0.5, t + 0.5 * h + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.top() + 0.25 * h + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.top() + 0.5);

        canvas.stroke_path(&mut path, &paint);
    }
}
