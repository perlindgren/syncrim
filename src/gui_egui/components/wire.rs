use crate::common::{EguiComponent, Simulator};
use crate::components::Wire;
use crate::gui_egui::helper::offset_helper;
use egui::{
    containers, Color32, Id, PointerButton, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, Vec2,
};

#[typetag::serde]
impl EguiComponent for Wire {
    fn render(
        &self,
        ui: &mut Ui,
        simulator: Option<Simulator>,
        offset: Vec2,
        scale: f32,
        _clip_rect: Rect,
    ) -> Option<Response> {
        let oh: fn((f32, f32), f32, Vec2) -> Pos2 = offset_helper;
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let s = scale;
        let o = offset;
        ui.painter().add(Shape::line_segment(
            [
                oh((0f32, 0f32), s, o),
                oh((self.delta.0, self.delta.1), s, o),
            ],
            Stroke {
                width: scale,
                color: Color32::BLACK,
            },
        ));
        let rect = Rect {
            min: oh((0f32, 0f32), s, o),
            max: oh((self.delta.0, self.delta.1), s, o),
        };
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
    ) -> bool {
        let mut delete = false;
        let resp = Wire::render(self, ui, simulator, offset, scale, clip_rect).unwrap();
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
            min: Pos2 { x: 0f32, y: 0f32 },
            max: Pos2 {
                x: self.delta.0,
                y: self.delta.1,
            },
        }
    }
}
