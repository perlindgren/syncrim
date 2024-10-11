use crate::common::{Component, EguiComponent, Ports, Simulator};
use crate::components::PassThrough;
use crate::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use crate::gui_egui::gui::EguiExtra;
use crate::gui_egui::helper::basic_component_gui;
use egui::{Pos2, Rect, Response, Ui, Vec2};

#[typetag::serde]
impl EguiComponent for PassThrough {
    /// TODO this need to be rewritten when newer helper functions becomes available
    fn render(
        &self,
        _ui: &mut Ui,
        _context: &mut EguiExtra,
        _simulator: Option<&mut Simulator>,
        _offset: Vec2,
        _scale: f32,
        _clip_rect: Rect,
        _editor_mode: EditorMode,
    ) -> Option<Vec<Response>> {
        Some(vec![])
    }

    fn render_editor(
        &mut self,
        ui: &mut Ui,
        _context: &mut EguiExtra,
        simulator: Option<&mut Simulator>,
        offset: Vec2,
        scale: f32,
        clip_rect: Rect,
        _id_ports: &[(crate::common::Id, Ports)],
        _grid: &GridOptions,
        _editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        basic_component_gui(self, &simulator, ui.ctx(), offset, scale, clip_rect, |ui| {
            ui.label("\u{27A1}");
        });
        EditorRenderReturn {
            delete: false,
            resp: Some(vec![]),
        }
    }

    fn ports_location(&self) -> Vec<(crate::common::Id, Pos2)> {
        let own_pos = Vec2::new(self.pos.0, self.pos.1);
        vec![
            (
                crate::components::REGISTER_R_IN_ID.to_string(),
                Pos2::new(-10f32, 0f32) + own_pos,
            ),
            (
                crate::components::REGISTER_OUT_ID.to_string(),
                Pos2::new(10f32, 0f32) + own_pos,
            ),
        ]
    }

    fn top_padding(&self) -> f32 {
        20f32
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }

    fn get_pos(&self) -> (f32, f32) {
        self.pos
    }
}
