use crate::components::RVMem;
use egui::FontId;
use egui::{
    Color32, Context, Label, Pos2, Rect, Response, Rounding, Shape, Slider, Stroke, Ui, Vec2,
    Window,
};
use egui_extras::{Column, TableBuilder};
use syncrim::common::{EguiComponent, Ports, Simulator};
use syncrim::gui_egui::component_ui::{
    drag_logic, input_change_id, input_selector, pos_drag_value, properties_window,
    rect_with_hover, visualize_ports,
};
use syncrim::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use syncrim::gui_egui::gui::EguiExtra;
use syncrim::gui_egui::helper::offset_helper;
impl RVMem {
    fn side_panel(&self, ctx: &Context, _simulator: Option<&mut Simulator>) {
        Window::new("Data Memory").show(ctx, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .column(Column::initial(75.0))
                .column(Column::initial(75.0))
                .column(Column::initial(50.0))
                .header(30.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Address");
                    });
                    header.col(|ui| {
                        ui.heading("HEX");
                    });
                    header.col(|ui| {
                        ui.heading("ASCII");
                    });
                })
                .body(|body| {
                    body.rows(
                        15.0,
                        ((self.range.end - self.range.start) / 4) as usize,
                        |index, mut row| {
                            //println!("{}", index);
                            let address = self.range.start as usize + index * 4;
                            let memory = self.memory.0.borrow().clone();
                            row.col(|ui| {
                                ui.label(format!("0x{:08x}", address));
                            });
                            let mut bytes = [0u8; 4];
                            if self.big_endian {
                                bytes[0] = *(memory).get(&address).unwrap();
                                bytes[1] = *(memory).get(&(address + 1)).unwrap();
                                bytes[2] = *(memory).get(&(address + 2)).unwrap();
                                bytes[3] = *(memory).get(&(address + 3)).unwrap();
                            } else {
                                bytes[3] = *(memory).get(&address).unwrap();
                                bytes[2] = *(memory).get(&(address + 1)).unwrap();
                                bytes[1] = *(memory).get(&(address + 2)).unwrap();
                                bytes[0] = *(memory).get(&(address + 3)).unwrap();
                            }
                            let word = format!(
                                "0x{:02x}{:02x}{:02x}{:02x}",
                                bytes[0], bytes[1], bytes[2], bytes[3]
                            );
                            let mut ascii = "".to_string();
                            for b in bytes {
                                if b > 0x1f && b < 0x7f {
                                    ascii += &format!("{}", b as char);
                                } else {
                                    ascii += " ";
                                }
                            }
                            row.col(|ui| {
                                ui.add(Label::new(word).truncate(true));
                            });
                            row.col(|ui| {
                                ui.add(Label::new(ascii).truncate(true));
                            });
                        },
                    );
                });
        });
    }
}

#[typetag::serde]
impl EguiComponent for RVMem {
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
        // 201x101
        // middle: 101x 51y (0 0)
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
            Rounding::ZERO,
            Stroke {
                width: scale,
                color: Color32::BLACK,
            },
        ));
        ui.painter().text(
            o.to_pos2(),
            egui::Align2::CENTER_CENTER,
            "Data Mem",
            FontId::monospace(14.0),
            Color32::BLACK,
        );

        let r = rect_with_hover(rect, clip_rect, editor_mode, ui, self.id.clone(), |ui| {
            ui.label(format!("Id: {}", self.id.clone()));
            ui.label("Mem");
        });
        match editor_mode {
            EditorMode::Simulator => {
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
        let r_vec = RVMem::render(
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
        context.size_rect = resp.rect;
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
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut self.width, 0f32..=400f32).text("width"));
                    ui.add(Slider::new(&mut self.height, 0f32..=400f32).text("height"));
                });
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.data,
                    crate::components::RV_MEM_DATA_I_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.addr,
                    crate::components::RV_MEM_ADDR_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.ctrl,
                    crate::components::RV_MEM_CTRL_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.sext,
                    crate::components::RV_MEM_SEXT_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                clicked_dropdown |= input_selector(
                    ui,
                    &mut self.size,
                    crate::components::RV_MEM_SIZE_ID.to_string(),
                    id_ports,
                    self.id.clone(),
                );
                // todo: something about memory?
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
                crate::components::RV_MEM_DATA_I_ID.to_string(),
                Pos2::new(
                    self.width / 10f32 * 1f32 - self.width / 2f32,
                    -self.height / 2f32,
                ) + own_pos,
            ),
            (
                crate::components::RV_MEM_ADDR_ID.to_string(),
                Pos2::new(
                    self.width / 10f32 * 2f32 - self.width / 2f32,
                    -self.height / 2f32,
                ) + own_pos,
            ),
            (
                crate::components::RV_MEM_CTRL_ID.to_string(),
                Pos2::new(
                    self.width / 10f32 * 3f32 - self.width / 2f32,
                    -self.height / 2f32,
                ) + own_pos,
            ),
            (
                crate::components::RV_MEM_SEXT_ID.to_string(),
                Pos2::new(
                    self.width / 10f32 * 4f32 - self.width / 2f32,
                    -self.height / 2f32,
                ) + own_pos,
            ),
            (
                crate::components::RV_MEM_SIZE_ID.to_string(),
                Pos2::new(
                    -self.width / 10f32 * 3f32 + self.width / 2f32,
                    -self.height / 2f32,
                ) + own_pos,
            ),
            (
                crate::components::RV_MEM_DATA_O_ID.to_string(),
                Pos2::new(
                    -self.width / 10f32 * 2f32 + self.width / 2f32,
                    -self.height / 2f32,
                ) + own_pos,
            ),
        ]
    }

    fn top_padding(&self) -> f32 {
        self.height / 2f32
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }

    fn get_pos(&self) -> (f32, f32) {
        self.pos
    }
}
