use crate::components::{MipsMmu, MMU_ADDRESS_IN_ID, MMU_IO_REG_SEL_OUT, MMU_IO_RE_OUT, MMU_IO_WE_OUT, MMU_MEM_ADDRESS_OUT_ID, MMU_MEM_RE_OUT, MMU_MEM_WE_OUT, MMU_READ_ENABLE_IN, MMU_TIMER_ADDRESS_OUT, MMU_TIMER_RE_OUT, MMU_TIMER_WE_OUT, MMU_WRITE_ENABLE_IN};
use egui::{pos2, Pos2, Rect, Response, RichText, Ui, Vec2};
use syncrim::common::{EguiComponent, Id, Input, Ports, Simulator};
use syncrim::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use syncrim::gui_egui::gui::EguiExtra;
use syncrim::gui_egui::helper::basic_component_gui;

const WIDTH: f32 = 105.0;
const HEIGHT: f32 = 30.0;

#[typetag::serde]
impl EguiComponent for MipsMmu {
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
            ui.label(RichText::new("mips mmu").size(12f32 * scale));
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
        let own_pos = Vec2::new(self.pos.0, self.pos.1);
        
        // helper function to place the output ports
        fn out_pos(i: u32) -> Pos2 {
            pos2(
                -WIDTH / 2.0 + (i as f32 + 1.0) * WIDTH / 12.0, // there are twelve component ports, re, we, adrs for 4 components, 3*4=12
                HEIGHT / 2.0 + M,
            )
        }
        // margin of component
        const M: f32 = 6.0;
        vec![
            (
                MMU_ADDRESS_IN_ID.to_string(),
                pos2(- WIDTH / 3.0, -HEIGHT-M) + own_pos,
            ),
            (
                MMU_READ_ENABLE_IN.to_string(),
                pos2(0.0, 0.0-HEIGHT - M) + own_pos,
            ),
            (
                MMU_WRITE_ENABLE_IN.to_string(),
                pos2(WIDTH / 3.0, -HEIGHT+WIDTH) + own_pos,
            ),
            // TODO CP0 ports
            (
                MMU_MEM_ADDRESS_OUT_ID.to_string(),
                out_pos(3) + own_pos,
            ),
            (
                MMU_MEM_RE_OUT.to_string(),
                out_pos(4) + own_pos,
            ),
            (
                MMU_MEM_WE_OUT.to_string(),
                out_pos(5) + own_pos,
            ),
            (
                MMU_TIMER_ADDRESS_OUT.to_string(),
                out_pos(6) + own_pos,
            ),
            (
                MMU_TIMER_RE_OUT.to_string(),
                out_pos(7) + own_pos,
            ),
            (
                MMU_TIMER_WE_OUT.to_string(),
                out_pos(8) + own_pos,
            ),
            (
                MMU_IO_REG_SEL_OUT.to_string(),
                out_pos(9) + own_pos,
            ),
            (
                MMU_IO_RE_OUT.to_string(),
                out_pos(10) + own_pos,
            ),
            (
                MMU_IO_WE_OUT.to_string(),
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

        if id == self.address_in {
            Some(loc[0])
        } else if id == self.re_in {
            Some(loc[1])
        } else if id == self.we_in {
            Some(loc[2])
        // TODO CP0
        } else if id == Input::new(&self.id, MMU_MEM_ADDRESS_OUT_ID) {
            Some(loc[3])
        } else if id == Input::new(&self.id, MMU_MEM_RE_OUT) {
            Some(loc[4])
        } else if id == Input::new(&self.id, MMU_MEM_WE_OUT) {
            Some(loc[5])
        } else if id == Input::new(&self.id, MMU_TIMER_ADDRESS_OUT) {
            Some(loc[6])
        } else if id == Input::new(&self.id, MMU_TIMER_RE_OUT) {
            Some(loc[7])
        } else if id == Input::new(&self.id, MMU_TIMER_WE_OUT) {
            Some(loc[8])
        } else if id == Input::new(&self.id, MMU_IO_REG_SEL_OUT) {
            Some(loc[9])
        } else if id == Input::new(&self.id, MMU_IO_RE_OUT) {
            Some(loc[10])
        } else if id == Input::new(&self.id, MMU_IO_WE_OUT) {
            Some(loc[11])
        } else {
            None
        }
    }
}
