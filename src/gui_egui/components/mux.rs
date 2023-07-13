use crate::common::{EguiComponent, Simulator};
use crate::components::Mux;
use crate::gui_egui::helper::offset_helper;

#[typetag::serde]
impl EguiComponent for Mux {
    fn render(
        &self,
        ui: &mut egui::Ui,
        simulator: Option<Simulator>,
        offset: egui::Vec2,
        scale: f32,
        _clip_rect: egui::Rect,
    ) {
        // 41x(20*ports + 11)
        // middle: 21x ((20*ports + 10)/2+1)y (0 0)
        let oh: fn((f32, f32), f32, egui::Vec2) -> egui::Pos2 = offset_helper;
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let s = scale;
        let o = offset;
        let pa = self.m_in.len() as f32;

        // selector
        let select = match simulator {
            Some(s) => s.get_input_val(&self.select),
            None => 0,
        };

        // The shape
        ui.painter().add(egui::Shape::closed_line(
            vec![
                oh((-20f32, pa * (-10f32) - 10f32), s, o),
                oh((0f32, pa * (-10f32) - 10f32), s, o),
                oh((20f32, pa * (-10f32) + 10f32), s, o),
                oh((20f32, pa * (10f32) - 10f32), s, o),
                oh((0f32, pa * (10f32) + 10f32), s, o),
                oh((-20f32, pa * (10f32) + 10f32), s, o),
            ],
            egui::Stroke {
                width: scale,
                color: egui::Color32::BLACK,
            },
        ));
        // select line
        ui.painter().add(egui::Shape::line_segment(
            [
                oh(
                    (-20f32, ((select as f32) * 20f32) - pa * 10f32 + 10f32),
                    s,
                    o,
                ),
                oh((20f32, 0f32), s, o),
            ],
            egui::Stroke {
                width: scale,
                color: egui::Color32::RED,
            },
        ));
    }
}
