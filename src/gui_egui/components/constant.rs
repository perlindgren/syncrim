use crate::common::{
    Components, EditorMode, EditorRenderReturn, EguiComponent, Ports, Signal, SignalUnsigned,
    Simulator,
};
use crate::components::Constant;
use crate::gui_egui::component_ui::{input_id, pos_slider, properties_window, rect_with_hover};
use crate::gui_egui::helper::{editor_mode_to_sense, out_of_bounds, unique_component_name};
use egui::{
    Align2, Area, Color32, Frame, Margin, Order, PointerButton, Pos2, Rect, RichText, Rounding,
    Sense, Vec2, Window,
};
use epaint::Shadow;

#[typetag::serde]
impl EguiComponent for Constant {
    fn render(
        &self,
        ui: &mut egui::Ui,
        _simulator: Option<&mut Simulator>,
        offset: egui::Vec2,
        scale: f32,
        clip_rect: Rect,
        editor_mode: EditorMode,
    ) -> Option<Vec<egui::Response>> {
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
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
                    RichText::new(format!("{:?}", self.value))
                        .size(scale * 12f32)
                        .background_color(Color32::LIGHT_GREEN),
                )
                .on_hover_text({
                    let r: Result<SignalUnsigned, String> = self.value.try_into();
                    match r {
                        Ok(data) => format!("{:#x}", data),
                        _ => format!("{:?}", self.value),
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
                ui.label(format!("{:?}", self.value));
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
        id_ports: &Vec<(crate::common::Id, Ports)>,
        editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        let mut delete = false;
        let r_vec =
            Constant::render(self, ui, simulator, offset, scale, clip_rect, editor_mode).unwrap();
        let resp = &r_vec[0];
        if resp.dragged_by(PointerButton::Primary) {
            let delta = resp.drag_delta() / scale;
            self.pos = (self.pos.0 + delta.x, self.pos.1 + delta.y);
        }
        if resp.drag_released_by(PointerButton::Primary) {
            if resp.interact_pointer_pos().unwrap().x < offset.x {
                delete = true;
            }
        }

        properties_window(
            ui,
            self.id.clone(),
            resp,
            &mut self.egui_x.properties_window,
            |ui| {
                input_id(ui, &mut self.egui_x.id_tmp, &mut self.id, id_ports);
                pos_slider(ui, &mut self.pos);
                // todo: Fix this
                /*
                match &mut self.value {
                    Signal::Data(d) => {
                        ui.add(egui::Slider::new(&mut d, u32::MIN..=u32::MAX).text("value"));
                    }
                    _ => (),
                }
                */
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
        vec![(String::from("out"), Pos2::new(10f32, 0f32) + own_pos)]
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }

    fn set_id_tmp(&mut self) {
        self.egui_x.id_tmp = self.id.clone();
    }
}
