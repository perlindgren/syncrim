use std::path::PathBuf;
#[cfg(feature = "gui-egui")]
use syncrim::gui_egui::editor::Library;
use syncrim::{
    autowire::autowire,
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
            Register::rc_new("reg", (150.0, 250.0), Input::new("clk", "out")),
            //Register::rc_new("reg2", (250.0, 250.0), Input::new("reg", "reg_out")),
            // Probe::rc_new("p1", (270.0, 120.0), Input::new("clk", CLK_OUT_ID)),
        ],
    };
    #[cfg(feature = "gui-egui")]
    let cs = autowire(cs);

    let path = PathBuf::from("add.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path, Library::default()).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
