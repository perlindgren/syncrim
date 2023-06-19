use crate::common::{Component, Input, Output, OutputType, Ports, SimState, Simulator};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use vizia::prelude::*;
use vizia::vg::{Paint, Path};

#[derive(Serialize, Deserialize)]
pub struct Mux {
    pub id: String,
    pub pos: (f32, f32),
    pub select: Input,
    pub m_in: Vec<Input>,
}

#[typetag::serde]
impl Component for Mux {
    fn to_(&self) {
        println!("mux");
    }

    fn get_id_ports(&self) -> (String, Ports) {
        let mut inputs = vec![self.select.clone()];
        let mut m = self.m_in.clone();
        inputs.append(&mut m);

        (
            self.id.clone(),
            Ports {
                inputs,
                out_type: OutputType::Combinatorial,
                outputs: vec![Output::Function],
            },
        )
    }

    // propagate selected input value to output
    fn evaluate(&self, simulator: &Simulator, sim_state: &mut SimState) {
        // get input value
        let select = simulator.get_input_val(sim_state, &self.select) as usize;
        let value = simulator.get_input_val(sim_state, &self.m_in[select]);

        // set output
        simulator.set_id_index(sim_state, &self.id, 0, value);
    }

    // create view vizia
    fn view(&self, cx: &mut Context, simulator: Rc<Simulator>) {
        println!("---- Create Add View");
        View::build(
            MuxView {
                simulator,
                select: self.select.clone(),
            },
            cx,
            |_cx| {},
        )
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - 20.0))
        .top(Pixels(self.pos.1 - 10.0 * self.m_in.len() as f32 - 10.0))
        .width(Pixels(40.0))
        .height(Pixels(20.0 * self.m_in.len() as f32 + 20.0));
    }
}

pub struct MuxView {
    simulator: Rc<Simulator>,
    select: Input,
}

impl View for MuxView {
    fn element(&self) -> Option<&'static str> {
        Some("Mux")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        let scale = cx.scale_factor();
        // println!("Mux draw {:?}", bounds);

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
        let select = self
            .simulator
            .get_input_val(&crate::gui::Gui::state.get(cx), &self.select);

        println!("----- select = {}", select);
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
