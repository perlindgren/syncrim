use crate::common::{EditorMode, EguiComponent, Simulator};
use crate::components::ProbeEdit;
use egui::Rect;

#[typetag::serde]
impl EguiComponent for ProbeEdit {
    fn render(
        &self,
        _ui: &mut egui::Ui,
        _simulator: Option<&mut Simulator>,
        _offset: egui::Vec2,
        _scale: f32,
        _clip_rect: Rect,
        _editor_mode: EditorMode,
    ) -> Option<Vec<egui::Response>> {
        todo!();
        None
    }
}
