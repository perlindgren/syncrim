use mips_lib::components::*;
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
            Rc::new(ProbeEdit::new("instr", (100.0, 10.0))),
            ControlUnit::rc_new("cntr", (200.0, 10.0), Input::new("instr", "out")),
            Probe::rc_new(
                "reg_dest",
                (300.0, 10.0),
                Input::new("cntr", cntr_field::REG_DEST_OUT),
            ),
            Probe::rc_new(
                "reg_write",
                (300.0, 30.0),
                Input::new("cntr", cntr_field::REG_WRITE_ENABLE_OUT),
            ),
            Probe::rc_new(
                "reg_write_src",
                (300.0, 50.0),
                Input::new("cntr", cntr_field::REG_WRITE_SRC_OUT),
            ),
            Probe::rc_new(
                "alu_op",
                (300.0, 70.0),
                Input::new("cntr", cntr_field::ALU_OP_OUT),
            ),
            Probe::rc_new(
                "alu_src_a",
                (300.0, 90.0),
                Input::new("cntr", cntr_field::ALU_SRC_A_OUT),
            ),
            Probe::rc_new(
                "alu_src_b",
                (300.0, 110.0),
                Input::new("cntr", cntr_field::ALU_SRC_B_OUT),
            ),
            Probe::rc_new(
                "extend_select",
                (300.0, 130.0),
                Input::new("cntr", cntr_field::EXTEND_SELECT_OUT),
            ),
            Probe::rc_new(
                "mem_write",
                (300.0, 150.0),
                Input::new("cntr", cntr_field::MEM_WRITE_ENABLE_OUT),
            ),
            Probe::rc_new(
                "branch_interrupt",
                (300.0, 170.0),
                Input::new("cntr", cntr_field::BRANCH_INTERRUPT_OUT),
            ),
        ],
    };
    let path = PathBuf::from("cntr_unit.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    {
        use syncrim::autowire::autowire;
        let cs = autowire(cs);
        cs.save_file(&path);
        syncrim::gui_egui::gui(cs, &path, Library::default()).ok();
    }

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
