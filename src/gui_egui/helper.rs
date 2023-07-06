pub fn offset_helper(xy: (f32, f32), scale: f32, offset: egui::Vec2) -> egui::Pos2 {
    return egui::Pos2 {
        x: xy.0 * scale,
        y: xy.1 * scale,
    } + offset;
}
