use crate::components::{cntr_field, ControlUnit};
use egui::{pos2, Pos2, Rect, Response, Ui, Vec2};
use syncrim::common::{EguiComponent, Id, Input, Ports, Simulator};
use syncrim::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use syncrim::gui_egui::gui::EguiExtra;
use syncrim::gui_egui::helper::basic_component_gui;

const WIDTH: f32 = 400.0;
const HEIGHT: f32 = 15.0;

#[typetag::serde]
impl EguiComponent for ControlUnit {
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
                ui.label("Control Unit");
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

    fn ports_location(&self) -> Vec<(syncrim::common::Id, Pos2)> {
        // width 50
        // height 200
        let own_pos = Vec2::new(self.pos.0, self.pos.1);
        const M: f32 = 6.0;
        fn out_pos(i: u32) -> Pos2 {
            pos2(
                -WIDTH / 2.0 + (i as f32 + 1.0) * WIDTH / 12.0,
                HEIGHT / 2.0 + M,
            )
        }
        vec![
            (
                crate::components::cntr_field::INSTR_IN.to_string(),
                Pos2::new(-WIDTH / 2.0 - M, 0.0) + own_pos,
            ),
            (
                crate::components::cntr_field::EXTEND_SELECT_OUT.to_string(),
                out_pos(0) + own_pos,
            ),
            (
                crate::components::cntr_field::BRANCH_INTERRUPT_OUT.to_string(),
                out_pos(1) + own_pos,
            ),
            (
                crate::components::cntr_field::REG_DEST_OUT.to_string(),
                out_pos(2) + own_pos,
            ),
            (
                crate::components::cntr_field::ALU_SRC_A_OUT.to_string(),
                out_pos(3) + own_pos,
            ),
            (
                crate::components::cntr_field::ALU_SRC_B_OUT.to_string(),
                out_pos(4) + own_pos,
            ),
            (
                crate::components::cntr_field::MEM_WRITE_ENABLE_OUT.to_string(),
                out_pos(5) + own_pos,
            ),
            (
                crate::components::cntr_field::MEM_MODE_OUT.to_string(),
                out_pos(6) + own_pos,
            ),
            (
                crate::components::cntr_field::MMU_OUT.to_string(),
                out_pos(7) + own_pos,
            ),
            (
                crate::components::cntr_field::ALU_OP_OUT.to_string(),
                out_pos(8) + own_pos,
            ),
            (
                crate::components::cntr_field::REG_WRITE_SRC_OUT.to_string(),
                out_pos(9) + own_pos,
            ),
            (
                crate::components::cntr_field::REG_WRITE_ENABLE_OUT.to_string(),
                out_pos(10) + own_pos,
            ),
            (
                crate::components::cntr_field::CP0_OUT.to_string(),
                out_pos(11) + own_pos,
            ),
        ]
    }

    fn get_input_location(&self, id: Input) -> Option<(f32, f32)> {
        let loc = self
            .ports_location()
            .iter()
            .map(|(_, loc)| <(f32, f32)>::from(loc))
            .collect::<Vec<(f32, f32)>>();

        if id == self.a_in {
            Some(loc[0])
        } else if id == Input::new(&self.id, cntr_field::EXTEND_SELECT_OUT) {
            Some(loc[1])
        } else if id == Input::new(&self.id, cntr_field::BRANCH_INTERRUPT_OUT) {
            Some(loc[2])
        } else if id == Input::new(&self.id, cntr_field::REG_DEST_OUT) {
            Some(loc[3])
        } else if id == Input::new(&self.id, cntr_field::ALU_SRC_A_OUT) {
            Some(loc[4])
        } else if id == Input::new(&self.id, cntr_field::ALU_SRC_B_OUT) {
            Some(loc[5])
        } else if id == Input::new(&self.id, cntr_field::MEM_WRITE_ENABLE_OUT) {
            Some(loc[6])
        } else if id == Input::new(&self.id, cntr_field::MEM_MODE_OUT) {
            Some(loc[7])
        } else if id == Input::new(&self.id, cntr_field::MMU_OUT) {
            Some(loc[8])
        } else if id == Input::new(&self.id, cntr_field::ALU_OP_OUT) {
            Some(loc[9])
        } else if id == Input::new(&self.id, cntr_field::REG_WRITE_SRC_OUT) {
            Some(loc[10])
        } else if id == Input::new(&self.id, cntr_field::REG_WRITE_ENABLE_OUT) {
            Some(loc[11])
        } else if id == Input::new(&self.id, cntr_field::CP0_OUT) {
            Some(loc[12])
        } else {
            None
        }
    }
}
