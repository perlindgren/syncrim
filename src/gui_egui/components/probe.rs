use crate::common::{EguiComponent, Ports, SignalUnsigned, SignalValue, Simulator};
use crate::components::Probe;
use crate::gui_egui::component_ui::{
    drag_logic, input_change_id, input_selector, pos_drag_value, properties_window,
    rect_with_hover, visualize_ports,
};
use crate::gui_egui::editor::{EditorMode, EditorRenderReturn};
use crate::gui_egui::gui::EguiExtra;
use egui::{Align2, Area, Color32, Order, Pos2, Rect, Response, RichText, Ui, Vec2};

#[typetag::serde]
impl EguiComponent for Probe {
    fn render(
        &self,
        ui: &mut Ui,
        _context: &mut EguiExtra,
        simulator: Option<&mut Simulator>,
        offset: Vec2,
        scale: f32,
        clip_rect: Rect,
        editor_mode: EditorMode,
    ) -> Option<Vec<Response>> {
        let offset_old = offset;
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let input = self.input.clone();
        let value = match simulator {
            Some(s) => s.get_input_value(&input),
            None => SignalValue::Uninitialized,
        };
        let area = Area::new(self.id.to_string())
            .order(Order::Middle)
            .current_pos(offset.to_pos2())
            .movable(false)
            .enabled(true)
            .interactable(false)
            .pivot(Align2::CENTER_CENTER)
            .show(ui.ctx(), |ui| {
                ui.set_clip_rect(clip_rect);
                match editor_mode {
                    EditorMode::Simulator => ui.label(
                        RichText::new(format!("{:?}", value))
                            .size(scale * 12f32)
                            .background_color(Color32::LIGHT_BLUE),
                    ),
                    _ => ui.label(
                        RichText::new(format!("{:?}", value))
                            .size(scale * 12f32)
                            .underline(),
                    ),
                }
                .on_hover_text({
                    let r: Result<SignalUnsigned, String> = value.try_into();
                    match r {
                        Ok(data) => format!("{:#x}", data),
                        _ => format!("{:?}", value),
                    }
                });
            });

        let r = rect_with_hover(
            area.response.rect,
            clip_rect,
            editor_mode,
            ui,
            self.id.clone(),
            |ui| {
                ui.label(format!("Id: {}", self.id.clone()));
                ui.label(format!("{:?}", value));
            },
        );
        match editor_mode {
            EditorMode::Simulator => (),
            _ => visualize_ports(ui, self.ports_location(), offset_old, scale, clip_rect),
        }
        Some(vec![r])
    }

    fn render_editor(
        &mut self,
        ui: &mut Ui,
        context: &mut EguiExtra,
        simulator: Option<&mut Simulator>,
        offset: Vec2,
        scale: f32,
        clip_rect: Rect,
        id_ports: &[(crate::common::Id, Ports)],
        editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        let r_vec = Probe::render(
            self,
            ui,
            context,
            simulator,
            offset,
            scale,
            clip_rect,
            editor_mode,
        )
        .unwrap();
        let resp = &r_vec[0];
        let delete = drag_logic(ui.ctx(), resp, &mut self.pos, scale, offset);

        properties_window(
            ui,
            self.id.clone(),
            resp,
            &mut context.properties_window,
            |ui| {
                let mut clicked_dropdown = false;
                input_change_id(ui, &mut context.id_tmp, &mut self.id, id_ports);
                pos_drag_value(ui, &mut self.pos);
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.input,
                    crate::components::PROBE_IN_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown
            },
        );

        EditorRenderReturn {
            delete,
            resp: Some(r_vec),
        }
    }

    fn ports_location(&self) -> Vec<(crate::common::Id, Pos2)> {
        let own_pos = Vec2::new(self.pos.0, self.pos.1);
        vec![(
            crate::components::PROBE_IN_ID.to_string(),
            Pos2::new(0f32, 0f32) + own_pos,
        )]
    }

    fn top_padding(&self) -> f32 {
        // todo: make this accurate?
        10f32
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }
}
