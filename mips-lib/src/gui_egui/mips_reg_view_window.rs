use egui::{
    ComboBox, RichText, ScrollArea, ViewportBuilder, ViewportId,Color32
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct RegViewWindow {
    pub visible: bool,
    title: String,
    id: String,

    // used for formatting the view
    reg_format: RegFormat,

    // used for show register
    register_values: [u32; 32],
    show_reg_names: bool,
    register_changed: [bool; 32],
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
enum RegFormat {
    Hex,
    Bin,
    DecSigned,
    DecUnsigned,
    UTF8BE,
    UTF8LE,
}

const REG_NAMES: [&str; 32] = [
    "zero", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5", "t6",
    "s7", "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "t8", "t9", "k0", "k1", "gp", "sp", "fp",
    "ra",
];

impl RegViewWindow {
    // set register values
    pub fn set_reg_values(&mut self, reg_values: [u32; 32], reg_value_has_changed: [bool; 32]) {
        self.register_values = reg_values;
        self.register_changed = reg_value_has_changed;
    }

    // creates a new register file view window with id string and the given memory
    pub fn new(id: String, title: String) -> Self {
        RegViewWindow {
            title,
            id,
            visible: false,
            register_values: [0; 32],
            show_reg_names: true,
            reg_format: RegFormat::Hex,
            register_changed: [false; 32],
        }
    }

    // Updates the content in the window.
    // If there isn't a window and the "Show reg window"-button has been pressed: show the window
    pub fn render(&mut self, ctx: &egui::Context) {
        if !self.visible {
            return;
        };

        ctx.show_viewport_immediate(
            ViewportId::from_hash_of(&self.id),
            ViewportBuilder::default().with_title(&self.title),
            |ctx, _class| {
                // If window is close is sent set visible to false
                // WARNING, DON'T USE CONTEXT INSIDE READER: WILL CAUSE DEADLOCK
                if ctx.input(|i| i.viewport().close_requested()) {
                    self.visible = false
                }

                self.render_top(ctx);

                self.render_registers(ctx);
            },
        );
    }

    // Update the top toolbar in the window
    fn render_top(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top(self.id.clone()).show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.toggle_value(&mut self.show_reg_names, "Show names");

                // show the display format of the register
                let mut tmp: RegFormat = self.reg_format.clone();
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
                self.reg_format = tmp;
            });
        });
    }

    // Update the scroll area with all the registers.
    // TODO: separate register names and values. When scrolling horizontally the names should stick at the same position
    fn render_registers(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both().show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);

                // Format and display the register values
                for (i, val) in self.register_values.iter().enumerate() {
                    let name = if self.show_reg_names {
                        format!("{:<4}", REG_NAMES[i])
                    } else {
                        format!("r{:<3}", i)
                    };
                    let val_str = match self.reg_format {
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

                    // Colour the registers that was last changed
                    let color:Color32;
                    if self.register_changed[i] {
                        color = Color32::RED;
                    } else {
                        color = Color32::GRAY;
                    }

                    // Draw the label with monospace font and the chosen color
                    ui.label(RichText::new(text).monospace().color(color));
                }
            });
        });
    }
}