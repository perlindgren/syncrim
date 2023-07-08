use std::{path::PathBuf, rc::Rc};
use syncrim::{
    common::{ComponentStore, Input, Signal},
    components::*,
};

fn main() {
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
                data: Input::new("data", 0),
                addr: Input::new("addr", 0),
                ctrl: Input::new("ctrl", 0),
                sign_extend: Input::new("sext", 0),
                size: Input::new("size", 0),

                // memory
                memory: Memory::new(),
                // later history... tbd
            }),
            Rc::new(Constant {
                id: "data".to_string(),
                pos: (100.0, 100.0),
                value: 3,
            }),
            Rc::new(Constant {
                id: "addr".to_string(),
                pos: (120.0, 100.0),
                value: 4,
            }),
            Rc::new(Constant {
                id: "ctrl".to_string(),
                pos: (140.0, 100.0),
                value: MemCtrl::Write as Signal,
            }),
            Rc::new(Constant {
                id: "sext".to_string(),
                pos: (160.0, 100.0),
                value: false as Signal,
            }),
            Rc::new(Constant {
                id: "size".to_string(),
                pos: (180.0, 100.0),
                value: 1, // byte
            }),
            // Wires
            Rc::new(Wire {
                id: "w1".to_string(),
                pos: (100.0, 110.0),
                delta: (0.0, 40.0),
                input: Input::new("data", 0),
            }),
            Rc::new(Wire {
                id: "w2".to_string(),
                pos: (120.0, 110.0),
                delta: (0.0, 40.0),
                input: Input::new("addr", 0),
            }),
            Rc::new(Wire {
                id: "w3".to_string(),
                pos: (140.0, 110.0),
                delta: (0.0, 40.0),
                input: Input::new("sext", 0),
            }),
            Rc::new(Wire {
                id: "w4".to_string(),
                pos: (160.0, 110.0),
                delta: (0.0, 40.0),
                input: Input::new("size", 0),
            }),
            Rc::new(Wire {
                id: "w5".to_string(),
                pos: (220.0, 110.0),
                delta: (0.0, 40.0),
                input: Input::new("mem", 0),
            }),
            Rc::new(Wire {
                id: "w6".to_string(),
                pos: (240.0, 110.0),
                delta: (0.0, 40.0),
                input: Input::new("mem", 1),
            }),
            // probes
            Rc::new(Probe {
                id: "out".to_string(),
                pos: (220.0, 100.0),
                input: Input::new("mem", 0),
            }),
            Rc::new(Probe {
                id: "err".to_string(),
                pos: (240.0, 100.0),
                input: Input::new("mem", 1),
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
