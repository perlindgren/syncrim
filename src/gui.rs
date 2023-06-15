use crate::common::{ComponentStore, SimState, Simulator};
use std::rc::Rc;
use vizia::prelude::*;
use vizia::vg::{Paint, Path};

#[derive(Lens)]
pub struct Gui {
    pub simulator: Rc<Simulator>,
    pub state: SimState,
}

enum GuiEvent {
    Clock,
}

impl Model for Gui {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _meta| match app_event {
            GuiEvent::Clock => self.simulator.clock(&mut self.state),
        });
    }
}

pub fn gui(cs: &ComponentStore) {
    let (simulator, sim_state) = Simulator::new(cs);
    println!("--- SimState\n {:#?}", sim_state.lens_values);
    let simulator = Rc::new(simulator);
    Application::new(move |cx| {
        Gui {
            simulator: simulator.clone(),
            state: sim_state,
        }
        .build(cx);

        // Grid
        Grid::new(cx);

        for c in &simulator.ordered_components {
            c.view(cx, simulator.clone());
        }

        // a label to display the raw state for debugging purpose
        Label::new(
            cx,
            Gui::state
                .then(SimState::lens_values)
                .map(|v| format!("Raw state {:?}", v)),
        );

        Button::new(
            cx,
            |ex| ex.emit(GuiEvent::Clock),
            |cx| Label::new(cx, "Clock"),
        );
    })
    .run();
}

struct Grid {}

impl Grid {
    // create view
    fn new(cx: &mut Context) {
        println!("---- Create Grid ");
        View::build(GridView {}, cx, |cx| {});
    }
}

struct GridView {}

impl View for GridView {
    fn element(&self) -> Option<&'static str> {
        Some("Grid")
    }

    // draw operates on native pixels
    // bounds is given in scaled format
    fn draw(&self, cx: &mut DrawContext<'_>, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        let scale = cx.scale_factor();
        println!("Grid draw {:?}, {}", bounds, cx.scale_factor());

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

        canvas.stroke_path(&mut path, &paint);
    }
}
