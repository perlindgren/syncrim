use egui::{Key, KeyboardShortcut, Modifiers};

#[derive(Copy, Clone)]
pub struct Shortcuts {
    pub file_new: KeyboardShortcut,
    pub file_open: KeyboardShortcut,
    pub file_save: KeyboardShortcut,
    pub file_save_as: KeyboardShortcut,
    pub file_preferences: KeyboardShortcut,
    pub file_quit: KeyboardShortcut,
    pub edit_cut: KeyboardShortcut,
    pub edit_copy: KeyboardShortcut,
    pub edit_paste: KeyboardShortcut,
    pub view_zoom_in: KeyboardShortcut,
    pub view_zoom_out: KeyboardShortcut,
}

impl Shortcuts {
    pub fn new() -> Self {
        Shortcuts {
            file_new: KeyboardShortcut {
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
                key: Key::N,
            },
            file_open: KeyboardShortcut {
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
                key: Key::O,
            },
            file_save: KeyboardShortcut {
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
                key: Key::S,
            },
            file_save_as: KeyboardShortcut {
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: true,
                    mac_cmd: false,
                    command: false,
                },
                key: Key::S,
            },
            file_preferences: KeyboardShortcut {
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
                key: Key::P,
            },
            file_quit: KeyboardShortcut {
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
                key: Key::Q,
            },
            edit_cut: KeyboardShortcut {
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
                key: Key::X,
            },
            edit_copy: KeyboardShortcut {
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
                key: Key::C,
            },
            edit_paste: KeyboardShortcut {
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
                key: Key::P,
            },
            view_zoom_in: KeyboardShortcut {
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
                key: Key::PlusEquals,
            },
            view_zoom_out: KeyboardShortcut {
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
                key: Key::Minus,
            },
        }
    }

    pub fn inputs(self, ctx: &egui::Context, gui: &mut crate::egui_::Gui) {
        //let ctx = &mut ui.ctx();
        if ctx.input_mut(|i| i.consume_shortcut(&self.file_new)) {
            file_new_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.file_open)) {
            file_open_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.file_save)) {
            file_save_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.file_save_as)) {
            file_save_as_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.file_preferences)) {
            file_preferences_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.file_quit)) {
            file_quit_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.edit_cut)) {
            edit_cut_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.edit_copy)) {
            edit_copy_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.edit_paste)) {
            edit_paste_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.view_zoom_in)) {
            view_zoom_in_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.view_zoom_out)) {
            view_zoom_out_fn(gui);
        }
    }
}

pub fn file_new_fn(gui: &mut crate::egui_::Gui) {}
pub fn file_open_fn(gui: &mut crate::egui_::Gui) {}
pub fn file_save_fn(gui: &mut crate::egui_::Gui) {}
pub fn file_save_as_fn(gui: &mut crate::egui_::Gui) {}
pub fn file_preferences_fn(gui: &mut crate::egui_::Gui) {}
pub fn file_quit_fn(gui: &mut crate::egui_::Gui) {}
pub fn edit_cut_fn(gui: &mut crate::egui_::Gui) {}
pub fn edit_copy_fn(gui: &mut crate::egui_::Gui) {}
pub fn edit_paste_fn(gui: &mut crate::egui_::Gui) {}
pub fn view_zoom_in_fn(gui: &mut crate::egui_::Gui) {
    match gui.scale {
        x if (0.0f32..0.2f32).contains(&x) => gui.scale = 0.25f32,
        x if (0.2f32..0.4f32).contains(&x) => gui.scale = 0.5f32,
        x if (0.4f32..0.6f32).contains(&x) => gui.scale = 1f32,
        x if (0.9f32..1.1f32).contains(&x) => gui.scale = 1.5f32,
        x if (1.4f32..1.6f32).contains(&x) => gui.scale = 2f32,
        _ => gui.scale = 2f32,
    }
}
pub fn view_zoom_out_fn(gui: &mut crate::egui_::Gui) {
    match gui.scale {
        x if (0.2f32..0.4f32).contains(&x) => gui.scale = 0.1f32,
        x if (0.4f32..0.6f32).contains(&x) => gui.scale = 0.25f32,
        x if (0.9f32..1.1f32).contains(&x) => gui.scale = 0.5f32,
        x if (1.4f32..1.6f32).contains(&x) => gui.scale = 1f32,
        x if (1.9f32..2.1f32).contains(&x) => gui.scale = 1.5f32,
        _ => gui.scale = 0.1f32,
    }
}
