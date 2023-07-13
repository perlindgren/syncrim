use crate::common::{EguiComponent, Simulator};
use crate::components::Probe;
use egui::{Align2, Area, Color32, Order, Rect, RichText};

#[typetag::serde]
impl EguiComponent for Probe {
    fn render(
        &self,
        ui: &mut egui::Ui,
        simulator: Option<Simulator>,
        offset: egui::Vec2,
        scale: f32,
        clip_rect: Rect,
    ) {
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let input = self.input.clone();
        let value = match simulator {
            Some(s) => s.get_input_val(&input),
            None => 0,
        };
        Area::new(self.id.to_string())
            .order(Order::Middle)
            .current_pos(offset.to_pos2())
            .movable(false)
            .enabled(true)
            .pivot(Align2::CENTER_CENTER)
            .show(ui.ctx(), |ui| {
                ui.set_clip_rect(clip_rect);
                ui.label(
                    RichText::new(format!("{:?}", value))
                        .size(scale * 12f32)
                        .background_color(Color32::LIGHT_BLUE),
                )
                .on_hover_text(format!("{:#x}", value));
            });
    }
}
