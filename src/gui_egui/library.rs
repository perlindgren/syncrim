use crate::common::{ComponentStore, EguiComponent};
use crate::gui_egui::gui::EguiExtra;
use crate::gui_egui::{
    editor::{Editor, EditorMode},
    editor_wire_mode::get_grid_snap,
    helper::{id_ports_of_all_components, offset_reverse_helper_pos2, unique_component_name},
};
use egui::{Context, CursorIcon, LayerId, PointerButton, Pos2, Rect, Response, Ui, Vec2};
use std::{collections::HashMap, path::PathBuf, rc::Rc};

pub struct InputMode {
    pub comp: Option<Rc<dyn EguiComponent>>,
    pub cursor_location: Pos2,
    pub library_contexts: HashMap<crate::common::Id, EguiExtra>,
}

pub fn input_mode(ctx: &Context, e: &mut Editor, cpr: Response, layer_id: Option<LayerId>) {
    let layer_id = layer_id.unwrap();
    if cpr.drag_started_by(PointerButton::Secondary) {
        e.editor_mode = EditorMode::Default;
        reset_input_mode(&mut e.im);
        return;
    }
    ctx.output_mut(|o| o.cursor_icon = CursorIcon::None);
    ctx.input_mut(|i| {
        e.im.cursor_location += i.pointer.delta();
    });
    let clip_rect = Rect {
        min: Pos2 {
            x: 0f32,
            y: e.offset.y,
        },
        max: Pos2 {
            x: f32::INFINITY,
            y: f32::INFINITY,
        },
    };
    let mut ui = Ui::new(
        ctx.to_owned(),
        layer_id,
        "input".into(),
        clip_rect,
        Rect::EVERYTHING,
    );
    let pos = if e.grid.enable && e.grid.snap_enable {
        match get_grid_snap(
            e.grid.snap_distance,
            offset_reverse_helper_pos2(e.im.cursor_location, e.scale, e.offset_and_pan),
            e.grid.size,
        ) {
            Some(s) => Vec2::new(s.x, s.y) * e.scale + e.offset + e.pan * e.scale,
            None => Vec2::new(e.im.cursor_location.x, e.im.cursor_location.y),
        }
    } else {
        Vec2::new(e.im.cursor_location.x, e.im.cursor_location.y)
    };
    e.im.comp.as_ref().unwrap().render(
        &mut ui,
        &mut EguiExtra {
            properties_window: false,
            size_rect: Rect::NAN,
            id_tmp: String::new(),
            pos_tmp: Pos2::ZERO,
            clicked: false,
        },
        None,
        pos,
        e.scale,
        clip_rect,
        e.editor_mode,
    );

    if cpr.drag_started_by(PointerButton::Primary) {
        add_comp_to_editor(e);
    }
}

pub fn reset_input_mode(im: &mut InputMode) {
    im.comp = None;
    im.cursor_location = Pos2::ZERO;
}

pub fn show_library(e: &mut Editor, ui: &mut Ui) {
    let mut padding = Vec2 {
        x: e.offset.x / 2f32,
        y: e.offset.y + 10f32,
    };
    let clip_rect = Rect {
        min: Pos2 {
            x: 0f32,
            y: e.offset.y,
        },
        max: Pos2 {
            x: e.offset.x,
            y: f32::INFINITY,
        },
    };
    for c in e.library.iter() {
        let size = c.top_padding();
        padding.y += size;
        let r_vec = c
            .render(
                ui,
                &mut EguiExtra {
                    properties_window: false,
                    size_rect: Rect::NAN,
                    id_tmp: c.get_id_ports().0,
                    pos_tmp: Pos2::ZERO,
                    clicked:false
                },
                None,
                padding,
                0.5f32,
                clip_rect,
                e.editor_mode,
            )
            .unwrap();
        let rect = r_vec[0].rect;
        for resp in r_vec {
            // Create new component
            if resp.drag_started_by(PointerButton::Primary) {
                e.editor_mode = EditorMode::Input;
                e.im.comp = Some(c.clone());
                ui.input_mut(|i| {
                    let origin = i.pointer.press_origin().unwrap();
                    e.im.cursor_location = origin;
                });
            } else if resp.drag_started_by(PointerButton::Secondary) {
                reset_input_mode(&mut e.im);
            }
        }
        padding.y = rect.max.y + 10f32;
    }
}

// todo: This should really just copy the component that's in e.input_comp
pub fn add_comp_to_editor(e: &mut Editor) {
    let mut pos = offset_reverse_helper_pos2(e.im.cursor_location, e.scale, e.offset_and_pan);
    if e.grid.enable && e.grid.snap_enable {
        if let Some(p) = get_grid_snap(e.grid.snap_distance, pos, e.grid.size) {
            pos = p;
        }
    }
    let id_ports = id_ports_of_all_components(&e.components);
    let cloned = e.im.comp.clone().unwrap();
    let id = unique_component_name(&id_ports, cloned.get_id_ports().0.as_str());
    let instance = cloned.dummy(&id, (pos.x, pos.y));
    e.contexts.insert(
        id.clone(),
        EguiExtra {
            properties_window: false,
            size_rect: Rect::NAN,
            id_tmp: id,
            pos_tmp: pos,
            clicked: false,
        },
    );
    e.components.push(*instance);
    let path = PathBuf::from("autosave.json");
    ComponentStore {
        store: e.components.clone(),
    }
    .save_file(&path);
}
