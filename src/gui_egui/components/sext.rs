use crate::common::{EguiComponent, Simulator};
use crate::components::Sext;
use crate::gui_egui::editor::EditorMode;

#[typetag::serde]
impl EguiComponent for Sext {
    fn render(
        &self,
        _ui: &mut egui::Ui,
        _simulator: Option<&mut Simulator>,
        _offset: egui::Vec2,
        _scale: f32,
        _clip_rect: egui::Rect,
        _editor_mode: EditorMode,
    ) -> Option<Vec<egui::Response>> {
        todo!();
    }
}
