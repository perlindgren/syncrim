use syncrim::{common::ComponentStore, gui::gui};

fn main() {
    let cs = ComponentStore::load_file("mips.json");
    gui(&cs);
}
