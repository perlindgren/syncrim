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
            Rc::new(Mem {
                id: "mem".to_string(),
                pos: (180.0, 200.0),

                width: 200.0,
                height: 100.0,

                // configuration
                big_endian: true,

                // ports
                data: Input::new("data", "out"),
                addr: Input::new("addr", "out"),
                ctrl: Input::new("ctrl", "out"),
                sign: Input::new("sext", "out"),
                size: Input::new("size", "out"),

                // memory
                memory: Memory::new(),
                // later history... tbd
            }),
            Constant::rc_new("data", (100.0, 100.0), 3),
            Constant::rc_new("addr", (120.0, 100.0), 4),
            Constant::rc_new("ctrl", (140.0, 100.0), MemCtrl::Write as SignalUnsigned),
            Constant::rc_new("sext", (160.0, 100.0), false),
            Constant::rc_new("size", (180.0, 100.0), 1), // byte
            // Wires
            Wire::rc_new(
                "w1",
                vec![(100.0, 110.0), (100.0, 150.0)],
                Input::new("data", "out"),
            ),
            Wire::rc_new(
                "w2",
                vec![(120.0, 110.0), (120.0, 150.0)],
                Input::new("addr", "out"),
            ),
            Wire::rc_new(
                "w3",
                vec![(140.0, 110.0), (140.0, 150.0)],
                Input::new("sext", "out"),
            ),
            Wire::rc_new(
                "w4",
                vec![(160.0, 110.0), (160.0, 150.0)],
                Input::new("size", "out"),
            ),
            Wire::rc_new(
                "w5",
                vec![(220.0, 110.0), (220.0, 150.0)],
                Input::new("mem", "data"),
            ),
            Wire::rc_new(
                "w6",
                vec![(240.0, 110.0), (240.0, 150.0)],
                Input::new("mem", "err"),
            ),
            // probes
            Probe::rc_new("out", (220.0, 100.0), Input::new("mem", "data")),
            Probe::rc_new("err", (240.0, 100.0), Input::new("mem", "err")),
        ],
    };

    let path = PathBuf::from("data_mem.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
