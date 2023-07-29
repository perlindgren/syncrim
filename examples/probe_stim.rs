use std::path::PathBuf;
use syncrim::{common::ComponentStore, components::*, fern::fern_setup};

fn main() {
    fern_setup();
    let cs = ComponentStore {
        store: vec![ProbeStim::rc_new("stim", (100.0, 100.0), vec![42, 1, 2])],
    };

    let path = PathBuf::from("probe_stim.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
