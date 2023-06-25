use vizia::{
    prelude::*,
    vg::{Paint, Path},
};

pub struct Grid {}

impl Grid {
    pub(crate) fn new<F>(cx: &mut Context, content: F) -> Handle<'_, Self>
    where
        F: FnOnce(&mut Context),
    {
        View::build(Grid {}, cx, |cx| content(cx))
    }
}

impl View for Grid {
    fn element(&self) -> Option<&'static str> {
        Some("Grid")
    }

    // draw operates on native pixels
    // bounds is given in scaled format
    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        let scale = cx.scale_factor();
        // println!("Grid draw {:?}, {}", bounds, cx.scale_factor());

        let unscaled_height = bounds.height() / scale;
        let unscaled_width = bounds.width() / scale;

        let rows: usize = (unscaled_height / 20.0).round() as usize;
        let columns: usize = (unscaled_width / 20.0).round() as usize;

        let mut path = Path::new();
        let mut paint = Paint::color(vizia::vg::Color::rgbaf(0.0, 0.0, 1.0, 0.1));

        paint.set_line_width(cx.logical_to_physical(1.0));

        for r in 0..rows {
            path.move_to(
                bounds.left() + 0.5,
                bounds.top() + r as f32 * 20.0 * scale + 0.5,
            );
            path.line_to(
                bounds.right() + 0.5,
                bounds.top() + r as f32 * 20.0 * scale + 0.5,
            );
        }

        for c in 0..columns {
            path.move_to(
                bounds.left() + c as f32 * 20.0 * scale + 0.5,
                bounds.top() + 0.5,
            );
            path.line_to(
                bounds.left() + c as f32 * 20.0 * scale + 0.5,
                bounds.bottom() + 0.5,
            );
        }

        canvas.stroke_path(&path, &paint);
    }
}
