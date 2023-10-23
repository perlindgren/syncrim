use crate::components::InstrMem;
use egui::{
    Color32, Context, Label, Pos2, Rect, Response, RichText, Rounding, ScrollArea, Sense, Shape,
    SidePanel, Stroke, Ui, Vec2, Window,
};
use log::trace;
use riscv_asm_strings::Stringify;
use syncrim::common::{EguiComponent, Ports, Simulator};
use syncrim::gui_egui::component_ui::{
    drag_logic, input_change_id, input_selector, pos_drag_value, properties_window,
    rect_with_hover, visualize_ports,
};
use syncrim::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use syncrim::gui_egui::gui::EguiExtra;
use syncrim::gui_egui::helper::offset_helper;

impl InstrMem {
    fn side_panel(&self, ctx: &Context, simulator: Option<&mut Simulator>) {
        Window::new("Instruction Memory").show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                // trace!(":P");
                let pc: u32 = {
                    if simulator.is_some() {
                        simulator
                            .unwrap()
                            .get_input_value(&self.pc)
                            .try_into()
                            .unwrap_or(0)
                    } else {
                        0
                    }
                };
                for byte in &self.bytes {
                    if byte.0 % 4 == 0 {
                        let bg_color = {
                            if pc == (*byte.0 as u32) {
                                Color32::YELLOW
                            } else {
                                Color32::TRANSPARENT
                            }
                        };
                        let breakpoint_color = {
                            if self.breakpoints.borrow_mut().contains(byte.0) {
                                Color32::RED
                            } else {
                                Color32::TRANSPARENT
                            }
                        };
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("•").color(breakpoint_color));
                            match (&self.symbols.get(byte.0)) {
                                Some(s) => {
                                    ui.label(format!("{}:", s));
                                }
                                None => {}
                            };
                            ui.label(format!("0x{:08x}:", byte.0));
                            let instr = if !self.le {
                                (*self.bytes.get(&(*(byte.0) as usize)).unwrap() as u32) << 24
                                    | (*self.bytes.get(&((byte.0 + 1) as usize)).unwrap() as u32)
                                        << 16
                                    | (*self.bytes.get(&((byte.0 + 2) as usize)).unwrap() as u32)
                                        << 8
                                    | (*self.bytes.get(&((byte.0 + 3) as usize)).unwrap() as u32)
                            } else {
                                (*self.bytes.get(&(*(byte.0) as usize)).unwrap() as u32)
                                    | (*self.bytes.get(&((byte.0 + 1) as usize)).unwrap() as u32)
                                        << 8
                                    | (*self.bytes.get(&((byte.0 + 2) as usize)).unwrap() as u32)
                                        << 16
                                    | (*self.bytes.get(&((byte.0 + 3) as usize)).unwrap() as u32)
                                        << 24
                            };
                            let instruction_fmt = {
                                format!(
                                    "{:?}",
                                    match asm_riscv::I::try_from(instr) {
                                        Ok(i) => i.to_string(),
                                        Err(_) => "Unknown instruction".to_string(),
                                    }
                                )
                            };

                            if ui
                                .add(
                                    Label::new(
                                        RichText::new(format!(
                                            "0x{:08x}:{}",
                                            instr, instruction_fmt,
                                        ))
                                        .background_color(bg_color),
                                    )
                                    .sense(Sense::click()),
                                )
                                .clicked()
                            {
                                trace!("Clicked");
                                if !self.breakpoints.borrow_mut().remove(byte.0) {
                                    self.breakpoints.borrow_mut().insert(*byte.0);
                                }
                            };
                        });
                    }
                }
            });
        });
    }
}

#[typetag::serde]
impl EguiComponent for InstrMem {
    fn render(
        &self,
        ui: &mut Ui,
        ctx: &mut EguiExtra,
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
        //self.side_panel(ui.ctx(), simulator);
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
            ui.label("InstrMem");
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
        let r_vec = InstrMem::render(
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
                    &mut self.pc,
                    crate::components::INSTR_MEM_PC_ID.to_string(),
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
                crate::components::INSTR_MEM_PC_ID.to_string(),
                Pos2::new(
                    self.width / 10f32 * 1f32 - self.width / 2f32,
                    -self.height / 2f32,
                ) + own_pos,
            ),
            (
                crate::components::INSTR_MEM_INSTRUCTION_ID.to_string(),
                Pos2::new(
                    -self.width / 10f32 * 2f32 + self.width / 2f32,
                    -self.height / 2f32,
                ) + own_pos,
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
