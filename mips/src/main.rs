#[allow(unused_imports)]
use mips::components::*;
use syncrim::{common::ComponentStore, gui_vizia::gui};

fn main() {
    let cs = ComponentStore::load_file("mips.json");
    gui(&cs);
}
