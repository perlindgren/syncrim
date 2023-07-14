use egui::{Pos2, Rect, Vec2};

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

//pub fn
