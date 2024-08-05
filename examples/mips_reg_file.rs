use std::path::PathBuf;
use std::rc::Rc;
use syncrim::common::EguiComponent;
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
            RegFile::rc_new(
                "reg_file",
                (200.0, 200.0),
                Input::new("c0", "out"),
                Input::new("c1", "out"),
                Input::new("c2", "out"),
                Input::new("c3", "out"),
                Input::new("c4", "out"),
                true,
            ),
            ProbeEdit::rc_new("c0", (60.0, 100.0)),
            ProbeEdit::rc_new("c1", (60.0, 140.0)),
            ProbeEdit::rc_new("c2", (60.0, 160.0)),
            ProbeEdit::rc_new("c3", (60.0, 200.0)),
            ProbeEdit::rc_new("c4", (60.0, 240.0)),
            Probe::rc_new(
                "p1",
                (270.0, 120.0),
                Input::new("reg_file", REG_FILE_RD1_OUT_ID),
            ),
            Probe::rc_new(
                "p2",
                (270.0, 160.0),
                Input::new("reg_file", REG_FILE_RD2_OUT_ID),
            ),
        ],
    };

    let cs = autowire(cs);

    let path = PathBuf::from("add.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path, Library::default()).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
