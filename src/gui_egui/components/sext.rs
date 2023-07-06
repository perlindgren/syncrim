use crate::common::{Component, EguiComponent, Input, Simulator};
use crate::components::Sext;
use serde::{Deserialize, Serialize};

#[typetag::serde]
impl EguiComponent for Sext {
    fn render(&self, ui: &mut egui::Ui, simulator: Simulator, offset: egui::Vec2, scale: f32) {
        todo!("implement sext");
    }
}
