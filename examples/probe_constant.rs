use std::path::PathBuf;
#[cfg(feature = "gui-egui")]
use syncrim::gui_egui::editor::Library;
use syncrim::{common::ComponentStore, components::*, fern::fern_setup};
fn main() {
    fern_setup();

    let cs = ComponentStore {
        store: vec![
            Constant::rc_new("c1", (100.0, 100.0), 1),
            Constant::rc_new("c2", (100.0, 140.0), 2),
        ],
    };

    let path = PathBuf::from("constant.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path, Library::default()).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
