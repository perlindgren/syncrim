use mips::components::*;
use std::cell::Cell;
use std::{path::PathBuf, rc::Rc};
use syncrim::{
    common::{ComponentStore, Input, Signal},
    components::*,
    fern::fern_setup,
};

// TODO: fix wires and layout
fn main() {
    fern_setup();
    let cs = ComponentStore {
        store: vec![
            Rc::new(Constant {
                id: "c_read_reg_1".to_string(),
                pos: (100.0, 100.0),
                value: 3,
            }),
            Rc::new(Constant {
                id: "c_read_reg_2".to_string(),
                pos: (100.0, 200.0),
                value: 4,
            }),
            Rc::new(Constant {
                id: "c_write_data".to_string(),
                pos: (100.0, 140.0),
                value: 42,
            }),
            Rc::new(Constant {
                id: "c_write_addr".to_string(),
                pos: (100.0, 160.0),
                value: 4,
            }),
            Rc::new(Constant {
                id: "c_write_enable".to_string(),
                pos: (100.0, 180.0),
                value: true as Signal,
            }),
            // regfile
            Rc::new(RegFile {
                id: "reg_file".to_string(),
                pos: (300.0, 200.0),
                width: 200.0,
                height: 300.0,

                // ports
                read_addr1: Input::new("c_read_reg_1", "out"),
                read_addr2: Input::new("c_read_reg_2", "out"),
                write_data: Input::new("c_write_data", "out"),
                write_addr: Input::new("c_write_addr", "out"),
                write_enable: Input::new("c_write_enable", "out"),

                // data
                registers: Rc::new(vec![Cell::new(0); 32]),
            }),
        ],
    };

    let path = PathBuf::from("reg_file.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
