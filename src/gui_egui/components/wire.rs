use crate::common::{Components, EguiComponent, Simulator};
use crate::components::Wire;
use crate::gui_egui::helper::{
    offset_helper, out_of_bounds, unique_component_name, EditorRenderReturn,
};
use egui::{
    containers, Color32, ComboBox, Frame, Id, Margin, PointerButton, Pos2, Rect, Response,
    Rounding, Sense, Shape, Stroke, Ui, Vec2, Window,
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
        _clip_rect: egui::Rect,
    ) {
        let oh: fn((f32, f32), f32, egui::Vec2) -> egui::Pos2 = offset_helper;
        let offset = offset;
        let s = scale;
        let o = offset;
        let mut line_vec = vec![];
        for pos in self.pos.clone() {
            line_vec.push(oh(pos, s, o));
        }

        ui.painter().add(egui::Shape::line(
            line_vec,
            Stroke {
                width: scale,
                color: Color32::BLACK,
            },
        ));
        let rect = Rect {
            min: oh((0f32, 0f32), s, o),
            max: oh((self.delta.0, self.delta.1), s, o),
        };
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
            containers::popup::show_tooltip_for(ui.ctx(), Id::new(self.id.clone()), &rect, |ui| {
                ui.label(format!("Id: {}", self.id.clone()));
                match simulator {
                    Some(s) => {
                        ui.label(format!("{}", s.get_input_val(&self.input)));
                    }
                    _ => (),
                }
            });
        }

        Some(r)
    }

    fn render_editor(
        &mut self,
        ui: &mut Ui,
        simulator: Option<Simulator>,
        offset: Vec2,
        scale: f32,
        clip_rect: Rect,
        cs: &Components,
    ) -> EditorRenderReturn {
        let mut delete = false;
        let resp = Wire::render(self, ui, simulator, offset, scale, clip_rect).unwrap();
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
                    ui.add(
                        egui::Slider::new(&mut self.delta.0, 0f32..=1000f32)
                            .text("delta x")
                            .clamp_to_range(false),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.delta.1, 0f32..=1000f32)
                            .text("delta y")
                            .clamp_to_range(false),
                    );
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

        EditorRenderReturn {
            delete,
            resp: Some(resp),
        }
    }

    fn size(&self) -> Rect {
        Rect {
            min: Pos2 { x: 0f32, y: 0f32 },
            max: Pos2 {
                x: self.delta.0,
                y: self.delta.1,
            },
        }
    }
}
