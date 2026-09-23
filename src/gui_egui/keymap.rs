use crate::common::{ComponentStore, Simulator};
use crate::component_store::ComponentStoreLoadError;
use crate::gui_egui::editor::{Editor, EditorMode};
use crate::gui_egui::editor_wire_mode::reset_wire_mode;
use crate::gui_egui::gui::create_contexts;
use crate::gui_egui::library::reset_input_mode;
use crate::gui_egui::Gui;
use egui::{Key, KeyboardShortcut, Modifiers};
use rfd::FileDialog;
use std::path::PathBuf;

#[derive(Copy, Clone)]
pub struct Shortcuts {
    pub file_new: KeyboardShortcut,
    pub file_open: KeyboardShortcut,
    pub file_save: KeyboardShortcut,
    pub file_save_as: KeyboardShortcut,
    pub file_editor_toggle: KeyboardShortcut,
    pub file_preferences: KeyboardShortcut,
    pub file_quit: KeyboardShortcut,
    pub edit_cut: KeyboardShortcut,
    pub edit_copy: KeyboardShortcut,
    pub edit_paste: KeyboardShortcut,
    pub view_zoom_in: KeyboardShortcut,
    pub view_zoom_out: KeyboardShortcut,
    pub view_grid_toggle: KeyboardShortcut,
    pub view_grid_snap_toggle: KeyboardShortcut,
    pub control_play_toggle: KeyboardShortcut,
    pub control_play: KeyboardShortcut,
    pub control_pause: KeyboardShortcut,
    pub control_reset: KeyboardShortcut,
    pub control_step_forward: KeyboardShortcut,
    pub control_step_back: KeyboardShortcut,
    pub editor_wire_mode: KeyboardShortcut,
    pub editor_escape: KeyboardShortcut,
}

impl Default for Shortcuts {
    fn default() -> Self {
        Self::new()
    }
}

impl Shortcuts {
    pub fn new() -> Self {
        let ctrl = Modifiers {
            alt: false,
            ctrl: true,
            shift: false,
            mac_cmd: false,
            command: false,
        };
        let shift = Modifiers {
            alt: false,
            ctrl: false,
            shift: true,
            mac_cmd: false,
            command: false,
        };
        let none = Modifiers {
            alt: false,
            ctrl: false,
            shift: false,
            mac_cmd: false,
            command: false,
        };

        Shortcuts {
            file_new: KeyboardShortcut {
                modifiers: ctrl,
                logical_key: Key::N,
            },
            file_open: KeyboardShortcut {
                modifiers: ctrl,
                logical_key: Key::O,
            },
            file_save: KeyboardShortcut {
                modifiers: ctrl,
                logical_key: Key::S,
            },
            file_save_as: KeyboardShortcut {
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: true,
                    mac_cmd: false,
                    command: false,
                },
                logical_key: Key::S,
            },
            file_editor_toggle: KeyboardShortcut {
                modifiers: ctrl,
                logical_key: Key::E,
            },
            file_preferences: KeyboardShortcut {
                modifiers: ctrl,
                logical_key: Key::P,
            },
            file_quit: KeyboardShortcut {
                modifiers: ctrl,
                logical_key: Key::Q,
            },
            edit_cut: KeyboardShortcut {
                modifiers: ctrl,
                logical_key: Key::X,
            },
            edit_copy: KeyboardShortcut {
                modifiers: ctrl,
                logical_key: Key::C,
            },
            edit_paste: KeyboardShortcut {
                modifiers: ctrl,
                logical_key: Key::P,
            },
            view_zoom_in: KeyboardShortcut {
                modifiers: ctrl,
                logical_key: Key::Plus,
            },
            view_zoom_out: KeyboardShortcut {
                modifiers: ctrl,
                logical_key: Key::Minus,
            },
            view_grid_toggle: KeyboardShortcut {
                modifiers: ctrl,
                logical_key: Key::G,
            },
            view_grid_snap_toggle: KeyboardShortcut {
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: true,
                    mac_cmd: false,
                    command: false,
                },
                logical_key: Key::G,
            },
            control_play: KeyboardShortcut {
                modifiers: none,
                logical_key: Key::F6,
            },
            control_play_toggle: KeyboardShortcut {
                modifiers: none,
                logical_key: Key::F5,
            },
            control_pause: KeyboardShortcut {
                modifiers: shift,
                logical_key: Key::F5,
            },
            control_reset: KeyboardShortcut {
                modifiers: Modifiers {
                    alt: false,
                    ctrl: true,
                    shift: true,
                    mac_cmd: false,
                    command: false,
                },
                logical_key: Key::F5,
            },
            control_step_forward: KeyboardShortcut {
                modifiers: none,
                logical_key: Key::F10,
            },
            control_step_back: KeyboardShortcut {
                modifiers: shift,
                logical_key: Key::F10,
            },
            editor_wire_mode: KeyboardShortcut {
                modifiers: none,
                logical_key: Key::W,
            },
            editor_escape: KeyboardShortcut {
                modifiers: none,
                logical_key: Key::Escape,
            },
        }
    }

    pub fn inputs(self, ctx: &egui::Context, gui: &mut Gui) {
        if ctx.input_mut(|i| i.consume_shortcut(&self.file_new)) {
            file_new_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.file_open)) {
            let _ = file_open_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.file_save)) {
            file_save_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.file_save_as)) {
            file_save_as_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.file_editor_toggle)) {
            file_editor_toggle_fn(gui);
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
        if ctx.input_mut(|i| i.consume_shortcut(&self.view_grid_toggle)) {
            view_grid_toggle_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.view_grid_snap_toggle)) {
            view_grid_snap_toggle_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.control_play_toggle)) {
            control_play_toggle_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.control_play)) {
            control_play_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.control_pause)) {
            control_pause_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.control_reset)) {
            control_reset_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.control_step_forward)) {
            control_step_forward_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.control_step_back)) {
            control_step_back_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.editor_wire_mode)) {
            editor_wire_mode_fn(gui);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&self.editor_escape)) {
            editor_escape_fn(gui);
        }
    }
}

pub fn file_new_fn(_gui: &mut Gui) {}
pub fn file_open_fn(gui: &mut Gui) -> Result<(), ComponentStoreLoadError> {
    let files = FileDialog::new().add_filter("json", &["json"]).pick_file();
    if let Some(path_buf) = files {
        gui.path = path_buf;
    }
    let cs = ComponentStore::load_file(&gui.path)?;
    let contexts = create_contexts(&cs.store);
    match gui.editor_use {
        true => {
            if let Some(e) = gui.editor.as_mut() {
                // Clear all references
                reset_wire_mode(&mut e.wm);
                reset_input_mode(&mut e.im);
                e.components = cs.store;
                e.contexts = contexts;
            }
        }
        false => {
            let simulator = Simulator::new(cs);
            gui.contexts = contexts;
            match simulator {
                Err(e) => {
                    println!("couldn't open file with simulator: {}", e);
                }
                Ok(s) => {
                    let _ = gui.simulator.take();
                    gui.simulator = Some(s);
                }
            }
        }
    }
    Ok(())
}
pub fn file_save_fn(gui: &mut Gui) {
    match gui.editor_use {
        true => {
            if let Some(e) = gui.editor.as_mut() {
                ComponentStore {
                    store: e.components.clone(),
                }
                .save_file(&gui.path)
            }
        }
        false => ComponentStore {
            store: gui.simulator.clone().unwrap().ordered_components,
        }
        .save_file(&PathBuf::from("file.json")),
    }
}
pub fn file_save_as_fn(gui: &mut Gui) {
    let files = FileDialog::new().add_filter("json", &["json"]).save_file();
    if let Some(path_buf) = files {
        gui.path = path_buf;
        file_save_fn(gui);
    }
}
pub fn file_editor_toggle_fn(gui: &mut Gui) {
    // Auto-save
    file_save_fn(gui);
    match gui.editor_use {
        true => {
            gui.editor_use = false;
            if let Some(e) = gui.editor.as_mut() {
                let components = e.components.clone();
                gui.contexts = create_contexts(&components);
                let simulator = Simulator::new(ComponentStore { store: components });
                match simulator {
                    Err(e) => {
                        gui.editor_use = true;
                        println!("error: {}", e);
                    }
                    Ok(s) => gui.simulator = Some(s),
                }
            }
        }
        false => {
            let editor_existed: bool = gui.editor.as_mut().is_some();

            let simulator = gui.simulator.take().unwrap();
            let components = simulator.ordered_components;

            if !editor_existed {
                gui.editor = Some(Editor::gui(components, &gui.path, &gui.library));
            }

            gui.editor_use = true;
        }
    }
}
pub fn file_preferences_fn(_gui: &mut Gui) {}
pub fn file_quit_fn(_gui: &mut Gui) {}
pub fn edit_cut_fn(_gui: &mut Gui) {}
pub fn edit_copy_fn(_gui: &mut Gui) {}
pub fn edit_paste_fn(_gui: &mut Gui) {}
/// Discrete zoom levels used by the zoom in/out shortcuts and menu buttons
pub const ZOOM_LEVELS: [f32; 11] = [0.1, 0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 2.0, 2.5, 3.0, 4.0];
pub const ZOOM_MIN: f32 = 0.1;
pub const ZOOM_MAX: f32 = 4.0;

/// Sets the zoom of the active view (simulator or editor) to `new_scale`,
/// keeping the point under `anchor` (screen coordinates) fixed on screen.
/// If `anchor` is None the center of the drawing area is used.
pub fn view_zoom_to(gui: &mut Gui, new_scale: f32, anchor: Option<egui::Pos2>) {
    let anchor = anchor.unwrap_or(gui.canvas_rect.center());
    let (scale, pan, offset) = match gui.editor_use {
        true => {
            let e = gui.editor.as_mut().unwrap();
            (&mut e.scale, &mut e.pan, e.offset)
        }
        false => (&mut gui.scale, &mut gui.pan, gui.offset),
    };
    let new_scale = new_scale.clamp(ZOOM_MIN, ZOOM_MAX);
    // screen = world * scale + offset + pan
    // Solve for the world position under the anchor, then pick the pan that
    // puts that world position under the anchor again with the new scale.
    if anchor.is_finite() {
        let world = (anchor.to_vec2() - offset - *pan) / *scale;
        *pan = anchor.to_vec2() - offset - world * new_scale;
    }
    *scale = new_scale;
    if let Some(e) = gui.editor.as_mut().filter(|_| gui.editor_use) {
        e.offset_and_pan = e.offset + e.pan;
    }
}

fn view_scale(gui: &Gui) -> f32 {
    match gui.editor_use {
        true => gui.editor.as_ref().unwrap().scale,
        false => gui.scale,
    }
}

/// Zoom factor per point of scroll, a mouse wheel notch is roughly 50 points
const ZOOM_SCROLL_SPEED: f32 = 1.0 / 250.0;

/// Zooms the active view with the scroll wheel, ctrl+scroll or pinch gestures,
/// keeping the point under the cursor fixed.
/// Call this after the drawing area has been rendered, so that scroll areas inside
/// components get the first chance to consume the scroll.
pub fn view_scroll_zoom(ctx: &egui::Context, gui: &mut Gui) {
    let Some(pointer) = ctx.pointer_hover_pos() else {
        return;
    };
    if !gui.canvas_rect.contains(pointer) {
        return;
    }
    // Don't zoom when hovering menus, popups or tooltips drawn above the canvas
    if ctx
        .layer_id_at(pointer)
        .is_some_and(|l| l.order > egui::Order::Middle)
    {
        return;
    }
    let factor = ctx.input_mut(|i| {
        let scroll = i.smooth_scroll_delta.y;
        i.smooth_scroll_delta.y = 0.0;
        i.zoom_delta() * (scroll * ZOOM_SCROLL_SPEED).exp()
    });
    if factor != 1.0 {
        let scale = view_scale(gui);
        view_zoom_to(gui, scale * factor, Some(pointer));
        ctx.request_repaint();
    }
}

pub fn view_zoom_in_fn(gui: &mut Gui) {
    let scale = view_scale(gui);
    let next = ZOOM_LEVELS
        .into_iter()
        .find(|l| *l > scale * 1.01)
        .unwrap_or(ZOOM_MAX);
    view_zoom_to(gui, next, None);
}
pub fn view_zoom_out_fn(gui: &mut Gui) {
    let scale = view_scale(gui);
    let next = ZOOM_LEVELS
        .into_iter()
        .rev()
        .find(|l| *l < scale * 0.99)
        .unwrap_or(ZOOM_MIN);
    view_zoom_to(gui, next, None);
}
pub fn view_grid_toggle_fn(gui: &mut Gui) {
    if gui.editor_use {
        let editor = gui.editor.as_mut().unwrap();
        editor.grid.enable = !editor.grid.enable;
    }
}
pub fn view_grid_snap_toggle_fn(gui: &mut Gui) {
    if gui.editor_use {
        let editor = gui.editor.as_mut().unwrap();
        editor.grid.snap_enable = !editor.grid.snap_enable;
    }
}
pub fn control_play_toggle_fn(gui: &mut Gui) {
    if !gui.editor_use {
        gui.pause = !gui.pause;
    }
}
pub fn control_play_fn(gui: &mut Gui) {
    if !gui.editor_use {
        gui.pause = false;
        let _ = gui.simulator.as_mut().unwrap().set_running();
        //gui.simulator.as_mut().unwrap().run();
        //gui.pause = true;
    }
}
//pub fn step(gui: &mut Gui) {
//    if gui.simulator.as_ref().unwrap().running == true {
//        gui.simulator.as_mut().unwrap().clock();
//    }
//}
pub fn control_pause_fn(gui: &mut Gui) {
    if !gui.editor_use {
        gui.pause = true;
        let _ = gui.simulator.as_mut().unwrap().stop();
    }
}
pub fn control_reset_fn(gui: &mut Gui) {
    if !gui.editor_use {
        gui.simulator.as_mut().unwrap().reset();
        gui.pause = true;
    }
}
pub fn control_step_forward_fn(gui: &mut Gui) {
    if !gui.editor_use {
        gui.simulator.as_mut().unwrap().clock();
    }
}
pub fn control_step_back_fn(gui: &mut Gui) {
    if !gui.editor_use {
        gui.simulator.as_mut().unwrap().un_clock();
    }
}
pub fn editor_wire_mode_fn(gui: &mut Gui) {
    if gui.editor_use {
        let editor = gui.editor.as_mut().unwrap();
        match editor.editor_mode {
            EditorMode::Default | EditorMode::Input | EditorMode::Simulator => {
                editor.editor_mode = EditorMode::Wire;
            }
            EditorMode::Wire => {
                editor.editor_mode = EditorMode::Default;
            }
        }
        reset_wire_mode(&mut editor.wm);
    }
}
pub fn editor_escape_fn(gui: &mut Gui) {
    if gui.editor_use {
        let editor = gui.editor.as_mut().unwrap();
        editor.editor_mode = EditorMode::Default;
        reset_wire_mode(&mut editor.wm);
        reset_input_mode(&mut editor.im);
    }
}
