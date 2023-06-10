// use vizia::fonts::icons_names::{DOWN, MINUS, UP};
use vizia::prelude::*;
// use vizia::vg::{Paint, Path};

use crate::common::{ComponentStore, SimState, Simulator};
// use crate::components::RegisterView;

#[derive(Lens)]
pub struct Gui {
    pub simulator: Simulator,
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

    Application::new(move |cx| {
        Gui {
            simulator,
            state: sim_state,
        }
        .build(cx);

        for c in Gui::simulator.then(Simulator::ordered_components).get(cx) {
            c.view(cx, Gui::state);
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
