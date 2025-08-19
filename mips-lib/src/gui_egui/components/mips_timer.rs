use crate::components::{MipsTimer, TIMER_DATA_OUT_ID, TIMER_INTERRUPT_OUT_ID};
use egui::{pos2, Pos2, ProgressBar, Rect, Response, RichText, Ui, Vec2};
use syncrim::common::{EguiComponent, Id, Input, Ports, Simulator};
use syncrim::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use syncrim::gui_egui::gui::EguiExtra;
use syncrim::gui_egui::helper::{basic_component_gui_with_on_hover, basic_editor_popup, basic_on_hover};

const WIDTH: f32 = 70.0;
const HEIGHT: f32 = 45.0;

#[typetag::serde]
impl EguiComponent for MipsTimer {
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
        basic_component_gui_with_on_hover(
            self,
            ui.ctx(),
            offset,
            scale,
            clip_rect,
            |ui| {
                ui.set_height(HEIGHT * scale);
                ui.set_width(WIDTH * scale);
                ui.label(RichText::new("Timer").size(12f32 * scale));
                let data = self.data.borrow();
                ui.add(
                    ProgressBar::new(data.div_counter as f32 / data.divider as f32)
                        .desired_height(4.0 * scale)
                        .desired_width(WIDTH * scale),
                );
                ui.add(
                    ProgressBar::new(data.counter as f32 / data.compare as f32)
                        .text(format!("{}/{}", data.counter, data.compare))
                        .corner_radius(5.0 * scale)
                        .desired_width(WIDTH * scale),
                );
            },
            |ui| {
                let data = self.data.borrow();
                ui.label(format!("Flags {:#08b}\nCounter {:#010x}\nCompare {:#010x}\nDivider {:#010x}\nDiv counter {:#010x}",
                    data.flags, data.counter, data.compare, data.divider, data.div_counter
                ));
                ui.separator();
                basic_on_hover(ui, self, &simulator);
            },
        )
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
        ).unwrap().remove(0); // no panic since we know basic_component_gui returns Some([area_response])
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
        // margin of component
        const M: f32 = 6.0;
        vec![
            (
                crate::components::TIMER_ADDRESS_IN_ID.to_string(),
                pos2(-WIDTH / 2.0 - M, -4.0) + own_pos,
            ),
            (
                crate::components::TIMER_DATA_IN_ID.to_string(),
                pos2(-WIDTH / 2.0 - M, 4.0) + own_pos,
            ),
            (
                crate::components::TIMER_WRITE_ENABLE.to_string(),
                pos2(0.0, -HEIGHT / 2.0 - M) + own_pos,
            ),
            (
                crate::components::TIMER_DATA_OUT_ID.to_string(),
                pos2(WIDTH / 2.0 + M, -4.0) + own_pos,
            ),
            (
                crate::components::TIMER_INTERRUPT_OUT_ID.to_string(),
                pos2(WIDTH / 2.0 + M, 4.0) + own_pos,
            ),
        ]
    }

    fn get_input_location(&self, id: Input) -> Option<(f32, f32)> {
        let loc = self
            .ports_location()
            .iter()
            .map(|(_, loc)| <(f32, f32)>::from(loc))
            .collect::<Vec<(f32, f32)>>();

        if id == self.address_in {
            Some(loc[0])
        } else if id == self.data_in {
            Some(loc[1])
        } else if id == self.we_in {
            Some(loc[2])
        } else if id == Input::new(&self.id, TIMER_DATA_OUT_ID) {
            Some(loc[3])
        } else if id == Input::new(&self.id, TIMER_INTERRUPT_OUT_ID) {
            Some(loc[4])
        } else {
            None
        }
    }
}
