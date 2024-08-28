use crate::common::{EguiComponent, Id, Ports, SignalValue, Simulator};
use crate::components::SignZeroExtend;
use crate::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use crate::gui_egui::gui::EguiExtra;
use crate::gui_egui::helper::basic_component_gui;
use egui::{Rect, Response, RichText, Ui, Vec2};

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
        _id_ports: &[(Id, Ports)],
        _grid: &GridOptions,
        editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        self.render(
            ui,
            context,
            simulator,
            offset,
            scale,
            clip_rect,
            editor_mode,
        );
        EditorRenderReturn {
            delete: false,
            resp: None,
        }
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
}
