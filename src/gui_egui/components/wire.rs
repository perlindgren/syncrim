use crate::common::{EguiComponent, Simulator};
use crate::components::Wire;
use crate::gui_egui::helper::offset_helper;

#[typetag::serde]
impl EguiComponent for Wire {
    fn render(
        &self,
        ui: &mut egui::Ui,
        _simulator: Simulator,
        offset: egui::Vec2,
        scale: f32,
        _clip_rect: egui::Rect,
    ) {
        let oh: fn((f32, f32), f32, egui::Vec2) -> egui::Pos2 = offset_helper;
        let s = scale;
        let o = offset;
        let mut line_vec = vec![];
        for pos in self.pos.clone() {
            line_vec.push(oh(pos, s, o));
        }

        ui.painter().add(egui::Shape::line(
            line_vec,
            egui::Stroke {
                width: scale,
                color: egui::Color32::BLACK,
            },
        ));
    }
}
