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
                pos: (300.0, 220.0),
                a_in: Input::new("c1", 0),
                b_in: Input::new("pc_reg", 0),
            }),
            Rc::new(Constant {
                id: "c1".to_string(),
                pos: (200.0, 200.0),
                value: 4,
            }),
            Rc::new(Register {
                id: "pc_reg".to_string(),
                pos: (200.0, 240.0),
                r_in: Input::new("add", 0),
            }),
            Rc::new(Wire {
                id: "c1_to_add_a".to_string(),
                pos: (210.0, 200.0),
                delta: (70.0, 0.0),
                input: Input::new("c1", 0),
            }),
            Rc::new(Wire {
                id: "pc_reg_to_add_b".to_string(),
                pos: (210.0, 240.0),
                delta: (70.0, 0.0),
                input: Input::new("pc_reg", 0),
            }),
            Rc::new(Wire {
                id: "add_to_right".to_string(),
                pos: (320.0, 220.0),
                delta: (40.0, 0.0),
                input: Input::new("add", 0),
            }),
            Rc::new(Wire {
                id: "add_to_up".to_string(),
                pos: (360.0, 160.0),
                delta: (0.0, 60.0),
                input: Input::new("add", 0),
            }),
            Rc::new(Wire {
                id: "add_to_left".to_string(),
                pos: (160.0, 160.0),
                delta: (200.0, 0.0),
                input: Input::new("add", 0),
            }),
            Rc::new(Wire {
                id: "add_to_down".to_string(),
                pos: (160.0, 160.0),
                delta: (0.0, 80.0),
                input: Input::new("add", 0),
            }),
            Rc::new(Wire {
                id: "pc_reg_in".to_string(),
                pos: (160.0, 240.0),
                delta: (30.0, 0.0),
                input: Input::new("add", 0),
            }),
            Rc::new(Wire {
                id: "pc_to_down".to_string(),
                pos: (240.0, 240.0),
                delta: (0.0, 340.0),
                input: Input::new("pc_reg", 0),
            }),
            Rc::new(InstrMem {
                id: "instr_mem".to_string(),
                pos: (300.0, 600.0),
                width: 200.0,
                height: 50.0,
                pc: Input::new("pc_reg", 0),
                // fake instructions just to show the relation between input address and instruction
                instr: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            }),
            Rc::new(Wire {
                id: "w8".to_string(),
                pos: (550.0, 220.0),
                delta: (70.0, 0.0),
                input: Input::new("instr_mem", 0),
            }),
            Rc::new(Probe {
                id: "p1".to_string(),
                pos: (380.0, 260.0),
                input: Input::new("pc_reg", 0),
            }),
            Rc::new(Probe {
                id: "p2".to_string(),
                pos: (500.0, 200.0),
                input: Input::new("instr_mem", 0),
            }),
            // instr_split
            Rc::new(InstrSplit {
                id: "instr_split".to_string(),
                pos: (400.0, 300.0),
                width: 10.0,
                height: 300.0,

                // ports
                instr: Input::new("instr_mem", 0),
            }),
            Rc::new(Wire {
                id: "instr_to_split".to_string(),
                pos: (350.0, 220.0),
                delta: (0.0, 300.0),
                input: Input::new("instr_mem", 0),
            }),
            Rc::new(Wire {
                id: "instr_to_split".to_string(),
                pos: (200.0, 220.0),
                delta: (40.0, 300.0),
                input: Input::new("instr_mem", 0),
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
