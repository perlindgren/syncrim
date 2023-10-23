use crate::components::{Reg, RegFile, RegStore};
use egui::{
    Color32, Context, Label, Pos2, Rect, Response, Rounding, ScrollArea, Shape, Stroke, Ui, Vec2,
    Window,
};
use syncrim::common::{EguiComponent, Ports, Simulator};
use syncrim::gui_egui::component_ui::{
    drag_logic, input_change_id, input_selector, pos_drag_value, properties_window,
    rect_with_hover, visualize_ports,
};
use syncrim::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use syncrim::gui_egui::gui::EguiExtra;
use syncrim::gui_egui::helper::offset_helper;
impl RegFile {
    fn side_panel(&self, ctx: &Context, simulator: Option<&mut Simulator>) {
        Window::new("Register File").show(ctx, |ui| {
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for reg in RegStore::full_range() {
                        ui.horizontal(|ui| {
                            ui.label(format!(
                                "{:?}:0x{:08x}",
                                Reg::try_from(reg).unwrap(),
                                self.registers.0.borrow().get(reg as usize).unwrap()
                            ));
                        });
                    }
                });
        });
    }
}
#[typetag::serde]
impl EguiComponent for RegFile {
    fn render(
        &self,
        ui: &mut Ui,
        _context: &mut EguiExtra,
        simulator: Option<&mut Simulator>,
        offset: Vec2,
        scale: f32,
        clip_rect: Rect,
        editor_mode: EditorMode,
    ) -> Option<Vec<Response>> {
        // 21x41
        // middle: 11x 21y (0 0)
        let oh: fn((f32, f32), f32, Vec2) -> Pos2 = offset_helper;
        let offset_old = offset;
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let s = scale;
        let o = offset;
        // The shape
        let rect = Rect {
            min: oh((-self.width / 2f32, -self.height / 2f32), s, o),
            max: oh((self.width / 2f32, self.height / 2f32), s, o),
        };
        ui.painter().add(Shape::rect_stroke(
            rect,
            Rounding::none(),
            Stroke {
                width: scale,
                color: Color32::BLACK,
            },
        ));
        let r = rect_with_hover(rect, clip_rect, editor_mode, ui, self.id.clone(), |ui| {
            ui.label(format!("Id: {}", self.id.clone()));
            ui.label("RegFile");
        });
        match editor_mode {
            EditorMode::Simulator => {
                ui.allocate_ui_at_rect(rect, |ui| {
                    ui.vertical(|ui| {
                        for reg in RegStore::lo_range() {
                            ui.horizontal(|ui| {
                                ui.label(format!(
                                    "{:?}:0x{:08x}",
                                    Reg::try_from(reg).unwrap(),
                                    self.registers.0.borrow().get(reg as usize).unwrap()
                                ));
                            });
                        }
                    });
                });
                self.side_panel(ui.ctx(), simulator);
            }
            _ => visualize_ports(ui, self.ports_location(), offset_old, scale, clip_rect),
        }
        Some(vec![r])
    }

    fn render_editor(
        &mut self,
        ui: &mut Ui,
        context: &mut EguiExtra,
        simulator: Option<&mut Simulator>,
        offset: Vec2,
        scale: f32,
        clip_rect: Rect,
        id_ports: &[(syncrim::common::Id, Ports)],
        grid: &GridOptions,
        editor_mode: EditorMode,
    ) -> EditorRenderReturn {
        let r_vec = RegFile::render(
            self,
            ui,
            context,
            simulator,
            offset,
            scale,
            clip_rect,
            editor_mode,
        )
        .unwrap();
        let resp = &r_vec[0];
        let delete = drag_logic(
            ui.ctx(),
            resp,
            &mut self.pos,
            &mut context.pos_tmp,
            scale,
            offset,
            grid,
        );

        properties_window(
            ui,
            self.id.clone(),
            resp,
            &mut context.properties_window,
            |ui| {
                let mut clicked_dropdown = false;
                input_change_id(ui, &mut context.id_tmp, &mut self.id, id_ports);
                pos_drag_value(ui, &mut self.pos);
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.read_addr1,
                    crate::components::REG_FILE_READ_ADDR1_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.read_addr2,
                    crate::components::REG_FILE_READ_ADDR2_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.write_data,
                    crate::components::REG_FILE_WRITE_DATA_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.write_addr,
                    crate::components::REG_FILE_WRITE_ADDR_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.write_enable,
                    crate::components::REG_FILE_WRITE_ENABLE_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown
            },
        );

        EditorRenderReturn {
            delete,
            resp: Some(r_vec),
        }
    }

    fn ports_location(&self) -> Vec<(syncrim::common::Id, Pos2)> {
        let own_pos = Vec2::new(self.pos.0, self.pos.1);
        vec![
            (
                crate::components::REG_FILE_READ_ADDR1_ID.to_string(),
                Pos2::new(-self.width / 2f32, -self.height / 5f32) + own_pos,
            ),
            (
                crate::components::REG_FILE_READ_ADDR2_ID.to_string(),
                Pos2::new(-self.width / 2f32, self.height / 5f32) + own_pos,
            ),
            (
                crate::components::REG_FILE_WRITE_ADDR_ID.to_string(),
                Pos2::new(-self.width / 2f32, 0f32) + own_pos,
            ),
            (
                crate::components::REG_FILE_WRITE_DATA_ID.to_string(),
                Pos2::new(self.width / 4f32, -self.height / 2f32) + own_pos,
            ),
            (
                crate::components::REG_FILE_WRITE_ENABLE_ID.to_string(),
                Pos2::new(-self.width / 4f32, -self.height / 2f32) + own_pos,
            ),
            (
                crate::components::REG_FILE_REG_A_OUT.to_string(),
                Pos2::new(self.width / 2f32, -self.height / 5f32) + own_pos,
            ),
            (
                crate::components::REG_FILE_REG_B_OUT.to_string(),
                Pos2::new(self.width / 2f32, self.height / 5f32) + own_pos,
            ),
        ]
    }

    fn top_padding(&self) -> f32 {
        self.height / 4f32
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }

    fn get_pos(&self) -> (f32, f32) {
        self.pos
    }
}
