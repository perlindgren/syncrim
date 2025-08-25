use crate::components::{
    InstrSplit, INSTRUCTION_SPLITTER_FUNCT_ID, INSTRUCTION_SPLITTER_IMMEDIATE_ID,
    INSTRUCTION_SPLITTER_OP_ID, INSTRUCTION_SPLITTER_RD_ID, INSTRUCTION_SPLITTER_RS_ID,
    INSTRUCTION_SPLITTER_RT_ID, INSTRUCTION_SPLITTER_SHAMT_ID, INSTRUCTION_SPLITTER_TARGET_ID,
};
use egui::{pos2, Pos2, Rect, Response, RichText, Ui, Vec2};
use syncrim::common::{EguiComponent, Id, Input, Ports, Simulator};
use syncrim::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use syncrim::gui_egui::gui::EguiExtra;
use syncrim::gui_egui::helper::{basic_component_gui, basic_editor_popup};

const WIDTH: f32 = 50.0;
const HEIGHT: f32 = 200.0;

#[typetag::serde]
impl EguiComponent for InstrSplit {
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
        // size of the component
        let w = WIDTH * scale;
        let h: f32 = HEIGHT * scale;
        basic_component_gui(self, &simulator, ui.ctx(), offset, scale, clip_rect, |ui| {
            ui.set_height(h);
            ui.set_width(w);
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("instruction\n splitter").size(12f32 * scale));
            });
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
        let res = self
            .render(
                ui,
                context,
                simulator,
                offset,
                scale,
                clip_rect,
                editor_mode,
            )
            .unwrap()
            .remove(0);
        basic_editor_popup(self, ui, context, id_ports, res, |_| {})
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
        // width 50
        // height 200
        let own_pos = Vec2::new(self.pos.0, self.pos.1);
        const M: f32 = 6.0;
        fn out_pos(i: u32) -> Pos2 {
            pos2(
                WIDTH / 2.0 + M,
                -HEIGHT / 2.0 + (i as f32 + 1.0) * HEIGHT / 9.0,
            )
        }
        vec![
            (
                crate::components::INSTRUCTION_SPLITTER_IN_ID.to_string(),
                Pos2::new(-WIDTH / 2.0 - M, 0.0) + own_pos,
            ),
            (
                crate::components::INSTRUCTION_SPLITTER_SHAMT_ID.to_string(),
                out_pos(0) + own_pos,
            ),
            (
                crate::components::INSTRUCTION_SPLITTER_OP_ID.to_string(),
                out_pos(1) + own_pos,
            ),
            (
                crate::components::INSTRUCTION_SPLITTER_FUNCT_ID.to_string(),
                out_pos(2) + own_pos,
            ),
            (
                crate::components::INSTRUCTION_SPLITTER_RS_ID.to_string(),
                out_pos(3) + own_pos,
            ),
            (
                crate::components::INSTRUCTION_SPLITTER_RT_ID.to_string(),
                out_pos(4) + own_pos,
            ),
            (
                crate::components::INSTRUCTION_SPLITTER_IMMEDIATE_ID.to_string(),
                out_pos(5) + own_pos,
            ),
            (
                crate::components::INSTRUCTION_SPLITTER_RD_ID.to_string(),
                out_pos(6) + own_pos,
            ),
            (
                crate::components::INSTRUCTION_SPLITTER_TARGET_ID.to_string(),
                out_pos(7) + own_pos,
            ),
        ]
    }

    fn get_input_location(&self, id: Input) -> Option<(f32, f32)> {
        let loc = self
            .ports_location()
            .iter()
            .map(|(_, loc)| <(f32, f32)>::from(loc))
            .collect::<Vec<(f32, f32)>>();

        if id == self.instruction_in {
            Some(loc[0])
        } else if id == Input::new(&self.id, INSTRUCTION_SPLITTER_SHAMT_ID) {
            Some(loc[1])
        } else if id == Input::new(&self.id, INSTRUCTION_SPLITTER_OP_ID) {
            Some(loc[2])
        } else if id == Input::new(&self.id, INSTRUCTION_SPLITTER_FUNCT_ID) {
            Some(loc[3])
        } else if id == Input::new(&self.id, INSTRUCTION_SPLITTER_RS_ID) {
            Some(loc[4])
        } else if id == Input::new(&self.id, INSTRUCTION_SPLITTER_RT_ID) {
            Some(loc[5])
        } else if id == Input::new(&self.id, INSTRUCTION_SPLITTER_IMMEDIATE_ID) {
            Some(loc[6])
        } else if id == Input::new(&self.id, INSTRUCTION_SPLITTER_RD_ID) {
            Some(loc[7])
        } else if id == Input::new(&self.id, INSTRUCTION_SPLITTER_TARGET_ID) {
            Some(loc[8])
        } else {
            None
        }
    }
}
