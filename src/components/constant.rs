use crate::common::{Component, Output, OutputType, Ports, SimState, Simulator};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use vizia::prelude::*;
use vizia::vg::{Paint, Path};

#[derive(Serialize, Deserialize)]
pub struct Constant {
    pub id: String,
    pub pos: (f32, f32),
    pub value: u32, // perhaps vector here ... not sure
}

#[typetag::serde]
impl Component for Constant {
    fn to_(&self) {
        println!("constant {:?}", self.value);
    }

    fn get_id_ports(&self) -> (String, Ports) {
        (
            self.id.clone(),
            Ports {
                // Constants do not take any inputs
                inputs: vec![],
                out_type: OutputType::Combinatorial,
                // Single output value
                outputs: vec![Output::Constant(self.value)],
            },
        )
    }

    fn evaluate(&self, simulator: &Simulator, sim_state: &mut SimState) {
        simulator.set_id_index(sim_state, &self.id, 0, self.value);
    }

    // egui
    fn render(&self, ui: &mut egui::Ui, simulator: Rc<Simulator>, offset: egui::Vec2, scale: f32) {
        let mut offset = offset.clone();
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        //println!("---- Create Add View");
        // The shape
        // 40x30
        /*
        ui.painter().add(egui::Shape::text(
            egui::Painter::fonts(),
            egui::Pos2 {
                x: 0.0f32,
                y: 0.0f32,
            } + offset,
            egui::Align2::LEFT_TOP,
            self.value.to_string(),
            egui::FontId::proportional(scale),
            egui::Color32::GREEN,
        ));
        */
        //ui.add(egui::Label::new(self.value.to_string()));
        //ui.label(self.value.to_string());
        //ui.painter().add(egui::Shape::closed_line());
    }

    // create view vizia
    fn view(
        &self,
        cx: &mut Context,
        _simulator: Rc<Simulator>,
        //_state: Wrapper<crate::gui::gui_derived_lenses::state>,
    ) {
        println!("---- Create Constant View");
        View::build(ConstantView {}, cx, |cx| {
            Label::new(cx, &format!("{:?}", self.value));
        })
        .position_type(PositionType::SelfDirected)
        .left(Pixels(self.pos.0 - 10.0))
        .top(Pixels(self.pos.1 - 10.0))
        .width(Pixels(20.0))
        .height(Pixels(20.0));
    }
}

pub struct ConstantView {}

impl View for ConstantView {
    fn element(&self) -> Option<&'static str> {
        Some("Constant")
    }

    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        // println!("Constant draw {:?}", bounds);

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
