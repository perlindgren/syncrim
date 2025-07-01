use crate::components::{reg_file_fields, RegFile, RegFormat};
use egui::{vec2, Color32, ComboBox, Pos2, Rect, Response, RichText, ScrollArea, Ui, Vec2};
use syncrim::common::{EguiComponent, Input, Ports, Simulator};
use syncrim::gui_egui::editor::{EditorMode, EditorRenderReturn, GridOptions};
use syncrim::gui_egui::gui::EguiExtra;
use syncrim::gui_egui::helper::basic_component_gui;
use syncrim::signal::Id;

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
        let mut reg_view_vis: bool = self.reg_view.borrow().visible;

        let r = basic_component_gui(self, &simulator, ui.ctx(), offset, scale, clip_rect, |ui| {
            ui.set_width(120f32 * scale);
            ui.set_height(250f32 * scale);
            ui.label("Register File");

            // showing the display format of the register
            let mut tmp: RegFormat = self.reg_format.borrow().clone();
            ComboBox::from_id_source(&self.id)
                .selected_text(format!("{:?}", tmp))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut tmp, RegFormat::Hex, "Hex");
                    ui.selectable_value(&mut tmp, RegFormat::DecUnsigned, "Decimal");
                    ui.selectable_value(&mut tmp, RegFormat::DecSigned, "Decimal signed");
                    ui.selectable_value(&mut tmp, RegFormat::Bin, "Binary");
                    ui.selectable_value(&mut tmp, RegFormat::UTF8BE, "UTF-8 big endian");
                    ui.selectable_value(&mut tmp, RegFormat::UTF8LE, "UTF-8 little endian");
                });
            *self.reg_format.borrow_mut() = tmp;

            match reg_view_vis {
                false => {
                    if ui.button("Show reg window").clicked() {
                        reg_view_vis = true;
                    }
                }
                true => {
                    ui.toggle_value(&mut reg_view_vis, "Hide reg window");
                }
            };

            ui.separator();

            // A scroll area with all the registers
            ScrollArea::both().show(ui, |ui| {
                ui.set_width(ui.available_width());
                // Format and display the register values
                for (i, val) in self.registers.borrow().iter().enumerate() {
                    let name = if *(self.show_reg_names.borrow()) {
                        format!("{:<4}", REG_NAMES[i])
                    } else {
                        format!("r{:<3}", i)
                    };
                    let val_str = match *self.reg_format.borrow() {
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
                    };
                    let text = format!("{} {}", name, val_str);

                    // Colour the register that was last changed
                    let color: Color32;
                    if *(self.changed_register.borrow()) == i as u32 {
                        color = Color32::RED;
                    } else {
                        color = Color32::GRAY;
                    }

                    // Draw the label with monospace font and the chosen color
                    ui.label(RichText::new(text).monospace().color(color));
                }
            });
        });
        // {} to drop RefMut as early as possible
        {
            let mut reg_view = self.reg_view.borrow_mut();
            reg_view.visible = reg_view_vis;
            reg_view.render(ui.ctx());

            // Update the register view with the current register values
            reg_view.set_reg_values(*self.registers.borrow(), *self.changed_register.borrow());
        }
        r
    }
    fn render_editor(
        &mut self,
        ui: &mut Ui,
        context: &mut EguiExtra,
        simulator: Option<&mut Simulator>,
        offset: Vec2,
        scale: f32,
        clip_rect: Rect,
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

    fn get_input_location(&self, id: Input) -> Option<(f32, f32)> {
        // components size = (120,250)
        let margin = egui::style::Spacing::default().window_margin;

        // inputs
        if id == self.rs_address_in {
            Some((Pos2::from(self.pos) + vec2(0f32, -125.0 - margin.top)).into())
        } else if id == self.rt_address_in {
            Some((Pos2::from(self.pos) + vec2(0f32, 125.0 + margin.bottom)).into())
        } else if id == self.write_enable_in {
            Some((Pos2::from(self.pos) + vec2(-60.0 - margin.left, 70.0)).into())
        } else if id == self.write_address_in {
            Some((Pos2::from(self.pos) + vec2(-60.0 - margin.left, 90.0)).into())
        } else if id == self.write_data_in {
            Some((Pos2::from(self.pos) + vec2(-60.0 - margin.left, 110.0)).into())
        // outputs
        } else if id == Input::new(&self.id, reg_file_fields::RS_VALUE_OUT_ID) {
            Some((Pos2::from(self.pos) + vec2(60.0 + margin.right, 40.0)).into())
        } else if id == Input::new(&self.id, reg_file_fields::RT_VALUE_OUT_ID) {
            Some((Pos2::from(self.pos) + vec2(60.0 + margin.right, -40.0)).into())
        // no match
        } else {
            None
        }
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
