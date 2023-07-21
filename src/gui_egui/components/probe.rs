use crate::common::{Components, EguiComponent, Simulator};
use crate::components::Probe;
use crate::gui_egui::helper::{out_of_bounds, unique_component_name, EditorRenderReturn};
use egui::{
    Align2, Area, Color32, ComboBox, Frame, Margin, Order, PointerButton, Pos2, Rect, RichText,
    Rounding, Sense, Window,
};
use epaint::Shadow;

#[typetag::serde]
impl EguiComponent for Probe {
    fn render(
        &self,
        ui: &mut egui::Ui,
        simulator: Option<Simulator>,
        offset: egui::Vec2,
        scale: f32,
        clip_rect: Rect,
    ) -> Option<Vec<egui::Response>> {
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
                    ui.label(format!("Id: {}", self.id.clone()));
                    ui.label(format!("{:?}", value));
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
    ) -> EditorRenderReturn {
        let mut delete = false;
        let r_vec = Probe::render(self, ui, simulator, offset, scale, clip_rect).unwrap();
        let resp = &r_vec[0];
        if resp.dragged_by(egui::PointerButton::Primary) {
            let delta = resp.drag_delta() / scale;
            self.pos = (self.pos.0 + delta.x, self.pos.1 + delta.y);
        }

        if resp.drag_released_by(PointerButton::Primary) {
            if resp.interact_pointer_pos().unwrap().x < offset.x {
                delete = true;
            }
        }

        if self.properties_window {
            let mut input = self.input.id.clone();
            let mut input_field = self.input.field.clone();
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
                    ui.horizontal(|ui| {
                        ComboBox::from_label("input")
                            .selected_text(format!("{}", input))
                            .show_ui(ui, |ui| {
                                for c in cs.iter() {
                                    let id = match c.try_borrow_mut() {
                                        Ok(a) => a.get_id_ports().0.clone(),
                                        Err(e) => self.id.clone(),
                                    };
                                    ui.selectable_value(&mut input, id.clone(), id);
                                }
                            });
                        ComboBox::from_label("field")
                            .selected_text(format!("{}", input_field))
                            .show_ui(ui, |ui| {
                                for c in cs.iter() {
                                    let id = match c.try_borrow_mut() {
                                        Ok(a) => a.get_id_ports().0.clone(),
                                        Err(e) => self.id.clone(),
                                    };
                                    if id != input {
                                        continue;
                                    }
                                    let fields = match c.try_borrow_mut() {
                                        Ok(a) => a.get_id_ports().1.outputs,
                                        Err(_) => vec![self.input.id.clone()],
                                    };
                                    for field in fields {
                                        ui.selectable_value(&mut input_field, field.clone(), field);
                                    }
                                }
                            });
                    });
                    self.input.id = input;
                    self.input.field = input_field;
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
}
