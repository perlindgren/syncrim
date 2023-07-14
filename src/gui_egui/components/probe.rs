use crate::common::{EguiComponent, Simulator};
use crate::components::Probe;
use crate::gui_egui::helper::out_of_bounds;
use egui::{Align2, Area, Color32, Order, Rect, RichText, Sense};

#[typetag::serde]
impl EguiComponent for Probe {
    fn render(
        &self,
        ui: &mut egui::Ui,
        simulator: Option<Simulator>,
        offset: egui::Vec2,
        scale: f32,
        clip_rect: Rect,
    ) -> Option<egui::Response> {
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let input = self.input.clone();
        let value = match simulator {
            Some(s) => s.get_input_val(&input),
            None => 0,
        };
        let area = Area::new(self.id.to_string())
            .order(Order::Middle)
            .current_pos(offset.to_pos2())
            .movable(false)
            .enabled(true)
            .interactable(false)
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

        let rect = area.response.rect;
        let rect = out_of_bounds(rect, clip_rect);
        let r = ui.allocate_rect(
            rect,
            Sense {
                click: true,
                drag: true,
                focusable: true,
            },
        );
        if r.hovered() && !r.dragged() {
            egui::containers::popup::show_tooltip_for(
                ui.ctx(),
                egui::Id::new(self.id.clone()),
                &rect,
                |ui| {
                    ui.label(format!("{:?}", value));
                },
            );
        }
        Some(r)
    }

    fn render_editor(
        &mut self,
        ui: &mut egui::Ui,
        simulator: Option<Simulator>,
        offset: egui::Vec2,
        scale: f32,
        clip_rect: egui::Rect,
    ) {
        let resp = Probe::render(self, ui, simulator, offset, scale, clip_rect).unwrap();
        if resp.dragged_by(egui::PointerButton::Primary) {
            let delta = resp.drag_delta();
            self.pos = (self.pos.0 + delta.x, self.pos.1 + delta.y);
        }
    }
}
