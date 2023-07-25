use crate::common::{EguiComponent, Ports, Signal, SignalUnsigned, Simulator};
use crate::components::Probe;
use crate::gui_egui::component_ui::{
    input_port, input_selector, pos_slider, properties_window, rect_with_hover,
};
use crate::gui_egui::editor::{EditorMode, EditorRenderReturn};
use egui::{Align2, Area, Color32, Order, PointerButton, Pos2, Rect, RichText, Vec2};

#[typetag::serde]
impl EguiComponent for Probe {
    fn render(
        &self,
        ui: &mut egui::Ui,
        simulator: Option<&mut Simulator>,
        offset: egui::Vec2,
        scale: f32,
        clip_rect: Rect,
        editor_mode: EditorMode,
    ) -> Option<Vec<egui::Response>> {
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let input_port = self.input_port.clone();
        let value = match simulator {
            Some(s) => s.get_input_val(&input_port.input),
            None => Signal::Data(0),
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
                ui.label(
                    RichText::new(format!("{:?}", value))
                        .size(scale * 12f32)
                        .background_color(Color32::LIGHT_BLUE),
                )
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
        Some(vec![r])
    }

    fn render_editor(
        &mut self,
        ui: &mut egui::Ui,
        simulator: Option<&mut Simulator>,
        offset: egui::Vec2,
        scale: f32,
        clip_rect: egui::Rect,
        id_ports: &[(crate::common::Id, Ports)],
        editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        let mut delete = false;
        let r_vec =
            Probe::render(self, ui, simulator, offset, scale, clip_rect, editor_mode).unwrap();
        let resp = &r_vec[0];
        if resp.dragged_by(egui::PointerButton::Primary) {
            let delta = resp.drag_delta() / scale;
            self.pos = (self.pos.0 + delta.x, self.pos.1 + delta.y);
        }

        if resp.drag_released_by(PointerButton::Primary)
            && resp.interact_pointer_pos().unwrap().x < offset.x
        {
            delete = true;
        }

        properties_window(
            ui,
            self.id.clone(),
            resp,
            &mut self.egui_x.properties_window,
            |ui| {
                let mut clicked_dropdown = false;
                input_port(ui, &mut self.egui_x.id_tmp, &mut self.id, id_ports);
                pos_slider(ui, &mut self.pos);
                clicked_dropdown |= input_selector(ui, &mut self.input_port, id_ports);
                clicked_dropdown
            },
        );

        EditorRenderReturn {
            delete,
            resp: Some(r_vec),
        }
    }

    fn size(&self) -> Rect {
        Rect {
            min: Pos2 {
                x: -10f32,
                y: -10f32,
            },
            max: Pos2 { x: 10f32, y: 10f32 },
        }
    }

    fn ports_location(&self) -> Vec<(crate::common::Id, Pos2)> {
        let own_pos = Vec2::new(self.pos.0, self.pos.1);
        vec![(
            self.input_port.port_id.clone(),
            Pos2::new(-10f32, 0f32) + own_pos,
        )]
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }

    fn set_id_tmp(&mut self) {
        self.egui_x.id_tmp = self.id.clone();
    }
}
