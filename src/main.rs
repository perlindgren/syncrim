use syncrim::common::ComponentStore;

fn main() {
    let cs = ComponentStore::load_file("model.json");
    syncrim::gui_egui::egui::gui(&cs);
}
