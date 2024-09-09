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
            ProbeEdit::rc_new("A", (100.0, 100.0)),
            ProbeEdit::rc_new("B", (100.0, 140.0)),
            And::rc_new(
                "and",
                (150.0, 120.0),
                Input::new("A", PROBE_EDIT_OUT_ID),
                Input::new("B", PROBE_EDIT_OUT_ID),
            ),
            Probe::rc_new("probe", (250.0, 100.0), Input::new("and", AND_OUT_ID)),
        ],
    };

    let path = PathBuf::from("and.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path, Library::default()).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
