use crate::common::{EguiComponent, Simulator};
use crate::components::Constant;

#[typetag::serde]
impl EguiComponent for Constant {
    fn render(&self, ui: &mut egui::Ui, _simulator: Simulator, offset: egui::Vec2, scale: f32) {
        let mut offset = offset.clone();
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let w = egui::Window::new(self.id.to_string())
            .movable(false)
            .frame(egui::Frame {
                inner_margin: egui::Margin::same(1f32),
                outer_margin: egui::Margin::same(1f32),
                rounding: egui::Rounding::none(),
                shadow: epaint::Shadow::NONE,
                fill: egui::Color32::LIGHT_GREEN,
                stroke: egui::Stroke::NONE,
            })
            .fixed_pos(egui::Pos2 {
                x: offset.x,
                y: offset.y,
            })
            .title_bar(false)
            .resizable(false)
            .pivot(egui::Align2::CENTER_CENTER);
        w.show(ui.ctx(), |ui| {
            ui.label(egui::RichText::new(self.value.to_string()).size(scale * 12f32))
                .on_hover_text(format!("{:#x}", self.value));
        });
    }
}
