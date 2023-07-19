use crate::common::Components;
use egui::{Pos2, Rect, Response, Vec2};

pub struct EditorRenderReturn {
    pub delete: bool,
    pub resp: Option<Response>,
}

pub fn offset_helper(xy: (f32, f32), scale: f32, offset: Vec2) -> Pos2 {
    egui::Pos2 {
        x: xy.0 * scale,
        y: xy.1 * scale,
    } + offset
}

pub fn out_of_bounds(request: Rect, clip_rect: Rect) -> Rect {
    let mut rect = Rect::NOTHING;
    if request.max.x < clip_rect.min.x || request.max.y < clip_rect.min.y {
        return rect;
    } else if request.min.x > clip_rect.max.x || request.min.y > clip_rect.max.y {
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
    return rect;
}

pub fn unique_component_name(cs: &Components, id: &str) -> String {
    let mut new_id: String = id.into();
    let mut contains_id = true;
    while contains_id {
        contains_id = false;
        for c in cs.iter() {
            let id = match c.try_borrow_mut() {
                Ok(a) => a.get_id_ports().0,
                Err(e) => String::from(""),
            };
            if id == new_id {
                contains_id = true;
                // todo: make this fancier
                new_id.push_str("1");
                break;
            }
        }
    }
    String::from(new_id)
}

// todo: Create the properties window the same way every time for the different components

//pub fn
