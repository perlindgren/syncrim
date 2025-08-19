use crate::components::CP0;
use egui::{pos2, Pos2, Rect, Response, Ui, Vec2};
use syncrim::common::{EguiComponent, Id, Input, Ports, Simulator};
use syncrim::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use syncrim::gui_egui::gui::EguiExtra;
use syncrim::gui_egui::helper::{basic_component_gui, basic_editor_popup};

const WIDTH: f32 = 70.0;
const HEIGHT: f32 = 45.0;

#[typetag::serde]
impl EguiComponent for CP0 {
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
            ui.label("CP0");
            ui.label(format!(
                "SR: {:#010x} \nECR: {:#010x} \nEPC: {:#010x}",
                self.registers.borrow()[12],
                self.registers.borrow()[13],
                self.registers.borrow()[14]
            ));
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
                crate::components::CP0_WRITE_ENABLE_IN_ID.to_string(),
                pos2(-WIDTH / 2.0 - M, -4.0) + own_pos,
            ),
            (
                crate::components::CP0_REGISTER_ADDRESS_IN.to_string(),
                pos2(-WIDTH / 2.0 - M, 4.0) + own_pos,
            ),
            (
                crate::components::CP0_DATA_IN.to_string(),
                pos2(0.0, -HEIGHT / 2.0 - M) + own_pos,
            ),
            (
                crate::components::CP0_RFE_IN_ID.to_string(),
                pos2(WIDTH / 2.0 + M, -4.0) + own_pos,
            ),
            (
                crate::components::CP0_TIMER_INTERRUPT_IN_ID.to_string(),
                pos2(WIDTH / 2.0 + M, 4.0) + own_pos,
            ),
            (
                crate::components::CP0_IO_INTERRUPT_IN_ID.to_string(),
                pos2(WIDTH / 2.0 + M, 4.0) + own_pos,
            ),
            (
                crate::components::CP0_SYSCALL_IN_ID.to_string(),
                pos2(WIDTH / 2.0 + M, 4.0) + own_pos,
            ),
            (
                crate::components::CP0_INSTRUCTIO_ADDRESS_IN.to_string(),
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

        if id == self.write_enable {
            Some(loc[0])
        } else if id == self.register_address_in {
            Some(loc[1])
        } else if id == self.data_in {
            Some(loc[2])
        } else if id == self.rfe_in {
            Some(loc[3])
        } else if id == self.timer_interrupt_in {
            Some(loc[4])
        } else if id == self.io_interrupt_in {
            Some(loc[5])
        } else if id == self.syscall_in {
            Some(loc[6])
        } else if id == self.instruction_address_in {
            Some(loc[7])
        } else {
            None
        }
    }
}
