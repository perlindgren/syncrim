use crate::common::{Component, Input, Output, OutputType, Ports, SimState, Simulator};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use vizia::prelude::*;
use vizia::vg::{Paint, Path};
#[derive(Serialize, Deserialize)]
pub struct Register {
    pub id: String,
    pub pos: (f32, f32),
    pub r_in: Input,
}

#[typetag::serde]
impl Component for Register {
    fn to_(&self) {
        println!("register");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                // Vector of inputs
                inputs: vec![self.r_in.clone()],
                out_type: OutputType::Sequential,
                outputs: vec![Output::Function],
            },
        )
    }

    // propagate input value to output
    fn evaluate(&self, simulator: &Simulator, sim_state: &mut SimState) {
        // get input value
        let value = simulator.get_input_val(sim_state, &self.r_in);
        // set output
        simulator.set_id_index(sim_state, &self.id, 0, value);
        println!("eval: register id {} in {}", self.id, value);
    }

    // create view
    fn view(
        &self,
        cx: &mut Context,
        _simulator: Rc<Simulator>,
        // state: Wrapper<crate::gui::gui_derived_lenses::state>,
    ) {
        println!("---- Create Register View ");
        View::build(RegisterView {}, cx, |_cx| {
            // Label::new(cx, state.map(|s| format!("{:?}", s.lens_values[0])));
        })
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - 5.0))
        .top(Pixels(self.pos.1 - 5.0))
        .width(Pixels(10.0))
        .height(Pixels(10.0));
    }
}

pub struct RegisterView {}

impl View for RegisterView {
    fn element(&self) -> Option<&'static str> {
        Some("Register")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        println!("Register draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbf(1.0, 0.0, 0.0));
        paint.set_line_width(cx.logical_to_physical(1.0));

        path.move_to(bounds.left() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.top() + 0.5);
        path.line_to(bounds.right() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.bottom() + 0.5);
        path.line_to(bounds.left() + 0.5, bounds.top() + 0.5);

        canvas.stroke_path(&path, &paint);
    }
}
