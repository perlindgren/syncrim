use std::cell::RefCell;
use std::rc::Rc;
use std::{path::PathBuf, rc};
use syncrim::common::EguiComponent;
#[cfg(feature = "gui-egui")]
use syncrim::gui_egui::editor::Library;
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
    fern::fern_setup,
};

fn main() {
    fern_setup();

    // create an empty memory that both IM and DM can refrence
    let mem = Rc::new(RefCell::new(MipsMem::default()));
    let reg_file = RegFile::rc_new(
        "reg_file",
        (200.0, 200.0),
        Input::new("rs_addr", "out"),
        Input::new("rt_addr", "out"),
        Input::new("write_addr", "out"),
        Input::new("write_data", "out"),
        Input::new("write_enable", "out"),
    );

    let cs = ComponentStore {
        store: vec![
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
            Rc::new(
                InstrMem::new(
                    "instr_mem".into(),
                    (200.0, 500.0),
                    Input::new("pc", "out"),
                    Rc::clone(&mem),
                )
                .set_mem_view_reg(Rc::clone(&reg_file)),
            ), // InstrMem::rc_new(
            //     "instr_mem".into(),
            //     (200.0, 500.0),
            //     Input::new("pc", "out"),
            //     Rc::clone(&mem),
            // ).set_mem_view_reg(reg_rc),
            reg_file,
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
