use crate::common::{offset_helper, Component, OutputType, Ports, SimState, Simulator};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
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

    fn evaluate(&self, _simulator: &Simulator, _sim_state: &mut SimState) {}

    fn render(
        &self,
        _sim_state: &mut crate::common::SimState,
        ui: &mut egui::Ui,
        simulator: Rc<Simulator>,
        offset: egui::Vec2,
        scale: f32,
    ) {
        let oh: fn((f32, f32), f32, egui::Vec2) -> egui::Pos2 = offset_helper;
        let mut offset = offset.clone();
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let s = scale;
        let o = offset;
        ui.painter().add(egui::Shape::line_segment(
            [oh((0f32, 0f32), s, o), oh((self.size.0, self.size.1), s, o)],
            egui::Stroke {
                width: scale,
                color: egui::Color32::BLACK,
            },
        ));
    }
    // create view vizia
    fn view(&self, cx: &mut Context, _simulator: Rc<Simulator>) {
        println!("---- Create Wire View");
        View::build(WireView {}, cx, |_cx| {})
            .position_type(PositionType::SelfDirected)
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
        // println!("Wire draw {:?}", bounds);

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbaf(0.0, 0.0, 0.1, 0.5));
        paint.set_line_width(cx.logical_to_physical(1.0));

        path.move_to(bounds.left() + 0.5, bounds.top() + 0.5);
        path.line_to(
            bounds.left() + bounds.width() + 0.5,
            bounds.top() + bounds.height() + 0.5,
        );

        canvas.stroke_path(&path, &paint);
    }
}
