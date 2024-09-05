use mips_lib::components::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
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
            Rc::new(PhysicalMem::new("phys_mem", (0.0, 0.0))),
            ProbeEdit::rc_new("rs_addr", (60.0, 100.0)),
            ProbeEdit::rc_new("rt_addr", (60.0, 140.0)),
            ProbeEdit::rc_new("write_addr", (60.0, 180.0)),
            ProbeEdit::rc_new("write_data", (60.0, 220.0)),
            ProbeEdit::rc_new("write_enable", (60.0, 260.0)),
            Probe::rc_new(
                "rs",
                (300.0, 120.0),
                Input::new("reg_file", reg_file_fields::RS_VALUE_OUT_ID),
            ),
            Probe::rc_new(
                "rt",
                (60.0, 400.0),
                Input::new("reg_file", reg_file_fields::RT_VALUE_OUT_ID),
            ),
            ProbeEdit::rc_new("pc", (60.0, 500.0)),
            Rc::new(InstrMem::new(
                "instr_mem".into(),
                (200.0, 500.0),
                Input::new("pc", "out"),
                "phys_mem".into(),
                "reg_file".into(),
            )),
            ProbeEdit::rc_new("data_adrs", (60.0, 660.0)),
            ProbeEdit::rc_new("data_write_enable", (60.0, 700.0)),
            ProbeEdit::rc_new("data_mem_op", (60.0, 740.0)),
            ProbeEdit::rc_new("data_write_data", (60.0, 780.0)),
            Rc::new(DataMem::new(
                "data_mem".into(),
                (200.0, 700.0),
                Input::new("data_adrs", "out"),
                Input::new("data_write_data", "out"),
                Input::new("data_mem_op", "out"),
                Input::new("data_write_enable", "out"),
                "phys_mem".into(),
                "reg_file".into(),
            )),
            RegFile::rc_new(
                "reg_file",
                (200.0, 200.0),
                Input::new("rs_addr", "out"),
                Input::new("rt_addr", "out"),
                Input::new("write_addr", "out"),
                Input::new("write_data", "out"),
                Input::new("write_enable", "out"),
            ),
        ],
    };

    // let cs = autowire(cs);

    let path = PathBuf::from("add.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path, Library::default()).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
