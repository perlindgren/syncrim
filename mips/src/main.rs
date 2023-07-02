#[allow(unused_imports)]
use mips::components::*;
use std::path::PathBuf;
use syncrim::{common::ComponentStore, gui_vizia::gui};

fn main() {
    let path = PathBuf::from("mips.json");
    let cs = ComponentStore::load_file(&path);
    gui(&cs, &path);
}
