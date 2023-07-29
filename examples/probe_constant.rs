use std::path::PathBuf;
use syncrim::{common::ComponentStore, components::*, fern::fern_setup};

fn main() {
    fern_setup();

    let cs = ComponentStore {
        store: vec![Constant::rc_new("constant", (100.0, 100.0), 0)],
    };

    let path = PathBuf::from("constant.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
