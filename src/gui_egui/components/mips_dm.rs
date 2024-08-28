use crate::common::{EguiComponent, Id, Ports, Simulator};
use crate::components::DataMem;
use crate::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use crate::gui_egui::gui::EguiExtra;
use crate::gui_egui::helper::basic_component_gui;
use egui::{Rect, Response, RichText, Ui, Vec2};

#[typetag::serde]
impl EguiComponent for DataMem {
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
        // we could avoid this if we clone self in our basic_component_ui.
        // but instead we let our Closure save stuff here (let path_option, mem_view_vis)
        // and apply our changes when basic_component_gui returns our borrow
        // this is to avoid cloning all the fields.
        // avoiding to clone the fields might be premature optimization
        // as instrMem.mem is a reference count and wont actually clone the underlying btree and hashmaps
        //
        // we save 27 bytes of clone
        // and most of that clone might even be optimized away
        // yes this was premature optimization
        let mut mem_view_vis: bool = self.mem_view.borrow().visible;

        let r = basic_component_gui(self, &simulator, ui.ctx(), offset, scale, clip_rect, |ui| {
            // ui.centered_and_justified(|ui| {
            ui.label(RichText::new("Data memory").size(12f32 * scale));
            ui.button(
                RichText::new("load file")
                    .size(12f32 * scale)
                    .strikethrough(),
            )
            .on_hover_text("Use instruction memory to load a file");

            match mem_view_vis {
                false => {
                    if ui.button("Show mem window").clicked() {
                        mem_view_vis = true;
                    }
                }
                true => {
                    ui.toggle_value(&mut mem_view_vis, "Hide mem window");
                }
            };
            // });
        });
        let mut mem_view = self.mem_view.borrow_mut();
        mem_view.visible = mem_view_vis;
        mem_view.render(ui.ctx());
        // return response from basic component gui
        r
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
