pub(crate) struct Menu {}

impl Menu {
    pub(crate) fn new(ui: &mut egui::Ui, gui: &mut crate::egui_::Gui) {
        fn btn(ui: &mut egui::Ui, name: &str, keys: egui::KeyboardShortcut) -> egui::Response {
            ui.add(egui::Button::new(name).shortcut_text(ui.ctx().format_shortcut(&keys)))
        }
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if btn(ui, "File", gui.shortcuts.file_new).clicked() {
                    // New here
                }
                if btn(ui, "Open", gui.shortcuts.file_open).clicked() {
                    // Open here
                }
                ui.menu_button("Open Recent", |ui| {
                    // Recent here
                    //if ui.button("file1").clicked() {
                    //    // Open file
                    //}
                });
                ui.separator();
                if btn(ui, "Save", gui.shortcuts.file_save).clicked() {
                    // Save here
                }
                if btn(ui, "Save As", gui.shortcuts.file_save_as).clicked() {
                    // Save As here
                }
                ui.separator();
                if btn(ui, "Preferences", gui.shortcuts.file_preferences).clicked() {
                    // Preferences here
                }
                if btn(ui, "Quit", gui.shortcuts.file_quit).clicked() {
                    // Quit here
                }
            });

            ui.menu_button("Edit", |ui| {
                if btn(ui, "Cut", gui.shortcuts.edit_cut).clicked() {
                    // Cut here
                }
                if btn(ui, "Copy", gui.shortcuts.edit_copy).clicked() {
                    // Copy here
                }
                if btn(ui, "Paste", gui.shortcuts.edit_paste).clicked() {
                    // Paste here
                }
            });

            ui.menu_button("View", |ui| {
                if btn(ui, "Zoom In", gui.shortcuts.view_zoom_in).clicked() {
                    crate::egui_::shortcuts::view_zoom_in_fn(gui);
                }
                if btn(ui, "Zoom Out", gui.shortcuts.view_zoom_out).clicked() {
                    crate::egui_::shortcuts::view_zoom_out_fn(gui);
                }
                ui.menu_button("Zoom Level", |ui| {
                    if ui.button("10%").clicked() {
                        // 10% zoom here here
                        gui.scale = 0.1f32;
                    }
                    if ui.button("25%").clicked() {
                        // 25% zoom here here
                        gui.scale = 0.25f32;
                    }
                    if ui.button("50%").clicked() {
                        // 50% zoom here here
                        gui.scale = 0.5f32;
                    }
                    if ui.button("100%").clicked() {
                        // 100% zoom here here
                        gui.scale = 1f32;
                    }
                    if ui.button("150%").clicked() {
                        // 150% zoom here here
                        gui.scale = 1.5f32;
                    }
                    if ui.button("200%").clicked() {
                        // 200% zoom here here
                        gui.scale = 2f32;
                    }
                });
            });

            ui.menu_button("Help", |ui| {
                if ui.button("Show license").clicked() {
                    // Show license here
                }
                if ui.button("About").clicked() {
                    // About here
                }
            });
        });
        ui.horizontal(|ui| {
            if ui.button("▶").clicked() {
                //self.history.push(self.state.lens_values.clone());
                //self.simulator.clock(&mut self.state);
                println!("run!");
            }
            if ui.button("■").clicked() {
                //self.history.push(self.state.lens_values.clone());
                //self.simulator.clock(&mut self.state);
                println!("paused!");
            }
            if ui.button("⏮").clicked() {
                //self.history.push(self.state.lens_values.clone());
                //self.simulator.clock(&mut self.state);
                println!("stepped back once!");
            }
            if ui.button("⏭").clicked() {
                //self.history.push(self.state.lens_values.clone());
                gui.simulator.clock(&mut gui.state);
                println!("stepped once!");
            }
        });
    }
}
