use std::path::PathBuf;
use std::rc::Rc;
use syncrim::common::EguiComponent;
#[cfg(feature = "gui-egui")]
use syncrim::gui_egui::editor::Library;
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
    fern::fern_setup,
    mips_helper_functions::autowire,
};

fn main() {
    fern_setup();
    let cs = ComponentStore {
        store: vec![
            ProbeEdit::rc_new("c0", (50.0, 100.0)),
            // register that holds instr addr
            Register::rc_new("reg", (150.0, 100.0), Input::new("c0", "out")),
            // step addr from reg by 4
            Constant::rc_new("+4", (200.0, 150.0), 4),
            Add::rc_new(
                "pc+4",
                (250.0, 100.0),
                Input::new("reg", "out"),
                Input::new("+4", "out"),
            ),
            //
            //
            ProbeEdit::rc_new("ctrl", (200.0, 250.0)),
            // MUX to choose what intruction addr to choose from, branch jump, reg, pc+4
            Mux::rc_new(
                "mux",
                (200.0, 200.0),
                Input::new("ctrl", "out"),
                vec![
                    // Input::new("clk", CLK_OUT_ID),
                    //Input::new("c2", "out"),
                    Input::new("jump_merge", MERGE_OUT_ID),
                    Input::new("pc+4", CLK_OUT_ID),
                ],
            ), //
            //
            // merges to find out jump location
            Constant::rc_new("c1", (100.0, 350.0), 0),
            JumpMerge::rc_new(
                "jump_merge",
                (100.0, 300.0),
                Input::new("reg", "out"), //input from reg before pc+4
                Input::new("c1", "out"),  //input from instruction mem
            ),
            //
            //
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
