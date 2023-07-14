use crate::common::{EguiComponent, Simulator};
use crate::components::Sext;

#[typetag::serde]
impl EguiComponent for Sext {
    fn render(
        &self,
        _ui: &mut egui::Ui,
        _simulator: Option<Simulator>,
        _offset: egui::Vec2,
        _scale: f32,
        _clip_rect: egui::Rect,
    ) -> Option<egui::Response> {
        todo!("implement sext");
        None
    }
}
