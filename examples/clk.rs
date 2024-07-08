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
            MIPSCLK::rc_new("clk", (200.0, 120.0), Input::new("reg", "out")),
            //Constant::rc_new("c1", (60.0, 100.0), 10),
            // ProbeEdit::rc_new("c1", (60.0, 100.0)),
            Register::rc_new("reg", (100.0, 140.0), Input::new("clk", "out")),
            Probe::rc_new("p1", (270.0, 120.0), Input::new("clk", CLK_OUT_ID)),
            // Wire::rc_new(
            //     "w1",
            //     vec![(110.0, 140.0), (180.0, 100.0)],
            //     Input::new("c1", "out"),
            // ),
            // Wire::rc_new(
            //     "w2",
            //     vec![(220.0, 120.0), (260.0, 120.0)],
            //     Input::new("clk", CLK_OUT_ID),
            // ),
        ],
    };

    let path = PathBuf::from("add.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path, Library::default()).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
