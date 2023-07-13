use crate::common::{EguiComponent, Simulator};
use crate::components::Register;
use crate::gui_egui::helper::offset_helper;

#[typetag::serde]
impl EguiComponent for Register {
    fn render(
        &self,
        ui: &mut egui::Ui,
        _simulator: Option<Simulator>,
        offset: egui::Vec2,
        scale: f32,
        _clip_rect: egui::Rect,
    ) {
        // 21x41
        // middle: 11x 21y (0 0)
        let oh: fn((f32, f32), f32, egui::Vec2) -> egui::Pos2 = offset_helper;
        let mut offset = offset;
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
}
