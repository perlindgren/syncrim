use crate::common::{EguiComponent, Simulator};
use crate::components::Wire;
use crate::gui_egui::helper::offset_helper;

#[typetag::serde]
impl EguiComponent for Wire {
    fn render(
        &self,
        ui: &mut egui::Ui,
        _simulator: Option<Simulator>,
        offset: egui::Vec2,
        scale: f32,
        _clip_rect: egui::Rect,
    ) -> Option<egui::Response> {
        let oh: fn((f32, f32), f32, egui::Vec2) -> egui::Pos2 = offset_helper;
        let mut offset = offset;
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
        let rect = egui::Rect {
            min: oh((0f32, 0f32), s, o),
            max: oh((self.delta.0, self.delta.1), s, o),
        };
        Some(ui.allocate_rect(
            rect,
            egui::Sense {
                click: true,
                drag: true,
                focusable: true,
            },
        ))
    }

    fn render_editor(
        &mut self,
        ui: &mut egui::Ui,
        simulator: Option<Simulator>,
        offset: egui::Vec2,
        scale: f32,
        clip_rect: egui::Rect,
    ) {
        let resp = Wire::render(self, ui, simulator, offset, scale, clip_rect).unwrap();
        if resp.dragged_by(egui::PointerButton::Primary) {
            let delta = resp.drag_delta();
            self.pos = (self.pos.0 + delta.x, self.pos.1 + delta.y);
        }
    }
}
