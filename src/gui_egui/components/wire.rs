use crate::common::{Component, EguiComponent, Input, Simulator};
use crate::components::Wire;
use crate::gui_egui::helper::offset_helper;
use serde::{Deserialize, Serialize};

#[typetag::serde]
impl EguiComponent for Wire {
    fn render(&self, ui: &mut egui::Ui, simulator: Simulator, offset: egui::Vec2, scale: f32) {
        let oh: fn((f32, f32), f32, egui::Vec2) -> egui::Pos2 = offset_helper;
        let mut offset = offset.clone();
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let s = scale;
        let o = offset;
        ui.painter().add(egui::Shape::line_segment(
            [
                oh((0f32, 0f32), s, o),
                oh((self.delta.0, self.delta.1), s, o),
            ],
            egui::Stroke {
                width: scale,
                color: egui::Color32::BLACK,
            },
        ));
    }
}
