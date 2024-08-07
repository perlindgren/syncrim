use crate::common::{EguiComponent, Simulator};
use crate::components::ZeroExtend;
use crate::gui_egui::editor::EditorMode;
use crate::gui_egui::gui::EguiExtra;
use crate::gui_egui::helper::{component_area, offset_helper};
use egui::{Label, Pos2, Rect, Response, RichText, Ui, Vec2};

#[typetag::serde]
impl EguiComponent for ZeroExtend {
    fn render(
        &self,
        ui: &mut Ui,
        _context: &mut EguiExtra,
        _simulator: Option<&mut Simulator>,
        offset: Vec2,
        scale: f32,
        _clip_rect: Rect,
        _editor_mode: EditorMode,
    ) -> Option<Vec<Response>> {
        // size of the component
        let width = 200f32;
        let height: f32 = 20f32;
        let rect = Rect::from_center_size(
            (Pos2::from(self.pos) * scale + offset),
            Vec2 {
                x: width,
                y: height,
            } * scale,
        );
        let r = component_area(self.id.to_string(), ui.ctx(), rect.center(), |ui| {
            ui.set_height(rect.height());
            ui.set_width(rect.width());
            ui.group(|ui| {
                ui.add_sized(
                    ui.available_size_before_wrap(),
                    // Change string here for another name
                    Label::new(RichText::new("zero_extend").size(12f32 * scale)),
                )
            })
            .response
        })
        .inner;

        Some(vec![r])
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }

    fn get_pos(&self) -> (f32, f32) {
        self.pos
    }
}
