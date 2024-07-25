use crate::common::{EguiComponent, Ports, SignalSigned, SignalUnsigned, SignalValue, Simulator};
use crate::components::{ProbeEdit, TextSignal};
use crate::gui_egui::component_ui::{
    drag_logic, input_change_id, pos_drag_value, properties_window, rect_with_hover,
    visualize_ports,
};
use crate::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use crate::gui_egui::gui::EguiExtra;
use egui::{Align2, Area, DragValue, Order, Pos2, Rect, Response, TextStyle, Ui, Vec2};

#[typetag::serde]
impl EguiComponent for ProbeEdit {
    fn render(
        &self,
        ui: &mut Ui,
        _context: &mut EguiExtra,
        _simulator: Option<&mut Simulator>,
        offset: Vec2,
        scale: f32,
        clip_rect: Rect,
        editor_mode: EditorMode,
    ) -> Option<Vec<Response>> {
        let offset_old = offset;
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let interact = matches!(editor_mode, EditorMode::Simulator);
        let area = Area::new(egui::Id::from(self.id.to_string()))
            .order(Order::Middle)
            .current_pos(offset.to_pos2())
            .movable(false)
            .enabled(true)
            .interactable(interact)
            .pivot(Align2::CENTER_CENTER)
            .constrain(false)
            .show(ui.ctx(), |ui| {
                ui.set_clip_rect(clip_rect);
                let hst = self.edit_history.clone();
                let x = hst.read().unwrap().last().unwrap().text.clone();
                let signal = parse_signal(x.as_str());
                let r = match signal {
                    SignalValue::Data(d) => {
                        let mut val = d;
                        // todo: Somehow make this scale...
                        ui.style_mut() // manage to change the font size but not the background
                            .text_styles
                            .get_mut(&TextStyle::Button)
                            .unwrap()
                            .size = scale * 12f32;
                        let r = ui.add(DragValue::new(&mut val));
                        *self.edit_history.write().unwrap().last_mut().unwrap() = TextSignal {
                            text: format!("{}", val),
                            signal: d.into(),
                        };
                        r
                    }
                    SignalValue::Uninitialized => ui.label("Uninitialized"),
                    SignalValue::DontCare => ui.label("DontCare"),
                    SignalValue::Unknown => ui.label("Unknown"),
                };
                r.on_hover_text(format!(
                    "{:?}",
                    parse_signal(
                        self.edit_history
                            .read()
                            .unwrap()
                            .last()
                            .unwrap()
                            .text
                            .as_str()
                    )
                ));
            });

        let r = rect_with_hover(
            area.response.rect,
            clip_rect,
            editor_mode,
            ui,
            self.id.clone(),
            |ui| {
                ui.label(format!("Id: {}", self.id.clone()));
                ui.label(
                    self.edit_history
                        .read()
                        .unwrap()
                        .last()
                        .unwrap()
                        .text
                        .to_string(),
                );
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
        grid: &GridOptions,
        editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        let r_vec = ProbeEdit::render(
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
        let delete = drag_logic(
            ui.ctx(),
            resp,
            &mut self.pos,
            &mut context.pos_tmp,
            scale,
            offset,
            grid,
        );

        properties_window(
            ui,
            self.id.clone(),
            resp,
            &mut context.properties_window,
            |ui| {
                let clicked_dropdown = false;
                input_change_id(ui, &mut context.id_tmp, &mut self.id, id_ports);
                pos_drag_value(ui, &mut self.pos);
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
            crate::components::PROBE_EDIT_OUT_ID.to_string(),
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

    fn get_pos(&self) -> (f32, f32) {
        self.pos
    }
}

fn parse_signal(text: &str) -> SignalValue {
    let text = text.trim();

    if let Ok(signal) = text.parse::<SignalSigned>() {
        (signal as SignalUnsigned).into()
    } else if let Ok(signal) = text.parse::<SignalUnsigned>() {
        (signal as SignalUnsigned).into()
    } else if let Some(hex) = text.strip_prefix("0x") {
        if let Ok(signal) = SignalUnsigned::from_str_radix(hex, 16) {
            signal.into()
        } else {
            SignalValue::Unknown
        }
    } else {
        SignalValue::Unknown
    }
}
