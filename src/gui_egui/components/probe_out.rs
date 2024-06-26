use crate::common::{EguiComponent, Simulator};
use crate::components::ProbeOut;
use crate::gui_egui::editor::EditorMode;
use crate::gui_egui::gui::EguiExtra;
use egui::Rect;

#[typetag::serde]
impl EguiComponent for ProbeOut {
    fn render(
        &self,
        _ui: &mut egui::Ui,
        _context: &mut EguiExtra,
        _simulator: Option<&mut Simulator>,
        _offset: egui::Vec2,
        _scale: f32,
        _clip_rect: Rect,
        _editor_mode: EditorMode,
    ) -> Option<Vec<egui::Response>> {
        todo!();
    }
}
