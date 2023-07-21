use std::{path::PathBuf, rc::Rc};
use syncrim::{
    common::{ComponentStore, Input, Signal, SignalUnsigned},
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
            Rc::new(Constant {
                id: "data".to_string(),
                pos: (100.0, 100.0),
                value: 3.into(),
            }),
            Rc::new(Constant {
                id: "addr".to_string(),
                pos: (120.0, 100.0),
                value: 4.into(),
            }),
            Rc::new(Constant {
                id: "ctrl".to_string(),
                pos: (140.0, 100.0),
                value: (MemCtrl::Write as SignalUnsigned).into(),
            }),
            Rc::new(Constant {
                id: "sext".to_string(),
                pos: (160.0, 100.0),
                value: (false as SignalUnsigned).into(),
            }),
            Rc::new(Constant {
                id: "size".to_string(),
                pos: (180.0, 100.0),
                value: 1.into(), // byte
            }),
            // Wires
            Rc::new(Wire {
                id: "w1".to_string(),
                pos: vec![(100.0, 110.0), (100.0, 150.0)],
                input: Input::new("data", "out"),
            }),
            Rc::new(Wire {
                id: "w2".to_string(),
                pos: vec![(120.0, 110.0), (120.0, 150.0)],
                input: Input::new("addr", "out"),
            }),
            Rc::new(Wire {
                id: "w3".to_string(),
                pos: vec![(140.0, 110.0), (140.0, 150.0)],
                input: Input::new("sext", "out"),
            }),
            Rc::new(Wire {
                id: "w4".to_string(),
                pos: vec![(160.0, 110.0), (160.0, 150.0)],
                input: Input::new("size", "out"),
            }),
            Rc::new(Wire {
                id: "w5".to_string(),
                pos: vec![(220.0, 110.0), (220.0, 150.0)],
                input: Input::new("mem", "data"),
            }),
            Rc::new(Wire {
                id: "w6".to_string(),
                pos: vec![(240.0, 110.0), (240.0, 150.0)],
                input: Input::new("mem", "err"),
            }),
            // probes
            Rc::new(Probe {
                id: "out".to_string(),
                pos: (220.0, 100.0),
                input: Input::new("mem", "data"),
            }),
            Rc::new(Probe {
                id: "err".to_string(),
                pos: (240.0, 100.0),
                input: Input::new("mem", "err"),
            }),
        ],
    };

    let path = PathBuf::from("mem.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
