use crate::gui_egui::helper::{offset_helper, out_of_bounds, EditorRenderReturn};
use crate::{
    common::{Components, EguiComponent, Simulator},
    components::Add,
};
use egui::{
    containers::{ComboBox, Window},
    PointerButton, Pos2, Rect, Sense, Vec2,
};

#[typetag::serde]
impl EguiComponent for Add {
    fn render(
        &self,
        ui: &mut egui::Ui,
        _simulator: Option<Simulator>,
        offset: egui::Vec2,
        scale: f32,
        clip_rect: egui::Rect,
    ) -> Option<egui::Response> {
        // 41x81
        // middle: 21x 41y (0 0)
        let oh: fn((f32, f32), f32, Vec2) -> egui::Pos2 = offset_helper;
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let s = scale;
        let o = offset;
        //println!("---- Create Add View");
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
                    ui.label("ALU");
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
        cs: &Components,
    ) -> EditorRenderReturn {
        let mut delete = false;
        let resp = Add::render(self, ui, simulator, offset, scale, clip_rect).unwrap();
        if resp.dragged_by(PointerButton::Primary) {
            let delta = resp.drag_delta();
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
            let resp = Window::new(format!("Properties: {}", self.id)).show(ui.ctx(), |ui| {
                let r = ComboBox::from_label("a_in")
                    .selected_text(format!("{:?}", a_in))
                    .show_ui(ui, |ui| {
                        for c in cs.iter() {
                            let id = match c.try_borrow_mut() {
                                Ok(a) => a.get_id_ports().0.clone(),
                                Err(e) => self.id.clone(),
                            };
                            ui.selectable_value(&mut a_in, id.clone(), id);
                        }
                    });
                let r2 = ComboBox::from_label("b_in")
                    .selected_text(format!("{:?}", b_in))
                    .show_ui(ui, |ui| {
                        for c in cs.iter() {
                            let id = match c.try_borrow_mut() {
                                Ok(a) => a.get_id_ports().0.clone(),
                                Err(e) => self.id.clone(),
                            };
                            ui.selectable_value(&mut b_in, id.clone(), id);
                        }
                    });
            });
            if resp.unwrap().response.clicked_elsewhere() {
                self.properties_window = false;
            }
        }

        if resp.clicked_by(PointerButton::Secondary) {
            println!("opening properties window");
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
            min: Pos2 {
                x: -20f32,
                y: -40f32,
            },
            max: Pos2 { x: 20f32, y: 40f32 },
        }
    }
}
