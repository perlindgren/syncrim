// An example MIPS model

use mips::components::*;
use std::path::PathBuf;
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
    fern::fern_setup,
};

fn main() {
    fern_setup();
    let cs = ComponentStore {
        store: vec![
            Add::rc_new(
                "add",
                (200.0, 120.0),
                Input::new("c1", "out"),
                Input::new("reg", "out"),
            ),
            Constant::rc_new("c1", (100.0, 100.0), 4),
            Register::rc_new("reg", (100.0, 140.0), Input::new("add", "out")),
            Wire::rc_new(
                "c1_to_add_a",
                vec![(110.0, 100.0), (180.0, 100.0)],
                Input::new("c1", "out"),
            ),
            Wire::rc_new(
                "reg_to_add_b",
                vec![(110.0, 140.0), (180.0, 140.0)],
                Input::new("reg", "out"),
            ),
            Wire::rc_new(
                "add_to_reg",
                vec![
                    (220.0, 120.0),
                    (260.0, 120.0),
                    (260.0, 60.0),
                    (60.0, 60.0),
                    (60.0, 140.0),
                    (90.0, 140.0),
                ],
                Input::new("add", "out"),
            ),
            Wire::rc_new(
                "pc_to_down",
                vec![(140.0, 140.0), (140.0, 180.0), (350.0, 180.0)],
                Input::new("reg", "out"),
            ),
            InstrMem::rc_new(
                "instr_mem",
                (400.0, 150.0),
                Input::new("reg", "out"),
                // fake instructions just to show the relation between input address and instruction
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            ),
            Wire::rc_new(
                "w8",
                vec![(450.0, 120.0), (520.0, 120.0)],
                Input::new("instr_mem", "out"),
            ),
            Probe::rc_new("p1", (280.0, 160.0), Input::new("reg", "out")),
            Probe::rc_new("p2", (500.0, 100.0), Input::new("instr_mem", "out")),
        ],
    };

    let path = PathBuf::from("mips.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
