use syncrim::{common::ComponentStore, gui::gui};

fn main() {
    let cs = ComponentStore::load_file("model.json");
    gui(&cs);
}
