use mips_lib::components::*;
use std::{path::PathBuf, rc::Rc};
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
            ProbeEdit::rc_new("address", (60.0, 100.0)),
            ProbeEdit::rc_new("data", (60.0, 140.0)),
            ProbeEdit::rc_new("WE", (200.0, 50.0)),
            Rc::new(MipsTimer::new(
                "timer",
                (200.0, 120.0),
                Input::new("address", "out"),
                Input::new("data", "out"),
                Input::new("WE", "out"),
            )),
            Probe::rc_new(
                "data_out",
                (320.0, 120.0),
                Input::new("timer", TIMER_DATA_OUT_ID
            ),
            ),
            Probe::rc_new(
                "interrupt",
                (320.0, 140.0),
                Input::new("timer", TIMER_INTERRUPT_OUT_ID),
            ),
        ],
    };

    let path = PathBuf::from("timer.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    {
        use syncrim::autowire::autowire;
        let cs = autowire(cs);
        cs.save_file(&path);
        syncrim::gui_egui::gui(cs, &path, Library::default()).ok();
    }

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
