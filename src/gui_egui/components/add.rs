use crate::gui_egui::helper::{
    editor_mode_to_sense, offset_helper, out_of_bounds, unique_component_name,
};
use crate::{
    common::{Components, EditorMode, EditorRenderReturn, EguiComponent, Simulator},
    components::Add,
};
use egui::{
    containers::{ComboBox, Window},
    Frame, Margin, PointerButton, Pos2, Rect, Rounding, Vec2,
};
use epaint::Shadow;

#[typetag::serde]
impl EguiComponent for Add {
    fn render(
        &self,
        ui: &mut egui::Ui,
        _simulator: Option<Simulator>,
        offset: egui::Vec2,
        scale: f32,
        clip_rect: egui::Rect,
        editor_mode: EditorMode,
    ) -> Option<Vec<egui::Response>> {
        // 41x81
        // middle: 21x 41y (0 0)
        let oh: fn((f32, f32), f32, Vec2) -> egui::Pos2 = offset_helper;
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let s = scale;
        let o = offset;
        //trace!("---- Create Add View");
        // The shape
        // 40x30
        ui.painter().add(egui::Shape::closed_line(
            vec![
                oh((-20f32, -40f32), s, o),
                oh((0f32, -40f32), s, o),
                oh((20f32, -20f32), s, o),
                oh((20f32, 20f32), s, o),
                oh((0f32, 40f32), s, o),
                oh((-20f32, 40f32), s, o),
                oh((-20f32, 20f32), s, o),
                oh((-10f32, 0f32), s, o),
                oh((-20f32, -20f32), s, o),
            ],
            egui::Stroke {
                width: scale,
                color: egui::Color32::RED,
            },
        ));
        // plus sign
        ui.painter().add(egui::Shape::line_segment(
            [oh((0f32, 0f32), s, o), oh((10f32, 0f32), s, o)],
            egui::Stroke {
                width: scale,
                color: egui::Color32::BLACK,
            },
        ));
        ui.painter().add(egui::Shape::line_segment(
            [oh((5f32, -5f32), s, o), oh((5f32, 5f32), s, o)],
            egui::Stroke {
                width: scale,
                color: egui::Color32::BLACK,
            },
        ));
        let rect = egui::Rect {
            min: oh((-20f32, -40f32), s, o),
            max: oh((20f32, 40f32), s, o),
        };
        let rect = out_of_bounds(rect, clip_rect);
        let r = ui.allocate_rect(rect, editor_mode_to_sense(editor_mode));

        if r.hovered() && !r.dragged() {
            egui::containers::popup::show_tooltip_for(
                ui.ctx(),
                egui::Id::new(self.id.clone()),
                &rect,
                |ui| {
                    ui.label(format!("Id: {}", self.id.clone()));
                    ui.label("ALU");
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
            Add::render(self, ui, simulator, offset, scale, clip_rect, editor_mode).unwrap();
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
            let mut a_in = self.a_in.id.clone();
            let mut b_in = self.b_in.id.clone();
            let mut a_in_field = self.a_in.field.clone();
            let mut b_in_field = self.b_in.field.clone();
            let w_resp = Window::new(format!("Properties: {}", self.id))
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
                        ComboBox::from_label("a_in.id")
                            .selected_text(format!("{}", a_in))
                            .show_ui(ui, |ui| {
                                for c in cs.iter() {
                                    let id = match c.try_borrow_mut() {
                                        Ok(a) => a.get_id_ports().0.clone(),
                                        Err(e) => self.id.clone(),
                                    };
                                    ui.selectable_value(&mut a_in, id.clone(), id);
                                }
                            });
                        ComboBox::from_label("a_in.field")
                            .selected_text(format!("{}", a_in_field))
                            .show_ui(ui, |ui| {
                                for c in cs.iter() {
                                    let id = match c.try_borrow_mut() {
                                        Ok(a) => a.get_id_ports().0.clone(),
                                        Err(e) => self.id.clone(),
                                    };
                                    if id != a_in {
                                        continue;
                                    }
                                    let fields = match c.try_borrow_mut() {
                                        Ok(a) => a.get_id_ports().1.outputs,
                                        Err(_) => vec![],
                                    };
                                    for field in fields {
                                        ui.selectable_value(&mut a_in_field, field.clone(), field);
                                    }
                                }
                            });
                    });

                    ui.horizontal(|ui| {
                        ComboBox::from_label("b_in.id")
                            .selected_text(format!("{}", b_in))
                            .show_ui(ui, |ui| {
                                for c in cs.iter() {
                                    let id = match c.try_borrow_mut() {
                                        Ok(a) => a.get_id_ports().0.clone(),
                                        Err(e) => self.id.clone(),
                                    };
                                    ui.selectable_value(&mut b_in, id.clone(), id);
                                }
                            });
                        ComboBox::from_label("b_in.field")
                            .selected_text(format!("{}", b_in_field))
                            .show_ui(ui, |ui| {
                                for c in cs.iter() {
                                    let id = match c.try_borrow_mut() {
                                        Ok(a) => a.get_id_ports().0.clone(),
                                        Err(e) => self.id.clone(),
                                    };
                                    if id != b_in {
                                        continue;
                                    }
                                    let fields = match c.try_borrow_mut() {
                                        Ok(a) => a.get_id_ports().1.outputs,
                                        Err(_) => vec![],
                                    };
                                    for field in fields {
                                        ui.selectable_value(&mut b_in_field, field.clone(), field);
                                    }
                                }
                            });
                    });
                    self.a_in.id = a_in;
                    self.b_in.id = b_in;
                    self.a_in.field = a_in_field;
                    self.b_in.field = b_in_field;
                });
            if w_resp.unwrap().response.clicked_elsewhere() {
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
                x: -20f32,
                y: -40f32,
            },
            max: Pos2 { x: 20f32, y: 40f32 },
        }
    }

    fn ports_location(&self) -> Vec<(crate::common::Id, Pos2)> {
        let own_pos = Vec2::new(self.pos.0, self.pos.1);
        vec![
            (self.a_in.id.clone(), Pos2::new(-20f32, -20f32) + own_pos),
            (self.b_in.id.clone(), Pos2::new(-20f32, 20f32) + own_pos),
            (String::from("out"), Pos2::new(20f32, 0f32) + own_pos),
        ]
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }
}
