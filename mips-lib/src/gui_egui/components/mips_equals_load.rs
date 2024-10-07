use crate::components::{
    EqualLoad, EQUAL_LOAD_A_IN_ID, EQUAL_LOAD_B_IN_ID, EQUAL_LOAD_LOAD_IN_ID, EQUAL_LOAD_OUT_ID,
    EQUAL_LOAD_WE_IN_ID,
};
use egui::{pos2, Rect, Response, Ui, Vec2};
use syncrim::common::{EguiComponent, Id, Input, Ports, Simulator};
use syncrim::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use syncrim::gui_egui::gui::EguiExtra;
use syncrim::gui_egui::helper::basic_component_gui;

#[typetag::serde]
impl EguiComponent for EqualLoad {
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
            ui.label("Equal_Load");
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

    fn get_input_location(&self, id: Input) -> Option<(f32, f32)> {
        let loc = self
            .ports_location()
            .iter()
            .map(|(_, loc)| <(f32, f32)>::from(loc))
            .collect::<Vec<(f32, f32)>>();
        if id == self.a_in {
            Some(loc[0])
        } else if id == self.b_in {
            Some(loc[1])
        } else if id == self.we_in {
            Some(loc[2])
        } else if id == self.load_in {
            Some(loc[3])
        } else if id == Input::new(&self.id, EQUAL_LOAD_OUT_ID) {
            Some(loc[4])
        } else {
            None
        }
    }

    fn ports_location(&self) -> Vec<(Id, egui::Pos2)> {
        //size 22-14
        let m = 6f32; // margin
        let pos: Vec2 = self.pos.into();
        vec![
            (EQUAL_LOAD_A_IN_ID.to_string(), pos2(-11.0 - m, -10.0) + pos),
            (EQUAL_LOAD_B_IN_ID.to_string(), pos2(-11.0 - m, 10.0) + pos),
            (EQUAL_LOAD_WE_IN_ID.to_string(), pos2(-11.0 - m, 2.5) + pos),
            (
                EQUAL_LOAD_LOAD_IN_ID.to_string(),
                pos2(-11.0 - m, 7.5) + pos,
            ),
            (EQUAL_LOAD_OUT_ID.to_string(), pos2(11.0 + m, 0.0) + pos),
        ]
    }
}
