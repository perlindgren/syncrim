use crate::common::{Components, EditorMode, EditorRenderReturn, EguiComponent, Simulator};
use crate::components::Constant;
use crate::gui_egui::helper::{editor_mode_to_sense, out_of_bounds, unique_component_name};
use egui::{
    Align2, Area, Color32, Frame, Margin, Order, PointerButton, Pos2, Rect, RichText, Rounding,
    Sense, Vec2, Window,
};
use epaint::Shadow;

#[typetag::serde]
impl EguiComponent for Constant {
    fn render(
        &self,
        ui: &mut egui::Ui,
        _simulator: Option<Simulator>,
        offset: egui::Vec2,
        scale: f32,
        clip_rect: Rect,
        editor_mode: EditorMode,
    ) -> Option<Vec<egui::Response>> {
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
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
                    RichText::new(self.value.to_string())
                        .size(scale * 12f32)
                        .background_color(Color32::LIGHT_GREEN),
                );
                //.on_hover_text(format!("{:#x}", self.value));
            });
        let rect = area.response.rect;
        let rect = out_of_bounds(rect, clip_rect);
        let r = ui.allocate_rect(rect, editor_mode_to_sense(editor_mode));
        if r.hovered() && !r.dragged() {
            egui::containers::popup::show_tooltip_for(
                ui.ctx(),
                egui::Id::new(self.id.clone()),
                &rect,
                |ui| {
                    ui.label(format!("Id: {}", self.id.clone()));
                    ui.label(self.value.to_string());
                },
            );
        }
        Some(vec![r])
    }

    fn render_editor(
        &mut self,
        ui: &mut egui::Ui,
        simulator: Option<Simulator>,
        offset: egui::Vec2,
        scale: f32,
        clip_rect: egui::Rect,
        cs: &Components,
        editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        let mut delete = false;
        let r_vec =
            Constant::render(self, ui, simulator, offset, scale, clip_rect, editor_mode).unwrap();
        let resp = &r_vec[0];
        if resp.dragged_by(PointerButton::Primary) {
            let delta = resp.drag_delta() / scale;
            self.pos = (self.pos.0 + delta.x, self.pos.1 + delta.y);
        }
        if resp.drag_released_by(PointerButton::Primary) {
            if resp.interact_pointer_pos().unwrap().x < offset.x {
                delete = true;
            }
        }

        if self.properties_window {
            let resp = Window::new(format!("Properties: {}", self.id))
                .frame(Frame {
                    inner_margin: Margin::same(10f32),
                    outer_margin: Margin::same(0f32),
                    rounding: Rounding::same(10f32),
                    shadow: Shadow::small_dark(),
                    fill: ui.visuals().panel_fill,
                    stroke: ui.visuals().window_stroke,
                })
                .default_pos(Pos2 {
                    x: (resp.rect.min.x + resp.rect.max.x) / 2f32,
                    y: (resp.rect.min.y + resp.rect.max.y) / 2f32,
                })
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        let id_label = ui.label("Id: ");
                        let r = ui
                            .text_edit_singleline(&mut self.id_tmp)
                            .labelled_by(id_label.id);
                        if r.lost_focus() && self.id_tmp != self.id {
                            self.id = unique_component_name(cs, self.id_tmp.as_str());
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.add(
                            egui::Slider::new(&mut self.pos.0, 0f32..=1000f32)
                                .text("pos x")
                                .clamp_to_range(false),
                        );
                        ui.add(
                            egui::Slider::new(&mut self.pos.1, 0f32..=1000f32)
                                .text("pos y")
                                .clamp_to_range(false),
                        );
                    });
                    ui.add(egui::Slider::new(&mut self.value, u32::MIN..=u32::MAX).text("value"));
                });
            if resp.unwrap().response.clicked_elsewhere() {
                self.properties_window = false;
            }
        }

        if resp.clicked_by(PointerButton::Secondary) {
            // Open properties window
            self.properties_window = true;
        }

        EditorRenderReturn {
            delete,
            resp: Some(r_vec),
        }
    }

    fn size(&self) -> Rect {
        Rect {
            min: Pos2 {
                x: -10f32,
                y: -10f32,
            },
            max: Pos2 { x: 10f32, y: 10f32 },
        }
    }

    fn ports_location(&self) -> Vec<(crate::common::Id, Pos2)> {
        let own_pos = Vec2::new(self.pos.0, self.pos.1);
        vec![(String::from("out"), Pos2::new(10f32, 0f32) + own_pos)]
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }
}
