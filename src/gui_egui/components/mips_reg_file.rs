use crate::common::{EguiComponent, Simulator};
use crate::components::{RegFile, RegFormat};
use crate::gui_egui::editor::EditorMode;
use crate::gui_egui::gui::EguiExtra;
use crate::gui_egui::helper::basic_component_gui;
use egui::{ComboBox, Rect, Response, RichText, ScrollArea, Ui, Vec2};

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
        _editor_mode: EditorMode,
    ) -> Option<Vec<Response>> {
        basic_component_gui(
            self,
            &simulator,
            ui.ctx(),
            (120f32, 400f32),
            offset,
            scale,
            clip_rect,
            |ui| {
                ui.label(RichText::new("Register File").size(12f32 * scale));

                // A toggle button for showing register names
                ui.toggle_value(
                    &mut *self.show_reg_names.borrow_mut(),
                    RichText::new("Show names").size(12f32 * scale),
                );

                // showsing the display format of the register
                let mut tmp: RegFormat = self.reg_format.borrow().clone();
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

                // A scroll area with all the registers in one label
                ScrollArea::vertical().show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.set_height(ui.available_height());

                    // for each register format the u32 and pus that formatted sting onto
                    // the string representing all registers
                    let mut str: String = "".into();
                    for (i, val) in self.registers.borrow().iter().enumerate() {
                        // add reg name or reg number to the formatted string
                        str.push_str(
                            match *self.show_reg_names.borrow() {
                                true => format!("{:<4}", REG_NAMES[i]),
                                false => format!("r{:<3}", i),
                            }
                            .as_str(),
                        );

                        // add a formatted register to the string
                        // TODO move to separate function
                        str.push_str(
                            match *self.reg_format.borrow() {
                                RegFormat::Hex => format!("{:#010x}", val),
                                RegFormat::DecSigned => format!("{}", (*val) as i32),
                                RegFormat::DecUnsigned => format!("{}", val),
                                RegFormat::Bin => format!("{:#034b}", val),
                                RegFormat::UTF8BE => String::from_utf8_lossy(&val.to_be_bytes())
                                    .escape_debug()
                                    .to_string(),
                                RegFormat::UTF8LE => String::from_utf8_lossy(&val.to_le_bytes())
                                    .escape_debug()
                                    .to_string(),
                            }
                            .as_str(),
                        );
                        str.push_str("\n")
                    }

                    // push the string as monospace to the ui
                    ui.label(RichText::new(str).size(12f32 * scale).monospace())
                });
            },
        )
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
