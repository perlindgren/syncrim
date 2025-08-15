use std::path::PathBuf;

use crate::components::InstrMem;
use crate::components::PhysicalMem;
use crate::components::RegFile;
use crate::components::INSTR_MEM_INSTRUCTION_ID;
use crate::helpers::find_component_with_type;
use egui::pos2;
use egui::Pos2;
use egui::{Rect, Response, RichText, Ui, Vec2};
use syncrim::common::Input;
use syncrim::common::{EguiComponent, Id, Ports, Simulator};
use syncrim::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use syncrim::gui_egui::gui::EguiExtra;
use syncrim::gui_egui::helper::basic_component_gui;

const WIDTH: f32 = 120.0;
const HEIGHT: f32 = 55.0;

impl InstrMem {
    fn update_mem_view_register_values(&self, sim: &Simulator) {
        #[allow(clippy::expect_fun_call)]
        let reg: &RegFile = find_component_with_type(sim, &self.regfile_id)
            .expect(&format!("can't find {} with type Regfile", self.regfile_id));
        self.mem_view
            .borrow_mut()
            .set_reg_values(*reg.registers.borrow());
    }
}

#[typetag::serde]
impl EguiComponent for InstrMem {
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
        let mut path_option: Option<PathBuf> = None;
        let mut mem_view_vis: bool = self.mem_view.borrow().visible;

        let r = basic_component_gui(self, &simulator, ui.ctx(), offset, scale, clip_rect, |ui| {
            // ui.centered_and_justified(|ui| {
            ui.set_height(HEIGHT * scale);
            ui.set_width(WIDTH * scale);
            ui.label(RichText::new("Instruction memory").size(12f32 * scale));
            if ui.button("load file").clicked() {
                path_option = rfd::FileDialog::new().pick_file();
            };

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

        // handle mem_window and load of new file
        if let Some(sim) = &simulator {
            self.update_mem_view_register_values(sim);
            self.mem_view
                .borrow_mut()
                .set_all_dynamic_symbols(self.dynamic_symbols.borrow().clone());
            #[allow(clippy::expect_fun_call)]
            let phys_mem: &PhysicalMem = find_component_with_type(sim, &self.phys_mem_id).expect(
                &format!("can't find {} with type PhysicalMem", self.regfile_id),
            );

            if let Some(path) = &path_option {
                phys_mem.load_file(path);
                mem_view_vis = true;
            };

            let mut mem_view = self.mem_view.borrow_mut();
            mem_view.visible = mem_view_vis;
            mem_view.render(ui.ctx(), &phys_mem.mem.borrow());
        }

        // this is done after handle mem_window, because our simulator need to be returned
        if let Some(sim) = simulator {
            if path_option.is_some() {
                sim.reset();
            }
        }

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

    fn ports_location(&self) -> Vec<(syncrim::common::Id, Pos2)> {
        // width 50
        // height 200
        let own_pos = Vec2::new(self.pos.0, self.pos.1);
        const M: f32 = 6.0;
        vec![
            (
                crate::components::INSTR_MEM_PC_ID.to_string(),
                pos2(-WIDTH / 3.0, -HEIGHT / 2.0 - M) + own_pos,
            ),
            (
                crate::components::DATA_MEM_A_IN_ID.to_string(),
                pos2(WIDTH / 3.0, -HEIGHT / 2.0 - M) + own_pos,
            ),
        ]
    }

    fn get_input_location(&self, id: Input) -> Option<(f32, f32)> {
        let loc = self
            .ports_location()
            .iter()
            .map(|(_, loc)| <(f32, f32)>::from(loc))
            .collect::<Vec<(f32, f32)>>();

        if id == self.pc {
            Some(loc[0])
        } else if id == Input::new(&self.id, INSTR_MEM_INSTRUCTION_ID) {
            Some(loc[1])
        } else {
            None
        }
    }
}
