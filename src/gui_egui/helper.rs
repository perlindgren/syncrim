use crate::common::{Components, Ports};
use crate::gui_egui::editor::EditorMode;
use egui::{Pos2, Rect, Sense, Vec2};

pub fn offset_reverse_helper_pos2(xy: Pos2, scale: f32, offset: Vec2) -> Pos2 {
    egui::Pos2 {
        x: (xy.x - offset.x) * scale,
        y: (xy.y - offset.y) * scale,
    }
}

pub fn offset_reverse_helper(xy: (f32, f32), scale: f32, offset: Vec2) -> Pos2 {
    egui::Pos2 {
        x: (xy.0 - offset.x) * scale,
        y: (xy.1 - offset.y) * scale,
    }
}

pub fn offset_helper(xy: (f32, f32), scale: f32, offset: Vec2) -> Pos2 {
    egui::Pos2 {
        x: xy.0 * scale,
        y: xy.1 * scale,
    } + offset
}

pub fn out_of_bounds(request: Rect, clip_rect: Rect) -> Rect {
    let mut rect = Rect::NOTHING;
    if request.max.x < clip_rect.min.x
        || request.max.y < clip_rect.min.y
        || request.min.x > clip_rect.max.x
        || request.min.y > clip_rect.max.y
    {
        return rect;
    }
    rect = request;
    if request.max.x > clip_rect.max.x {
        rect.max.x = clip_rect.max.x;
    }
    if request.max.y > clip_rect.max.y {
        rect.max.y = clip_rect.max.y;
    }
    if request.min.x < clip_rect.min.x {
        rect.min.x = clip_rect.min.x;
    }
    if request.min.y < clip_rect.min.y {
        rect.min.y = clip_rect.min.y;
    }
    rect
}

pub fn unique_component_name(id_ports: &[(crate::common::Id, Ports)], id: &str) -> String {
    let mut new_id: String = id.into();
    let mut contains_id = true;
    while contains_id {
        contains_id = false;
        for c in id_ports {
            let id = c.0.clone();
            if id == new_id {
                contains_id = true;
                // todo: make this fancier
                new_id.push('1');
                break;
            }
        }
    }
    new_id
}

pub fn id_ports_of_all_components(cs: &Components) -> Vec<(crate::common::Id, Ports)> {
    let mut v = vec![];
    for c in cs.iter() {
        v.push(c.get_id_ports())
    }
    v
}

pub fn editor_mode_to_sense(editor_mode: EditorMode) -> Sense {
    match editor_mode {
        EditorMode::Wire => Sense {
            click: false,
            drag: false,
            focusable: false,
        },
        _ => Sense {
            click: true,
            drag: true,
            focusable: true,
        },
    }
}
