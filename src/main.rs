use syncrim::common::ComponentStore;

fn main() {
    let cs = ComponentStore::load_file("model.json");
    syncrim::egui_::gui(&cs);
}
