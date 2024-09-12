use mips_lib::components::*;
use std::path::PathBuf;
use syncrim::autowire::autowire;
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
            InstrSplit::rc_new(
                "instruction_splitter",
                (200.0, 120.0),
                Input::new("c0", "out"),
            ),
            ProbeEdit::rc_new("c0", (60.0, 100.0)),
            Probe::rc_new(
                "op",
                (270.0, 120.0),
                Input::new("instruction_splitter", INSTRUCTION_SPLITTER_OP_ID),
            ),
            Probe::rc_new(
                "rs",
                (270.0, 140.0),
                Input::new("instruction_splitter", INSTRUCTION_SPLITTER_RS_ID),
            ),
            Probe::rc_new(
                "rt",
                (270.0, 160.0),
                Input::new("instruction_splitter", INSTRUCTION_SPLITTER_RT_ID),
            ),
            Probe::rc_new(
                "rd",
                (270.0, 180.0),
                Input::new("instruction_splitter", INSTRUCTION_SPLITTER_RD_ID),
            ),
            Probe::rc_new(
                "shamt",
                (270.0, 200.0),
                Input::new("instruction_splitter", INSTRUCTION_SPLITTER_SHAMT_ID),
            ),
            Probe::rc_new(
                "funct",
                (270.0, 220.0),
                Input::new("instruction_splitter", INSTRUCTION_SPLITTER_FUNCT_ID),
            ),
            Probe::rc_new(
                "immediate",
                (270.0, 240.0),
                Input::new("instruction_splitter", INSTRUCTION_SPLITTER_IMMEDIATE_ID),
            ),
            Probe::rc_new(
                "target",
                (270.0, 260.0),
                Input::new("instruction_splitter", INSTRUCTION_SPLITTER_TARGET_ID),
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
