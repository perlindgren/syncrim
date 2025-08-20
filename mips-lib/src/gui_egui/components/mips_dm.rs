use crate::components::{DataMem, PhysicalMem, RegFile, DATA_MEM_READ_DATA_OUT_ID};
use egui::{pos2, Pos2, Rect, Response, RichText, Ui, Vec2};
use syncrim::common::{EguiComponent, Id, Input, Ports, Simulator};
use syncrim::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use syncrim::gui_egui::gui::EguiExtra;
use syncrim::gui_egui::helper::{basic_component_gui, basic_editor_popup};

const WIDTH: f32 = 120.0;
const HEIGHT: f32 = 55.0;

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
            ui.set_height(HEIGHT * scale);
            ui.set_width(WIDTH * scale);
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

        if let Some(sim) = &simulator {
            let v = &sim.ordered_components;
            #[allow(clippy::expect_fun_call)]
            let comp = v
                .iter()
                .find(|x| x.get_id_ports().0 == self.regfile_id)
                .expect(&format!("cant find {} in simulator", self.regfile_id));
            // deref to get &dyn EguiComponent
            let comp_any = (*comp).as_any();
            let regfile: &RegFile = comp_any
                .downcast_ref()
                .expect("can't downcast to physical memory");
            self.mem_view
                .borrow_mut()
                .set_reg_values(*regfile.registers.borrow());
        }

        if let Some(sim) = &simulator {
            let v = &sim.ordered_components;
            #[allow(clippy::expect_fun_call)]
            let comp = v
                .iter()
                .find(|x| x.get_id_ports().0 == self.phys_mem_id)
                .expect(&format!("cant find {} in simulator", self.phys_mem_id));
            // deref to get &dyn EguiComponent
            let comp_any = (*comp).as_any();
            let phys_mem: &PhysicalMem = comp_any
                .downcast_ref()
                .expect("can't downcast to physical memory");
            // {} to drop RefMut as early as possible
            {
                let mut mem_view = self.mem_view.borrow_mut();
                mem_view.visible = mem_view_vis;
                mem_view.render(ui.ctx(), &phys_mem.mem.borrow());
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
                -WIDTH / 2.0 + (i as f32 + 1.0) * WIDTH / 4.0,
                -HEIGHT / 2.0 - M,
            )
        }
        vec![
            (
                crate::components::DATA_MEM_WD_IN_ID.to_string(),
                pos2(-WIDTH / 2.0 - M, 0.0) + own_pos,
            ),
            (
                crate::components::DATA_MEM_A_IN_ID.to_string(),
                out_pos(0) + own_pos,
            ),
            (
                crate::components::DATA_MEM_WRITE_ENABLE_ID.to_string(),
                out_pos(1) + own_pos,
            ),
            (
                crate::components::DATA_MEM_OP_IN_ID.to_string(),
                out_pos(2) + own_pos,
            ),
            (
                crate::components::DATA_MEM_READ_DATA_OUT_ID.to_string(),
                pos2(WIDTH / 2.0 + M, 0.0) + own_pos,
            ),
        ]
    }

    fn get_input_location(&self, id: Input) -> Option<(f32, f32)> {
        let loc = self
            .ports_location()
            .iter()
            .map(|(_, loc)| <(f32, f32)>::from(loc))
            .collect::<Vec<(f32, f32)>>();

        if id == self.data_input {
            Some(loc[0])
        } else if id == self.address_input {
            Some(loc[1])
        } else if id == self.write_enable_input {
            Some(loc[2])
        } else if id == self.op_input {
            Some(loc[3])
        } else if id == Input::new(&self.id, DATA_MEM_READ_DATA_OUT_ID) {
            Some(loc[4])
        } else {
            None
        }
    }
}
