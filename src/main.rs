use syncrim::{common::ComponentStore, gui_vizia::gui};

fn main() {
    let cs = ComponentStore::load_file("model.json");
    gui(&cs);
}
