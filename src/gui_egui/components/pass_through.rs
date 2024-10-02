use crate::common::{Component, EguiComponent, Ports, Simulator};
use crate::components::PassThrough;
use crate::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use crate::gui_egui::gui::EguiExtra;
use egui::{
    Align2, Area, Context, InnerResponse, Order, Pos2, Rect, Response, TextWrapMode, Ui, Vec2,
};

#[typetag::serde]
impl EguiComponent for PassThrough {
    /// TODO this need to be rewritten when newer helper functions becomes available
    fn render(
        &self,
        ui: &mut Ui,
        _context: &mut EguiExtra,
        _simulator: Option<&mut Simulator>,
        offset: Vec2,
        scale: f32,
        clip_rect: Rect,
        _editor_mode: EditorMode,
    ) -> Option<Vec<Response>> {
        fn component_area<R>(
            id: String,
            ctx: &Context,
            pos: impl Into<Pos2>,
            content: impl FnOnce(&mut Ui) -> R,
        ) -> InnerResponse<R> {
            Area::new(egui::Id::from(id))
                .order(Order::Middle)
                .current_pos(pos)
                .movable(false)
                .enabled(true)
                .interactable(false)
                .pivot(Align2::CENTER_CENTER)
                .constrain(false)
                .show(ctx, content)
        }

        let offset: Vec2 = offset.into();

        let r = component_area(
            self.get_id_ports().0,
            ui.ctx(),
            Pos2::from(self.get_pos()) * scale + offset,
            |ui| {
                ui.set_clip_rect(clip_rect);

                ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);

                for (_text_style, font) in ui.style_mut().text_styles.iter_mut() {
                    font.size *= scale;
                }
                ui.spacing_mut().button_padding *= scale;
                ui.spacing_mut().item_spacing *= scale;
                ui.spacing_mut().combo_height *= scale;
                ui.spacing_mut().combo_width *= scale;
                ui.spacing_mut().icon_width *= scale;
                ui.spacing_mut().icon_width_inner *= scale;
                ui.spacing_mut().icon_spacing *= scale;
                ui.spacing_mut().interact_size *= scale;

                let mut group = egui::containers::Frame::group(ui.style());
                group.inner_margin *= scale;
                group.rounding *= scale;
                // group.fill = Color32::LIGHT_RED; // Use this ween component background is implemented, probably when we implement dark mode
                group
                    .show(ui, |ui| {
                        ui.label("➡️");
                    })
                    .response
            },
        )
        .inner;
        Some(vec![r])
    }

    fn render_editor(
        &mut self,
        _ui: &mut Ui,
        _context: &mut EguiExtra,
        _simulator: Option<&mut Simulator>,
        _offset: Vec2,
        _scale: f32,
        _clip_rect: Rect,
        _id_ports: &[(crate::common::Id, Ports)],
        _grid: &GridOptions,
        _editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        EditorRenderReturn {
            delete: false,
            resp: Some(vec![]),
        }
    }

    fn ports_location(&self) -> Vec<(crate::common::Id, Pos2)> {
        let own_pos = Vec2::new(self.pos.0, self.pos.1);
        vec![
            (
                crate::components::REGISTER_R_IN_ID.to_string(),
                Pos2::new(-10f32, 0f32) + own_pos,
            ),
            (
                crate::components::REGISTER_OUT_ID.to_string(),
                Pos2::new(10f32, 0f32) + own_pos,
            ),
        ]
    }

    fn top_padding(&self) -> f32 {
        20f32
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }

    fn get_pos(&self) -> (f32, f32) {
        self.pos
    }
}
