use mips::components::*;
use std::{path::PathBuf, rc::Rc};
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
    fern::fern_setup,
};

// TODO: fix wires and layout
fn main() {
    fern_setup();
    let cs = ComponentStore {
        store: vec![
            Constant::rc_new("c_read_reg_1", (100.0, 100.0), 3),
            Constant::rc_new("c_read_reg_2", (100.0, 200.0), 4),
            Constant::rc_new("c_write_data", (100.0, 140.0), 42),
            Constant::rc_new("c_write_addr", (100.0, 160.0), 4),
            Constant::rc_new("c_write_enable", (100.0, 180.0), true),
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
                registers: RegStore::new(),
                history: RegHistory::new(),
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
