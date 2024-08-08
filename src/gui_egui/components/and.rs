use crate::common::{EguiComponent, Id, Ports, Simulator};
use crate::components::And;
use crate::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use crate::gui_egui::gui::EguiExtra;
use crate::gui_egui::helper::basic_component_gui;
use egui::{Rect, Response, RichText, Ui, Vec2};

#[typetag::serde]
impl EguiComponent for And {
    fn render(
        &self,
        ui: &mut Ui,
        _context: &mut EguiExtra,
        simulator: Option<&mut Simulator>,
        offset: Vec2,
        scale: f32,
        _clip_rect: Rect,
        _editor_mode: EditorMode,
    ) -> Option<Vec<Response>> {
        // size of the component
        let width = 100f32;
        let height: f32 = 0f32;
        basic_component_gui(
            self,
            &simulator,
            ui.ctx(),
            (width, height),
            offset,
            scale,
            |ui| {
                ui.label(RichText::new("And").size(12f32 * scale));
            },
            // This is a hack to stop the compiler from complaining
            // will hopefully be optimized away
            None::<Box<dyn FnOnce(&mut Ui)>>,
        )
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
}
