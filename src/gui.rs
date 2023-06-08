// use vizia::fonts::icons_names::{DOWN, MINUS, UP};
use vizia::prelude::*;
// use vizia::vg::{Paint, Path};
use crate::common::ComponentStore;
use crate::common::Simulator;
// use crate::components::RegisterView;

pub fn gui(cs: &ComponentStore) {
    let (simulator, mut sim_state) = Simulator::new(cs);
    println!("--- SimState\n {:#?}", sim_state.lens_values);

    Application::new(move |cx| {
        for c in &simulator.ordered_components {
            c.view(cx, &sim_state);
        }

        // simulate one clock
        simulator.clock(&mut sim_state);
    })
    .run();
}
