use std::{path::PathBuf, rc::Rc};
use syncrim::{
    common::{ComponentStore, Input, SignalUnsigned},
    components::*,
    fern::fern_setup,
};

fn main() {
    fern_setup();
    let cs = ComponentStore {
        store: vec![
            Rc::new(Mem::new(
                "mem",
                (180.0, 200.0),
                200.0,
                100.0,
                // configuration
                true,
                // ports
                Input::new("data", "out"),
                Input::new("addr", "out"),
                Input::new("ctrl", "out"),
                Input::new("sext", "out"),
                Input::new("size", "out"),
                // memory
                Memory::new(),
                // later history... tbd
            )),
            Rc::new(Constant::new("data", (100.0, 100.0), 3)),
            Rc::new(Constant::new("addr", (120.0, 100.0), 4)),
            Rc::new(Constant::new(
                "ctrl",
                (140.0, 100.0),
                MemCtrl::Write as SignalUnsigned,
            )),
            Rc::new(Constant::new(
                "sext",
                (160.0, 100.0),
                false as SignalUnsigned,
            )),
            Rc::new(Constant::new(
                "size",
                (180.0, 100.0),
                1, // byte
            )),
            // Wires
            Rc::new(Wire::new(
                "w1",
                vec![(100.0, 110.0), (100.0, 150.0)],
                Input::new("data", "out"),
            )),
            Rc::new(Wire::new(
                "w2",
                vec![(120.0, 110.0), (120.0, 150.0)],
                Input::new("addr", "out"),
            )),
            Rc::new(Wire::new(
                "w3",
                vec![(140.0, 110.0), (140.0, 150.0)],
                Input::new("sext", "out"),
            )),
            Rc::new(Wire::new(
                "w4",
                vec![(160.0, 110.0), (160.0, 150.0)],
                Input::new("size", "out"),
            )),
            Rc::new(Wire::new(
                "w5",
                vec![(220.0, 110.0), (220.0, 150.0)],
                Input::new("mem", "data"),
            )),
            Rc::new(Wire::new(
                "w6",
                vec![(240.0, 110.0), (240.0, 150.0)],
                Input::new("mem", "err"),
            )),
            // probes
            Rc::new(Probe::new("out", (220.0, 100.0), Input::new("mem", "data"))),
            Rc::new(Probe::new("err", (240.0, 100.0), Input::new("mem", "err"))),
        ],
    };

    let path = PathBuf::from("data_mem.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
