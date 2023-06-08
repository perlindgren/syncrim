// use vizia::fonts::icons_names::{DOWN, MINUS, UP};
use vizia::prelude::*;
// use vizia::vg::{Paint, Path};
use crate::common::{Component, ComponentStore};
use crate::components::RegisterView;

pub fn gui(cs: ComponentStore) {
    Application::new(|cx| {
        for c in cs.store {
            c.view(cx);
        }
    })
    .run();
}
