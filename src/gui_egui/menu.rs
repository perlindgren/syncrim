use crate::gui_egui::{
    editor::{Editor, GridOptions},
    gui::Gui,
    keymap,
};
use egui::{menu, Button, DragValue, KeyboardShortcut, Response, Ui};
pub(crate) struct Menu {}

impl Menu {
    #[allow(clippy::new_ret_no_self)]
    pub(crate) fn new(ui: &mut Ui, gui: &mut Gui) {
        menu::bar(ui, |ui| {
            shared_buttons_file(gui, ui);
            shared_buttons_edit(gui, ui);

            let mut scale = gui.scale;
            shared_buttons_view(gui, ui, &mut scale, |_| {});
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
            if let Some(s) = gui.simulator.as_ref() {
                ui.label(format!("Cycle #{}", s.cycle));
            }
        });
    }

    #[allow(clippy::new_ret_no_self)]
    pub(crate) fn new_editor(ui: &mut Ui, gui: &mut Gui) {
        fn editor(gui: &mut Gui) -> &mut Editor {
            gui.editor.as_mut().unwrap()
        }

        menu::bar(ui, |ui| {
            shared_buttons_file(gui, ui);
            shared_buttons_edit(gui, ui);
            let mut scale = editor(gui).scale;
            let mut grid_enable = editor(gui).grid.enable;
            let mut grid_size = editor(gui).grid.size;
            let mut grid_opacity = editor(gui).grid.opacity;
            let mut grid_snap_enable = editor(gui).grid.snap_enable;
            let mut grid_snap_distance = editor(gui).grid.snap_distance;
            let view_grid_toggle = gui.shortcuts.view_grid_toggle;
            let view_grid_snap_toggle = gui.shortcuts.view_grid_snap_toggle;
            shared_buttons_view(gui, ui, &mut scale, |ui| {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut grid_enable, "Grid Enable");
                    ui.label(ui.ctx().format_shortcut(&view_grid_toggle));
                });
                ui.horizontal(|ui| {
                    ui.label("Grid Size:");
                    ui.add(DragValue::new(&mut grid_size).range(5f32..=10000f32));
                });
                ui.horizontal(|ui| {
                    ui.label("Grid Opacity:");
                    ui.add(
                        DragValue::new(&mut grid_opacity)
                            .range(0f32..=1f32)
                            .speed(0.02f32),
                    );
                });
                ui.horizontal(|ui| {
                    ui.checkbox(&mut grid_snap_enable, "Grid Snapping Enable");
                    ui.label(ui.ctx().format_shortcut(&view_grid_snap_toggle));
                });
                ui.horizontal(|ui| {
                    ui.label("Grid Snap Distance:");
                    ui.add(DragValue::new(&mut grid_snap_distance).range(0f32..=100f32));
                });
            });
            editor(gui).scale = scale;
            editor(gui).grid = GridOptions {
                enable: grid_enable,
                size: grid_size,
                opacity: grid_opacity,
                snap_enable: grid_snap_enable,
                snap_distance: grid_snap_distance,
            };
            shared_buttons_help(gui, ui);
        });
        ui.horizontal(|ui| {
            let wire_button = ui.button("➖").on_hover_text("Wire mode");
            if wire_button.clicked() {
                keymap::editor_wire_mode_fn(gui);
            }
        });
    }
}

fn btn(ui: &mut Ui, name: &str, keys: KeyboardShortcut) -> Response {
    ui.add(Button::new(name).shortcut_text(ui.ctx().format_shortcut(&keys)))
}

fn shared_buttons_file(gui: &mut Gui, ui: &mut Ui) {
    ui.menu_button("File", |ui| {
        if btn(ui, "New", gui.shortcuts.file_new).clicked() {
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

fn shared_buttons_view<P>(gui: &mut Gui, ui: &mut Ui, scale: &mut f32, mut f: P)
where
    P: FnMut(&mut Ui),
{
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
        f(ui);
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
