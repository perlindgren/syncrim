// An example MIPS model

use mips::components::*;
use std::{path::PathBuf, rc::Rc};
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
};

fn main() {
    let cs = ComponentStore {
        store: vec![
            Rc::new(Add {
                id: "add".to_string(),
                pos: (200.0, 120.0),
                a_in: Input::new("c1", "out"),
                b_in: Input::new("reg", "out"),
            }),
            Rc::new(Constant {
                id: "c1".to_string(),
                pos: (100.0, 100.0),
                value: 4,
            }),
            Rc::new(Register {
                id: "reg".to_string(),
                pos: (100.0, 140.0),
                r_in: Input::new("add", "out"),
            }),
            Rc::new(Wire {
                id: "c1_to_add_a".to_string(),
                pos: (110.0, 100.0),
                delta: (70.0, 0.0),
                input: Input::new("c1", "out"),
            }),
            Rc::new(Wire {
                id: "reg_to_add_b".to_string(),
                pos: (110.0, 140.0),
                delta: (70.0, 0.0),
                input: Input::new("reg", "out"),
            }),
            Rc::new(Wire {
                id: "add_to_right".to_string(),
                pos: (220.0, 120.0),
                delta: (40.0, 0.0),
                input: Input::new("add", "out"),
            }),
            Rc::new(Wire {
                id: "add_to_up".to_string(),
                pos: (260.0, 60.0),
                delta: (0.0, 60.0),
                input: Input::new("add", "out"),
            }),
            Rc::new(Wire {
                id: "add_to_left".to_string(),
                pos: (60.0, 60.0),
                delta: (200.0, 0.0),
                input: Input::new("add", "out"),
            }),
            Rc::new(Wire {
                id: "add_to_down".to_string(),
                pos: (60.0, 60.0),
                delta: (0.0, 80.0),
                input: Input::new("add", "out"),
            }),
            Rc::new(Wire {
                id: "reg_in".to_string(),
                pos: (60.0, 140.0),
                delta: (30.0, 0.0),
                input: Input::new("add", "out"),
            }),
            Rc::new(Wire {
                id: "pc_to_down".to_string(),
                pos: (140.0, 140.0),
                delta: (0.0, 40.0),
                input: Input::new("reg", "out"),
            }),
            Rc::new(Wire {
                id: "pc_to_right".to_string(),
                pos: (140.0, 180.0),
                delta: (210.0, 0.0),
                input: Input::new("reg", "out"),
            }),
            Rc::new(InstrMem {
                id: "instr_mem".to_string(),
                pos: (400.0, 150.0),
                pc: Input::new("reg", "out"),
                // fake instructions just to show the relation between input address and instruction
                instr: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            }),
            Rc::new(Wire {
                id: "w8".to_string(),
                pos: (450.0, 120.0),
                delta: (70.0, 0.0),
                input: Input::new("instr_mem", "out"),
            }),
            Rc::new(Probe {
                id: "p1".to_string(),
                pos: (280.0, 160.0),
                input: Input::new("reg", "out"),
            }),
            Rc::new(Probe {
                id: "p2".to_string(),
                pos: (500.0, 100.0),
                input: Input::new("instr_mem", "out"),
            }),
        ],
    };

    let path = PathBuf::from("mips.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
