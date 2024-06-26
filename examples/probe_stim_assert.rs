use std::path::PathBuf;
#[cfg(feature = "gui-egui")]
use syncrim::gui_egui::editor::Library;
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
    fern::fern_setup,
};
fn main() {
    fern_setup();
    let cs = ComponentStore {
        store: vec![
            ProbeStim::rc_new("stim", (100.0, 100.0), vec![0, 1, 2, 3, 1337]),
            ProbeAssert::rc_new(
                "assert",
                (200.0, 100.0),
                Input::new("stim", "out"),
                vec![0, 1, 2, 3, 42],
            ),
        ],
    };

    let path = PathBuf::from("probe_stim_assert.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path, Library::default()).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
