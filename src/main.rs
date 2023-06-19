use syncrim::{common::ComponentStore, egui::gui};

fn main() {
    let cs = ComponentStore::load_file("model.json");
    gui(&cs);
}
