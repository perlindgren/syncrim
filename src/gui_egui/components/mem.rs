use crate::common::{EguiComponent, Ports, Simulator};
use crate::components::Mem;
use crate::gui_egui::component_ui::{
    drag_logic, input_change_id, input_selector, pos_drag_value, properties_window,
    rect_with_hover, visualize_ports,
};
use crate::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use crate::gui_egui::gui::EguiExtra;
use crate::gui_egui::helper::offset_helper;
use egui::{Color32, Pos2, Rect, Response, Rounding, Shape, Slider, Stroke, Ui, Vec2};

#[typetag::serde]
impl EguiComponent for Mem {
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
        // 201x101
        // middle: 101x 51y (0 0)
        let oh: fn((f32, f32), f32, Vec2) -> Pos2 = offset_helper;
        let offset_old = offset;
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let s = scale;
        let o = offset;

        // The shape
        let rect = Rect {
            min: oh((-self.width / 2f32, -self.height / 2f32), s, o),
            max: oh((self.width / 2f32, self.height / 2f32), s, o),
        };
        ui.painter().add(Shape::rect_stroke(
            rect,
            Rounding::none(),
            Stroke {
                width: scale,
                color: Color32::BLACK,
            },
        ));

        let r = rect_with_hover(rect, clip_rect, editor_mode, ui, self.id.clone(), |ui| {
            ui.label(format!("Id: {}", self.id.clone()));
            ui.label("Mem");
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
        let r_vec = Mem::render(
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
        context.size_rect = resp.rect;
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
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut self.width, 0f32..=400f32).text("width"));
                    ui.add(Slider::new(&mut self.height, 0f32..=400f32).text("height"));
                });
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.data,
                    crate::components::MEM_DATA_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.addr,
                    crate::components::MEM_ADDR_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.ctrl,
                    crate::components::MEM_CTRL_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.sext,
                    crate::components::MEM_SEXT_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.size,
                    crate::components::MEM_SIZE_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                // todo: something about memory?
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
                crate::components::MEM_DATA_ID.to_string(),
                Pos2::new(
                    self.width / 10f32 * 1f32 - self.width / 2f32,
                    -self.height / 2f32,
                ) + own_pos,
            ),
            (
                crate::components::MEM_ADDR_ID.to_string(),
                Pos2::new(
                    self.width / 10f32 * 2f32 - self.width / 2f32,
                    -self.height / 2f32,
                ) + own_pos,
            ),
            (
                crate::components::MEM_CTRL_ID.to_string(),
                Pos2::new(
                    self.width / 10f32 * 3f32 - self.width / 2f32,
                    -self.height / 2f32,
                ) + own_pos,
            ),
            (
                crate::components::MEM_SEXT_ID.to_string(),
                Pos2::new(
                    self.width / 10f32 * 4f32 - self.width / 2f32,
                    -self.height / 2f32,
                ) + own_pos,
            ),
            (
                crate::components::MEM_SIZE_ID.to_string(),
                Pos2::new(
                    -self.width / 10f32 * 3f32 + self.width / 2f32,
                    -self.height / 2f32,
                ) + own_pos,
            ),
            (
                crate::components::MEM_DATA_OUT_ID.to_string(),
                Pos2::new(
                    -self.width / 10f32 * 2f32 + self.width / 2f32,
                    -self.height / 2f32,
                ) + own_pos,
            ),
        ]
    }

    fn top_padding(&self) -> f32 {
        self.height / 2f32
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }

    fn get_pos(&self) -> (f32, f32) {
        self.pos
    }
}
