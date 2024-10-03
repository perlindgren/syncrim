use crate::common::{EguiComponent, Id, Input, Ports, Simulator};
use crate::components::{ZeroExtend, ZEROEXTEND_OUT_ID};
use crate::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use crate::gui_egui::gui::EguiExtra;
use crate::gui_egui::helper::basic_component_gui;
use egui::{pos2, Pos2, Rect, Response, Ui, Vec2};

const HEIGHT: f32 = 14.0;
const WIDTH: f32 = 70.0;

#[typetag::serde]
impl EguiComponent for ZeroExtend {
    fn render(
        &self,
        ui: &mut Ui,
        _context: &mut EguiExtra,
        simulator: Option<&mut Simulator>,
        offset: Vec2,
        scale: f32,
        clip_rect: Rect,
        _editor_mode: EditorMode,
    ) -> Option<Vec<Response>> {
        basic_component_gui(self, &simulator, ui.ctx(), offset, scale, clip_rect, |ui| {
            ui.set_height(HEIGHT * scale);
            ui.set_width(WIDTH * scale);
            ui.label("Zero Extend");
        })
    }

    fn render_editor(
        &mut self,
        ui: &mut egui::Ui,
        context: &mut EguiExtra,
        simulator: Option<&mut Simulator>,
        offset: egui::Vec2,
        scale: f32,
        clip_rect: egui::Rect,
        _id_ports: &[(Id, Ports)],
        _grid: &GridOptions,
        editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        self.render(
            ui,
            context,
            simulator,
            offset,
            scale,
            clip_rect,
            editor_mode,
        );
        EditorRenderReturn {
            delete: false,
            resp: None,
        }
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }

    fn get_pos(&self) -> (f32, f32) {
        self.pos
    }

    fn top_padding(&self) -> f32 {
        20f32
    }

    fn ports_location(&self) -> Vec<(crate::common::Id, Pos2)> {
        let own_pos = Vec2::new(self.pos.0, self.pos.1);
        const M: f32 = 6.0;
        vec![
            (
                crate::components::ZEROEXTEND_SIGNAL_IN_ID.to_string(),
                pos2(-WIDTH / 2.0 - M, 0.0) + own_pos,
            ),
            (
                crate::components::ZEROEXTEND_OUT_ID.to_string(),
                pos2(WIDTH / 2.0 + M, 0.0) + own_pos,
            ),
        ]
    }

    fn get_input_location(&self, id: Input) -> Option<(f32, f32)> {
        let loc = self
            .ports_location()
            .iter()
            .map(|(_, loc)| <(f32, f32)>::from(loc))
            .collect::<Vec<(f32, f32)>>();

        if id == self.signal_in {
            Some(loc[0])
        } else if id == Input::new(&self.id, ZEROEXTEND_OUT_ID) {
            Some(loc[1])
        } else {
            None
        }
    }
}
