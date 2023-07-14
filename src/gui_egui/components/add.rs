use crate::gui_egui::helper::{offset_helper, out_of_bounds};
use crate::{
    common::{EguiComponent, Simulator},
    components::Add,
};
use egui::{PointerButton, Pos2, Rect, Sense, Vec2};

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
    ) -> bool {
        let mut delete = false;
        let resp = Add::render(self, ui, simulator, offset, scale, clip_rect).unwrap();
        if resp.dragged_by(PointerButton::Primary) {
            let delta = resp.drag_delta();
            self.pos = (self.pos.0 + delta.x, self.pos.1 + delta.y);
        }
        if resp.drag_released_by(PointerButton::Primary) {
            if self.pos.0 < offset.x {
                println!("delete!");
                delete = true;
            }
        }
        delete
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
