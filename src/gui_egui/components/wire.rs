use crate::common::{
    Components, EditorMode, EditorRenderReturn, EguiComponent, Simulator, SnapPriority,
};
use crate::components::Wire;
use crate::gui_egui::helper::{
    editor_mode_to_sense, offset_helper, out_of_bounds, unique_component_name,
};
use egui::{
    containers, Color32, ComboBox, Frame, Margin, PointerButton, Pos2, Rect, Response, Rounding,
    Shape, Slider, Stroke, Ui, Vec2, Window,
};
use epaint::Shadow;

#[typetag::serde]
impl EguiComponent for Wire {
    fn render(
        &self,
        ui: &mut Ui,
        simulator: Option<Simulator>,
        offset: Vec2,
        scale: f32,
        clip_rect: Rect,
        editor_mode: EditorMode,
    ) -> Option<Vec<Response>> {
        let oh: fn((f32, f32), f32, Vec2) -> Pos2 = offset_helper;
        let offset = offset;
        let s = scale;
        let o = offset;
        let mut line_vec = vec![];
        for pos in self.pos.clone() {
            line_vec.push(oh(pos, s, o));
        }

        ui.painter().add(Shape::line(
            line_vec.clone(),
            Stroke {
                width: scale,
                color: Color32::BLACK,
            },
        ));
        let mut r_vec = vec![];

        for (i, _) in line_vec[1..].iter().enumerate() {
            let (line_top_left, line_bottom_right) =
                if line_vec[i].x > line_vec[i + 1].x || line_vec[i].y > line_vec[i + 1].y {
                    (line_vec[i + 1], line_vec[i])
                } else {
                    (line_vec[i], line_vec[i + 1])
                };
            let rect = Rect {
                min: line_top_left,
                max: line_bottom_right,
                //min: line_vec[i],
                //max: line_vec[i + 1],
            };
            let rect = out_of_bounds(rect, clip_rect);
            let r = ui.allocate_rect(rect, editor_mode_to_sense(editor_mode));
            if r.hovered() && !r.dragged() {
                containers::popup::show_tooltip_for(
                    ui.ctx(),
                    egui::Id::new(self.id.clone()),
                    &rect,
                    |ui| {
                        ui.label(format!("Id: {}", self.id.clone()));
                        match &simulator {
                            Some(s) => {
                                ui.label(format!("{}", s.get_input_val(&self.input)));
                            }
                            _ => (),
                        }
                    },
                );
            }
            r_vec.push(r);
        }

        Some(r_vec)
    }

    fn render_editor(
        &mut self,
        ui: &mut Ui,
        simulator: Option<Simulator>,
        offset: Vec2,
        scale: f32,
        clip_rect: Rect,
        cs: &Components,
        editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        let mut delete = false;
        let r_vec =
            Wire::render(self, ui, simulator, offset, scale, clip_rect, editor_mode).unwrap();
        for (i, resp) in r_vec.iter().enumerate() {
            if resp.dragged_by(PointerButton::Primary) {
                let delta = resp.drag_delta() / scale;
                self.pos[i] = (self.pos[i].0 + delta.x, self.pos[i].1 + delta.y);
                self.pos[i + 1] = (self.pos[i + 1].0 + delta.x, self.pos[i + 1].1 + delta.y);
            }
            if resp.drag_released_by(PointerButton::Primary) {
                if resp.interact_pointer_pos().unwrap().x < offset.x {
                    delete = true;
                }
            }
            if self.properties_window {
                let mut input = self.input.id.clone();
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
                                Slider::new(&mut self.pos[i].0, 0f32..=1000f32)
                                    .text("start x")
                                    .clamp_to_range(false),
                            );
                            ui.add(
                                Slider::new(&mut self.pos[i].1, 0f32..=1000f32)
                                    .text("start y")
                                    .clamp_to_range(false),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.add(
                                Slider::new(&mut self.pos[i + 1].0, 0f32..=1000f32)
                                    .text("end x")
                                    .clamp_to_range(false),
                            );
                            ui.add(
                                Slider::new(&mut self.pos[i + 1].1, 0f32..=1000f32)
                                    .text("end y")
                                    .clamp_to_range(false),
                            );
                        });
                        let r = ComboBox::from_label("input")
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
                        self.input.id = input;
                    });
                if resp.unwrap().response.clicked_elsewhere() {
                    self.properties_window = false;
                }
            }

            if resp.clicked_by(PointerButton::Secondary) {
                // Open properties window
                self.properties_window = true;
            }
        }

        EditorRenderReturn {
            delete,
            resp: Some(r_vec),
        }
    }

    // This isn't really it's true size
    fn size(&self) -> Rect {
        Rect {
            min: Pos2 { x: 0f32, y: 0f32 },
            max: Pos2 {
                x: self.pos[1].0,
                y: self.pos[1].1,
            },
        }
    }

    fn ports_location(&self) -> Vec<(crate::common::Id, Pos2)> {
        let mut vec: Vec<(crate::common::Id, Pos2)> = vec![];
        for (i, pos) in self.pos.iter().enumerate() {
            vec.push((format!("{}", i), Pos2 { x: pos.0, y: pos.1 }));
        }
        vec
    }

    fn snap_priority(&self) -> SnapPriority {
        SnapPriority::Wire
    }
}
