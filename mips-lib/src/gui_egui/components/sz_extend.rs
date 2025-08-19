use crate::components::{SignZeroExtend, SIGNZEROEXTEND_OUT_ID};
use egui::{pos2, Pos2, Rect, Response, RichText, Ui, Vec2};
use syncrim::common::{EguiComponent, Id, Input, Ports, SignalValue, Simulator};
use syncrim::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use syncrim::gui_egui::gui::EguiExtra;
use syncrim::gui_egui::helper::{basic_component_gui, basic_editor_popup};

const WIDTH: f32 = 105.0;
const HEIGHT: f32 = 30.0;

#[typetag::serde]
impl EguiComponent for SignZeroExtend {
    fn render(
        &self,
        ui: &mut Ui,
        _context: &mut EguiExtra,
        simulator: Option<&mut Simulator>,
        offset: Vec2,
        scale: f32,
        clip_rect: Rect,
        _editor_mode: EditorMode,
    ) -> Option<Vec<Response>> {
        basic_component_gui(self, &simulator, ui.ctx(), offset, scale, clip_rect, |ui| {
            ui.set_height(HEIGHT * scale);
            ui.set_width(WIDTH * scale);
            match &simulator {
                Some(sim) => {
                    ui.label(match sim.get_input_value(&self.signzero_ctrl_in) {
                        SignalValue::Uninitialized => {
                            " Sign/Zero extend:\nUninitialized cntr".to_string()
                        }
                        SignalValue::Unknown => "Sign/Zero extend:\nextendUnknown".to_string(),
                        SignalValue::DontCare => "Sign/Zero extend:\nDon't Care".to_string(),
                        SignalValue::Data(v) => match v {
                            0 => "Sign/Zero extend:\nZero",
                            1 => "Sign/Zero extend:\nSign",
                            _ => "Sign/Zero extend:\nInvalid cntr",
                        }
                        .to_string(),
                    });
                }

                None => {
                    ui.label(RichText::new("Signal Extender:\nNo Sim").size(12f32 * scale));
                }
            }
        })
    }

    fn render_editor(
        &mut self,
        ui: &mut egui::Ui,
        context: &mut EguiExtra,
        simulator: Option<&mut Simulator>,
        offset: egui::Vec2,
        scale: f32,
        clip_rect: egui::Rect,
        id_ports: &[(Id, Ports)],
        _grid: &GridOptions,
        editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        let res = self.render(
            ui,
            context,
            simulator,
            offset,
            scale,
            clip_rect,
            editor_mode,
        ).unwrap().remove(0);
        basic_editor_popup(self, ui, context, id_ports, res, |_|{})

    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }

    fn get_pos(&self) -> (f32, f32) {
        self.pos
    }

    fn top_padding(&self) -> f32 {
        20f32
    }

    fn ports_location(&self) -> Vec<(syncrim::common::Id, Pos2)> {
        let own_pos = Vec2::new(self.pos.0, self.pos.1);
        const M: f32 = 6.0;
        vec![
            (
                crate::components::SIGNZEROEXTEND_SIGNAL_IN_ID.to_string(),
                pos2(-WIDTH / 2.0 - M, 0.0) + own_pos,
            ),
            (
                crate::components::SIGNZEROEXTEND_CTRL_IN_ID.to_string(),
                pos2(0.0, -HEIGHT / 2.0 - M) + own_pos,
            ),
            (
                crate::components::SIGNZEROEXTEND_OUT_ID.to_string(),
                pos2(WIDTH / 2.0 + M, 0.0) + own_pos,
            ),
        ]
    }

    fn get_input_location(&self, id: Input) -> Option<(f32, f32)> {
        let loc = self
            .ports_location()
            .iter()
            .map(|(_, loc)| <(f32, f32)>::from(loc))
            .collect::<Vec<(f32, f32)>>();

        if id == self.signzero_signal_in {
            Some(loc[0])
        } else if id == self.signzero_ctrl_in {
            Some(loc[1])
        } else if id == Input::new(&self.id, SIGNZEROEXTEND_OUT_ID) {
            Some(loc[2])
        } else {
            None
        }
    }
}
