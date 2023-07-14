use crate::common::{EguiComponent, Simulator};
use crate::components::ProbeOut;
use egui::Rect;

#[typetag::serde]
impl EguiComponent for ProbeOut {
    fn render(
        &self,
        ui: &mut egui::Ui,
        _simulator: Option<Simulator>,
        offset: egui::Vec2,
        scale: f32,
        clip_rect: Rect,
    ) -> Option<egui::Response> {
        todo!();
        None
    }
    /*
    fn interactive_rect(&self, ui: &mut egui::Ui, offset: egui::Vec2, scale: f32) -> egui::Rect {
        todo!()
    }
    */
}
