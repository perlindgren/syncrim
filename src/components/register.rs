use crate::common::{
    offset_helper, Component, Input, Output, OutputType, Ports, SimState, Simulator,
};
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

    // egui
    fn render(
        &self,
        _sim_state: &mut crate::common::SimState,
        ui: &mut egui::Ui,
        simulator: Rc<Simulator>,
        offset: egui::Vec2,
        scale: f32,
    ) {
        // 21x41
        // middle: 11x 21y (0 0)
        let oh: fn((f32, f32), f32, egui::Vec2) -> egui::Pos2 = offset_helper;
        let mut offset = offset.clone();
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let s = scale;
        let o = offset;

        // The shape
        ui.painter().add(egui::Shape::line(
            vec![
                oh((-10f32, -20f32), s, o),
                oh((10f32, -20f32), s, o),
                oh((0f32, -15f32), s, o),
                oh((-10f32, -20f32), s, o),
                oh((-10f32, 20f32), s, o),
                oh((10f32, 20f32), s, o),
                oh((10f32, -20f32), s, o),
            ],
            egui::Stroke {
                width: scale,
                color: egui::Color32::BLACK,
            },
        ));
    }

    // create view vizia
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
        .left(Pixels(self.pos.0 - 10.0))
        .top(Pixels(self.pos.1 - 15.0))
        .width(Pixels(20.0))
        .height(Pixels(30.0));
    }
}

pub struct RegisterView {}

impl View for RegisterView {
    fn element(&self) -> Option<&'static str> {
        Some("Register")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        // println!("Register draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbf(0.0, 0.0, 0.0));
        paint.set_line_width(cx.logical_to_physical(1.0));

        let left = bounds.left();
        let right = bounds.right();
        let top = bounds.top();
        let bottom = bounds.bottom();
        let width = bounds.width();
        let height = bounds.height();

        // draw box
        path.move_to(left + 0.5, top + 0.5);
        path.line_to(right + 0.5, top + 0.5);
        path.line_to(right + 0.5, bottom + 0.5);
        path.line_to(left + 0.5, bottom + 0.5);
        path.line_to(left + 0.5, top + 0.5);

        // draw cut out
        path.line_to(left + width * 0.5 + 0.5, top + height * 0.25 + 0.5);
        path.line_to(right + 0.5, top + 0.5);

        canvas.stroke_path(&path, &paint);
    }
}
