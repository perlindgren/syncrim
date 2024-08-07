use crate::common::{EguiComponent, Ports, SignalUnsigned, Simulator};
use crate::components::{RegFile, RegFormat};
use crate::gui_egui::component_ui::{rect_with_hover, visualize_ports};
use crate::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use crate::gui_egui::gui::EguiExtra;
use crate::gui_egui::helper::{component_area, offset_helper};
use egui::{
    scroll_area, Color32, ComboBox, Pos2, Rect, Response, RichText, ScrollArea, Shape, Slider,
    Stroke, Ui, Vec2,
};

const REG_NAMES: [&str; 32] = [
    "zero", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5", "t6",
    "s7", "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "t8", "t9", "k0", "k1", "gp", "sp", "fp",
    "ra",
];

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
        let oh: fn((f32, f32), f32, Vec2) -> Pos2 = offset_helper;
        let offset_old = offset;
        let mut offset = offset;
        offset.x += self.pos.0 * scale;
        offset.y += self.pos.1 * scale;
        let s = scale;
        let o = offset;

        let rs: u32;
        let rt: u32;
        if let Some(sim) = simulator {
            rs = sim.get_input_value(&self.rs_address_in).try_into().unwrap();
            rt = sim.get_input_value(&self.rt_address_in).try_into().unwrap();
        } else {
            rs = 32; // register that dont exist
            rt = 32; //
        }
        // The shape
        let rect = Rect {
            min: oh((-60f32, -90f32), s, o),
            max: oh((60f32, 90f32), s, o),
        };

        component_area(self.id.to_string(), ui.ctx(), offset.to_pos2(), |ui| {
            ui.style_mut().visuals.panel_fill = Color32::LIGHT_BLUE;
            ui.set_height(rect.height());
            ui.set_width(rect.width());
            ui.group(|ui| {
                let mut tmp: RegFormat = self.reg_format.borrow().clone();
                ui.label(RichText::new("Register File").size(12f32 * scale));
                ui.toggle_value(
                    &mut *self.show_reg_names.borrow_mut(),
                    RichText::new("Show names").size(12f32 * scale),
                );
                ComboBox::from_id_source(&self.id)
                    .selected_text(RichText::new(format!("{:?}", tmp)).size(12f32 * scale))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut tmp, RegFormat::Hex, "Hex");
                        ui.selectable_value(&mut tmp, RegFormat::DecUnsigned, "Decimal");
                        ui.selectable_value(&mut tmp, RegFormat::DecSigned, "Decimal signed");
                        ui.selectable_value(&mut tmp, RegFormat::Bin, "Binary");
                        ui.selectable_value(&mut tmp, RegFormat::UTF8BE, "UTF-8 big endian");
                        ui.selectable_value(&mut tmp, RegFormat::UTF8LE, "UTF-8 little endian");
                    });
                *self.reg_format.borrow_mut() = tmp;

                ui.separator();
                ScrollArea::vertical()
                    .max_height(rect.height())
                    .max_width(rect.width())
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        let mut str: String = "".into();
                        for (i, val) in self.registers.borrow().iter().enumerate() {
                            str.push_str(
                                match *self.show_reg_names.borrow() {
                                    true => format!("{:<4}", REG_NAMES[i]),
                                    false => format!("r{:<3}", i),
                                }
                                .as_str(),
                            );
                            str.push_str(
                                match *self.reg_format.borrow() {
                                    RegFormat::Hex => format!("{:#010x}", val),
                                    RegFormat::DecSigned => format!("{}", (*val) as i32),
                                    RegFormat::DecUnsigned => format!("{}", val),
                                    RegFormat::Bin => format!("{:#034b}", val),
                                    RegFormat::UTF8BE => {
                                        String::from_utf8_lossy(&val.to_be_bytes())
                                            .escape_debug()
                                            .to_string()
                                    }
                                    RegFormat::UTF8LE => {
                                        String::from_utf8_lossy(&val.to_le_bytes())
                                            .escape_debug()
                                            .to_string()
                                    }
                                }
                                .as_str(),
                            );
                            str.push_str("\n")
                        }
                        ui.label(RichText::new(str).size(12f32 * scale).monospace())
                    })
            });
        });
        // r1.union(r2.)
        // .inner
        // .response
        // .on_hover_ui(|ui| {
        //     ui.label("on_hover");
        // });

        match editor_mode {
            EditorMode::Simulator => (),
            _ => visualize_ports(ui, self.ports_location(), offset_old, scale, clip_rect),
        }
        None
    }

    fn ports_location(&self) -> Vec<(crate::common::Id, Pos2)> {
        let own_pos = Vec2::new(self.pos.0, self.pos.1);
        vec![
            (
                crate::components::SEXT_IN_ID.to_string(),
                Pos2::new(-40f32, 0f32) + own_pos,
            ),
            (
                crate::components::SEXT_OUT_ID.to_string(),
                Pos2::new(40f32, 0f32) + own_pos,
            ),
        ]
    }

    fn top_padding(&self) -> f32 {
        20f32
    }

    fn set_pos(&mut self, pos: (f32, f32)) {
        self.pos = pos;
    }

    fn get_pos(&self) -> (f32, f32) {
        self.pos
    }
}
