use syncrim::{
    common::{ComponentStore, Input},
     components::*,
};
use std::rc::Rc;
use std::path::PathBuf;
use riscv::components::*;
fn main() {
    fern_setup_riscv();
    let cs = ComponentStore {
        store: vec![
            Probe::rc_new(
                "push_rdy_o",
                (500.0, 300.0),
                Input::new("antiq", "push_rdy_o"),
            ),
            Probe::rc_new(
                "pop_rdy_o",
                (500.0, 325.0),
                Input::new("antiq", "pop_rdy_o"),
            ),
            Probe::rc_new(
                "drop_rdy_o",
                (500.0, 350.0),
                Input::new("antiq", "drop_rdy_o"),
            ),
            Probe::rc_new(
                "full_o",
                (500.0, 375.0),
                Input::new("antiq", "full_o"),
            ),
            Probe::rc_new(
                "empty_o",
                (500.0, 400.0),
                Input::new("antiq", "empty_o"),
            ),
            Probe::rc_new(
                "cnt_o",
                (500.0, 425.0),
                Input::new("antiq", "cnt_o"),
            ),
            Probe::rc_new(
                "data_o",
                (500.0, 450.0),
                Input::new("antiq", "data_o"),
            ),
            Probe::rc_new(
                "peek_vld_o",
                (500.0, 475.0),
                Input::new("antiq", "peek_vld_o"),
            ),
            Probe::rc_new(
                "peek_data_o",
                (500.0, 500.0),
                Input::new("antiq", "peek_data_o"),
            ),
            Probe::rc_new(
                "overflow_o",
                (500.0, 525.0),
                Input::new("antiq", "overflow_o"),
            ),
            Probe::rc_new(
                "data_overflow_o",
                (500.0, 550.0),
                Input::new("antiq", "data_overflow_o"),
            ),
            ProbeEdit::rc_new(
               "pop_i",
                (300.0, 300.0),
            ),
            ProbeEdit::rc_new(
               "drop_i",
                (300.0, 325.0),
            ),
            ProbeEdit::rc_new(
               "push_i",
                (300.0, 350.0),
            ),
            ProbeEdit::rc_new(
               "drop_id_i",
                (300.0, 375.0),
            ),
            ProbeEdit::rc_new(
               "push_id_i",
                (300.0, 400.0),
            ),
            ProbeEdit::rc_new(
               "data_i",
                (300.0, 425.0),
            ),
            ProbeEdit::rc_new(
                "sysclk",
                (300.0, 450.0),
            ),

            Rc::new(Antiq::new(
                "antiq",
                (400.0, 400.0),
                Input::new("pop_i", "out"),
                Input::new("drop_i", "out"),
                Input::new("push_i", "out"),
                Input::new("drop_id_i", "out"),
                Input::new("push_id_i", "out"),
                Input::new("data_i", "out"),
                Input::new("sysclk", "out"),
 
                3,
                32,
            )),
        ],
    };

    let path = PathBuf::from("riscv.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
#[allow(unused_imports)]
use log::LevelFilter;
fn fern_setup_riscv() {
    let f = fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        // Add blanket level filter -
        // .level(log::LevelFilter::Debug);
        // .level_for(
        //     //   "syncrim::gui_vizia::components::mem",
        //     "riscv::gui_vizia::components::instr_mem",
        //     log::LevelFilter::Trace,
        // )
        .level(log::LevelFilter::Error);

    // - and per-module overrides
    #[cfg(feature = "gui-vizia")]
    let f = f
        //.level_for("riscv::components::instr_mem", LevelFilter::Trace)
        .level_for("riscv::components::antiq", LevelFilter::Trace);
        //.level_for("riscv::components::mem", LevelFilter::Trace)
        //.level_for("syncrim::simulator", LevelFilter::Trace);

    f
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log").unwrap())
        // Apply globally
        .apply()
        .unwrap()
}
