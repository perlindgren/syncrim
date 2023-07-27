use crate::gui_egui::component_ui::{
    input_change_id, input_selector, pos_slider, properties_window, rect_with_hover,
};
use crate::gui_egui::editor::{EditorMode, EditorRenderReturn};
use crate::gui_egui::gui::EguiExtra;
use crate::gui_egui::helper::offset_helper;
use crate::{
    common::{EguiComponent, Ports, Simulator},
    components::Add,
};
use egui::{Color32, PointerButton, Pos2, Rect, Response, Shape, Stroke, Ui, Vec2};

#[typetag::serde]
impl EguiComponent for Add {
    fn render(
        &self,
        ui: &mut Ui,
        _context: &mut EguiExtra,
        _simulator: Option<&mut Simulator>,
        offset: Vec2,
        scale: f32,
        clip_rect: Rect,
        editor_mode: EditorMode,
    ) -> Option<Vec<Response>> {
        // 41x81
        // middle: 21x 41y (0 0)
        let oh: fn((f32, f32), f32, Vec2) -> Pos2 = offset_helper;
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let s = scale;
        let o = offset;
        // The shape
        ui.painter().add(Shape::closed_line(
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
            Stroke {
                width: scale,
                color: Color32::RED,
            },
        ));
        // plus sign
        ui.painter().add(Shape::line_segment(
            [oh((0f32, 0f32), s, o), oh((10f32, 0f32), s, o)],
            Stroke {
                width: scale,
                color: Color32::BLACK,
            },
        ));
        ui.painter().add(Shape::line_segment(
            [oh((5f32, -5f32), s, o), oh((5f32, 5f32), s, o)],
            Stroke {
                width: scale,
                color: Color32::BLACK,
            },
        ));
        let rect = Rect {
            min: oh((-20f32, -40f32), s, o),
            max: oh((20f32, 40f32), s, o),
        };
        let r = rect_with_hover(rect, clip_rect, editor_mode, ui, self.id.clone(), |ui| {
            ui.label(format!("Id: {}", self.id.clone()));
            ui.label("ALU");
        });
        Some(vec![r])
    }

    fn render_editor(
        &mut self,
        ui: &mut Ui,
        context: &mut EguiExtra,
        simulator: Option<&mut Simulator>,
        offset: Vec2,
        scale: f32,
        clip_rect: Rect,
        id_ports: &[(crate::common::Id, Ports)],
        editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        let mut delete = false;
        let r_vec = Add::render(
            self,
            ui,
            context,
            simulator,
            offset,
            scale,
            clip_rect,
            editor_mode,
        )
        .unwrap();
        let resp = &r_vec[0];
        if resp.dragged_by(PointerButton::Primary) {
            let delta = resp.drag_delta() / scale;
            self.pos = (self.pos.0 + delta.x, self.pos.1 + delta.y);
        }

        if resp.drag_released_by(PointerButton::Primary)
            && resp.interact_pointer_pos().unwrap().x < offset.x
        {
            delete = true;
        }

        properties_window(
            ui,
            self.id.clone(),
            resp,
            &mut context.properties_window,
            |ui| {
                let mut clicked_dropdown = false;
                input_change_id(ui, &mut context.id_tmp, &mut self.id, id_ports);
                pos_slider(ui, &mut self.pos);
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.a_in,
                    crate::components::ADD_A_IN_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.b_in,
                    crate::components::ADD_B_IN_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown
            },
        );

        EditorRenderReturn {
            delete,
            resp: Some(r_vec),
        }
    }

    fn ports_location(&self) -> Vec<(crate::common::Id, Pos2)> {
        let own_pos = Vec2::new(self.pos.0, self.pos.1);
        vec![
            (
                crate::components::ADD_A_IN_ID.to_string(),
                Pos2::new(-20f32, -20f32) + own_pos,
            ),
            (
                crate::components::ADD_B_IN_ID.to_string(),
                Pos2::new(-20f32, 20f32) + own_pos,
            ),
            (
                crate::components::ADD_OUT_ID.to_string(),
                Pos2::new(20f32, 0f32) + own_pos,
            ),
            (
                crate::components::ADD_OVERFLOW_ID.to_string(),
                Pos2::new(0f32, -40f32) + own_pos,
            ),
        ]
    }

    fn top_padding(&self) -> f32 {
        40f32
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }
}
