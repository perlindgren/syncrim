use crate::common::{EguiComponent, Simulator};
use crate::components::Sext;
use crate::gui_egui::editor::EditorMode;
use crate::gui_egui::gui::EguiExtra;

#[typetag::serde]
impl EguiComponent for Sext {
    fn render(
        &self,
        _ui: &mut egui::Ui,
        _context: &mut EguiExtra,
        _simulator: Option<&mut Simulator>,
        _offset: egui::Vec2,
        _scale: f32,
        _clip_rect: egui::Rect,
        _editor_mode: EditorMode,
    ) -> Option<Vec<egui::Response>> {
        todo!();
    }
}
