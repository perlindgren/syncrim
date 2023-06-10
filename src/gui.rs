// use vizia::fonts::icons_names::{DOWN, MINUS, UP};
use vizia::prelude::*;
// use vizia::vg::{Paint, Path};

use crate::common::{ComponentStore, SimState, Simulator};
// use crate::components::RegisterView;

use std::rc::Rc;

#[derive(Lens)]
struct Gui {
    simulator: Simulator,
    state: SimState,
}

enum GuiEvent {
    Clock,
}

impl<'a> Model for Gui {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, meta| match app_event {
            GuiEvent::Clock => self.simulator.clock(&mut self.state),
        });
    }
}

pub fn gui(cs: &ComponentStore) {
    let (simulator, mut sim_state) = Simulator::new(cs);
    println!("--- SimState\n {:#?}", sim_state.lens_values);
    sim_state.lens_values[1] = 1;

    Application::new(move |cx| {
        let gui = Gui {
            simulator,
            state: sim_state,
        }
        .build(cx);

        for c in Gui::simulator.then(Simulator::ordered_components).get(cx) {
            c.view(cx, &|cx| Gui::state.get(cx).lens_values);
        }

        Label::new(
            cx,
            Gui::state
                .then(SimState::lens_values)
                .map(|v| format!("{:?}", v)),
        );

        Button::new(
            cx,
            |ex| ex.emit(GuiEvent::Clock),
            |cx| Label::new(cx, "Clock"),
        );
    })
    .run();
}
