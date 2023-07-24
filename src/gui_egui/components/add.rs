use crate::gui_egui::component_ui::{
    input_id, input_selector, pos_slider, properties_window, rect_with_hover,
};
use crate::gui_egui::helper::{
    editor_mode_to_sense, offset_helper, out_of_bounds, unique_component_name,
};
use crate::{
    common::{Components, EditorMode, EditorRenderReturn, EguiComponent, Ports, Simulator},
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
        _simulator: Option<&mut Simulator>,
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
        let r = rect_with_hover(rect, clip_rect, editor_mode, ui, self.id.clone(), |ui| {
            ui.label(format!("Id: {}", self.id.clone()));
            ui.label("ALU");
        });
        Some(vec![r])
    }

    fn render_editor(
        &mut self,
        ui: &mut egui::Ui,
        simulator: Option<&mut Simulator>,
        offset: egui::Vec2,
        scale: f32,
        clip_rect: egui::Rect,
        id_ports: &Vec<(crate::common::Id, Ports)>,
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

        properties_window(
            ui,
            self.id.clone(),
            resp,
            &mut self.egui_x.properties_window,
            |ui| {
                input_id(ui, &mut self.egui_x.id_tmp, &mut self.id, id_ports);
                pos_slider(ui, &mut self.pos);
                input_selector(ui, &mut self.a_in, id_ports);
                input_selector(ui, &mut self.b_in, id_ports);
            },
        );

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
            (String::from("overflow"), Pos2::new(0f32, -40f32) + own_pos),
        ]
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }
}
