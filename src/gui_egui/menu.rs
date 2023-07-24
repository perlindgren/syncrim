use crate::gui_egui::{editor::Editor, gui::Gui, keymap};
use egui::Ui;

pub(crate) struct Menu {}

impl Menu {
    #[allow(clippy::new_ret_no_self)]
    pub(crate) fn new(ui: &mut egui::Ui, gui: &mut Gui) {
        egui::menu::bar(ui, |ui| {
            shared_buttons_file(gui, ui);
            shared_buttons_edit(gui, ui);

            let mut scale = gui.scale;
            shared_buttons_view(gui, ui, &mut scale);
            gui.scale = scale;

            shared_buttons_help(gui, ui);
        });
        ui.horizontal(|ui| {
            if ui.button("⟲").clicked() {
                keymap::control_reset_fn(gui);
            }
            if ui.button("⏮").clicked() {
                keymap::control_step_back_fn(gui);
            }
            if ui.button("⏭").clicked() {
                keymap::control_step_forward_fn(gui);
            }
            if ui.button("▶").clicked() {
                keymap::control_play_fn(gui);
            }
            if ui.button("⏸").clicked() {
                keymap::control_pause_fn(gui);
            }
            ui.label(format!("Clock #{}", gui.clock));
        });
    }

    #[allow(clippy::new_ret_no_self)]
    pub(crate) fn new_editor(ui: &mut egui::Ui, gui: &mut Gui) {
        fn editor(gui: &mut Gui) -> &mut Editor {
            gui.editor.as_mut().unwrap()
        }

        egui::menu::bar(ui, |ui| {
            shared_buttons_file(gui, ui);
            shared_buttons_edit(gui, ui);
            let mut scale = editor(gui).scale;
            shared_buttons_view(gui, ui, &mut scale);
            editor(gui).scale = scale;
            shared_buttons_help(gui, ui);
        });
    }
}

fn btn(ui: &mut egui::Ui, name: &str, keys: egui::KeyboardShortcut) -> egui::Response {
    ui.add(egui::Button::new(name).shortcut_text(ui.ctx().format_shortcut(&keys)))
}

fn shared_buttons_file(gui: &mut Gui, ui: &mut Ui) {
    ui.menu_button("File", |ui| {
        if btn(ui, "File", gui.shortcuts.file_new).clicked() {
            keymap::file_new_fn(gui);
        }
        if btn(ui, "Open", gui.shortcuts.file_open).clicked() {
            keymap::file_open_fn(gui);
        }
        ui.menu_button("Open Recent", |_ui| {
            // Recent here
            //if ui.button("file1").clicked() {
            //    // Open file
            //}
        });
        ui.separator();
        if btn(ui, "Save", gui.shortcuts.file_save).clicked() {
            keymap::file_save_fn(gui);
        }
        if btn(ui, "Save As", gui.shortcuts.file_save_as).clicked() {
            keymap::file_save_as_fn(gui);
        }
        ui.separator();
        if btn(ui, "Editor", gui.shortcuts.file_editor_toggle).clicked() {
            keymap::file_editor_toggle_fn(gui);
        }
        if btn(ui, "Preferences", gui.shortcuts.file_preferences).clicked() {
            keymap::file_preferences_fn(gui);
        }
        if btn(ui, "Quit", gui.shortcuts.file_quit).clicked() {
            keymap::file_quit_fn(gui);
        }
    });
}

fn shared_buttons_edit(gui: &mut Gui, ui: &mut Ui) {
    ui.menu_button("Edit", |ui| {
        if btn(ui, "Cut", gui.shortcuts.edit_cut).clicked() {
            keymap::edit_cut_fn(gui);
        }
        if btn(ui, "Copy", gui.shortcuts.edit_copy).clicked() {
            keymap::edit_copy_fn(gui);
        }
        if btn(ui, "Paste", gui.shortcuts.edit_paste).clicked() {
            keymap::edit_paste_fn(gui);
        }
    });
}

fn shared_buttons_view(gui: &mut Gui, ui: &mut Ui, scale: &mut f32) {
    ui.menu_button("View", |ui| {
        if btn(ui, "Zoom In", gui.shortcuts.view_zoom_in).clicked() {
            keymap::view_zoom_in_fn(gui);
        }
        if btn(ui, "Zoom Out", gui.shortcuts.view_zoom_out).clicked() {
            keymap::view_zoom_out_fn(gui);
        }
        ui.menu_button("Zoom Level", |ui| {
            if ui.button("10%").clicked() {
                // 10% zoom here here
                *scale = 0.1f32;
            }
            if ui.button("25%").clicked() {
                // 25% zoom here here
                *scale = 0.25f32;
            }
            if ui.button("50%").clicked() {
                // 50% zoom here here
                *scale = 0.5f32;
            }
            if ui.button("100%").clicked() {
                // 100% zoom here here
                *scale = 1f32;
            }
            if ui.button("150%").clicked() {
                // 150% zoom here here
                *scale = 1.5f32;
            }
            if ui.button("200%").clicked() {
                // 200% zoom here here
                *scale = 2f32;
            }
        });
    });
}

fn shared_buttons_help(_gui: &mut Gui, ui: &mut Ui) {
    ui.menu_button("Help", |ui| {
        if ui.button("Show license").clicked() {
            // Show license here
        }
        if ui.button("About").clicked() {
            // About here
        }
    });
}
