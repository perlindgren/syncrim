use crate::common::Input;
use crate::components::{ADD_OUT_ID, ADD_OVERFLOW_ID};
use crate::gui_egui::component_ui::{
    drag_logic, input_change_id, input_selector, pos_drag_value, properties_window,
    rect_with_hover, visualize_ports,
};
use crate::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use crate::gui_egui::gui::EguiExtra;
use crate::gui_egui::helper::offset_helper;
use crate::{
    common::{EguiComponent, Ports, Simulator},
    components::Add,
};
use egui::{Color32, Pos2, Rect, Response, Shape, Stroke, Ui, Vec2};

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
        let offset_old = offset;
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let s = scale;
        let o = offset;
        // The shape
        #[rustfmt::skip] // stop formate from "compacting" our vec, doesn't affect anything else
        let shape: Vec<(f32, f32)> = vec![
            (-20f32, -40f32),
            (0f32, -40f32),
            (20f32, -20f32),
            (20f32, 20f32),
            (0f32, 40f32),
            (-20f32, 40f32),
            (-20f32, 20f32),
            (-10f32, 0f32),
            (-20f32, -20f32),
        ];

        let comp_scale = self.scale;

        // The shape
        ui.painter().add(Shape::closed_line(
            shape
                .iter()
                .map(|(x, y)| oh((x * comp_scale, y * comp_scale), s, o))
                .collect(),
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
            min: oh((-20.0 * self.scale, -40.0 * self.scale), s, o),
            max: oh((20.0 * self.scale, 40.0 * self.scale), s, o),
        };
        let r = rect_with_hover(rect, clip_rect, editor_mode, ui, self.id.clone(), |ui| {
            ui.label(format!("Id: {}", self.id.clone()));
            ui.label("Adder");
        });
        match editor_mode {
            EditorMode::Simulator => (),
            _ => visualize_ports(ui, self.ports_location(), offset_old, scale, clip_rect),
        }
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
        grid: &GridOptions,
        editor_mode: EditorMode,
    ) -> EditorRenderReturn {
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
        let delete = drag_logic(
            ui.ctx(),
            resp,
            &mut self.pos,
            &mut context.pos_tmp,
            scale,
            offset,
            grid,
        );

        properties_window(
            ui,
            self.id.clone(),
            resp,
            &mut context.properties_window,
            |ui| {
                let mut clicked_dropdown = false;
                input_change_id(ui, &mut context.id_tmp, &mut self.id, id_ports);
                pos_drag_value(ui, &mut self.pos);
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
                Pos2::new(-20f32, -20f32) * self.scale + own_pos,
            ),
            (
                crate::components::ADD_B_IN_ID.to_string(),
                Pos2::new(-20f32, 20f32) * self.scale + own_pos,
            ),
            (
                crate::components::ADD_OUT_ID.to_string(),
                Pos2::new(20f32, 0f32) * self.scale + own_pos,
            ),
            (
                crate::components::ADD_OVERFLOW_ID.to_string(),
                Pos2::new(0f32, -40f32) * self.scale + own_pos,
            ),
        ]
    }

    fn get_input_location(&self, id: Input) -> Option<(f32, f32)> {
        let loc = self
            .ports_location()
            .iter()
            .map(|(_, loc)| <(f32, f32)>::from(loc))
            .collect::<Vec<(f32, f32)>>();

        if id == self.a_in {
            Some(loc[0])
        } else if id == self.b_in {
            Some(loc[1])
        } else if id == Input::new(&self.id, ADD_OUT_ID) {
            Some(loc[2])
        } else if id == Input::new(&self.id, ADD_OVERFLOW_ID) {
            Some(loc[3])
        } else {
            None
        }
    }

    fn top_padding(&self) -> f32 {
        40f32
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }

    fn get_pos(&self) -> (f32, f32) {
        self.pos
    }
}
