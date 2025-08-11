use crate::components::{
    MipsIO, MipsTimer, IO_DATA_OUT_ID, IO_INTERRUPT_OUT_ID, TIMER_DATA_OUT_ID,
    TIMER_INTERRUPT_OUT_ID,
};
use egui::{
    pos2, Event, Pos2, ProgressBar, Rect, Response, RichText, Ui, Vec2, ViewportBuilder, ViewportId,
};
use syncrim::common::{EguiComponent, Id, Input, Ports, Simulator};
use syncrim::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use syncrim::gui_egui::gui::EguiExtra;
use syncrim::gui_egui::helper::{basic_component_gui_with_on_hover, basic_on_hover};

const WIDTH: f32 = 70.0;
const HEIGHT: f32 = 45.0;

#[typetag::serde]
impl EguiComponent for MipsIO {
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
                // ===================================================
                //
                //               WARNING UGLY CODE AHEAD
                //
                // ===================================================
                //
                // this works for now, but will require change close to a complete
                // rewrite if any more then minimum functionally is required 
                ui.label("IO component");
                ui.toggle_value(&mut self.gui_show.borrow_mut(), "show io window");
                if *self.gui_show.borrow() {
                    ui.ctx().show_viewport_immediate(
                        ViewportId::from_hash_of(&self.id),
                        ViewportBuilder::default().with_title("IO component"),
                        |ctx, _class| {
                            egui::CentralPanel::default().show(ctx, |ui| {
                                // TODO care about carriage return, backspace and other advance utf8/ascii codes
                                ui.label(String::from_utf8_lossy(&self.data.borrow().out_buff));
                                // TODO this is ugly, get all keypresses 
                                for key in ui.input(|ev| {
                                    ev.events
                                        .iter()
                                        .filter_map(|e| match e {
                                            Event::Key { key, .. } => Some(key.clone()),
                                            _ => None,
                                        })
                                        .collect::<Vec<_>>()
                                }) {
                                    // TODO actually write to input_buffer
                                    println!("{:?}", key.symbol_or_name())
                                }
                            })
                        },
                    );
                }
            },
            |ui| basic_on_hover(ui, self, &simulator),
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
        // margin of component
        const M: f32 = 6.0;
        vec![
            (
                crate::components::IO_ADDRESS_IN_ID.to_string(),
                pos2(-WIDTH / 2.0 - M, -4.0) + own_pos,
            ),
            (
                crate::components::IO_DATA_IN_ID.to_string(),
                pos2(-WIDTH / 2.0 - M, 4.0) + own_pos,
            ),
            (
                crate::components::IO_WRITE_ENABLE_IN.to_string(),
                pos2(-4.0, -HEIGHT / 2.0 - M) + own_pos,
            ),
            (
                crate::components::IO_READ_ENABLE_IN.to_string(),
                pos2(4.0, -HEIGHT / 2.0 - M) + own_pos,
            ),
            (
                crate::components::IO_DATA_OUT_ID.to_string(),
                pos2(WIDTH / 2.0 + M, -4.0) + own_pos,
            ),
            (
                crate::components::IO_INTERRUPT_OUT_ID.to_string(),
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
        } else if id == self.re_in {
            Some(loc[2])
        } else if id == Input::new(&self.id, IO_DATA_OUT_ID) {
            Some(loc[3])
        } else if id == Input::new(&self.id, IO_INTERRUPT_OUT_ID) {
            Some(loc[4])
        } else {
            None
        }
    }
}
