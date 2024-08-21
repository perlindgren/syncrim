// use crate::src::components::cntr_unit_signals;
use std::rc::Rc;
use std::{cell::RefCell, path::PathBuf};
use syncrim::common::EguiComponent;
#[cfg(feature = "gui-egui")]
use syncrim::gui_egui::editor::Library;
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
    fern::fern_setup,
    mips_helper_functions::autowire,
};

fn main() {
    fern_setup();

    let mem = Rc::new(RefCell::new(MipsMem::default()));
    let rc_reg_file = RegFile::rc_new(
        "reg_file",
        (3100.0, 2000.0),
        Input::new("instruction_split", INSTRUCTION_SPLITTER_RS_ID),
        Input::new("instruction_split", INSTRUCTION_SPLITTER_RT_ID),
        Input::new("reg_write_addr", REGISTER_OUT_ID), //write address
        Input::new("result_reg", REGISTER_OUT_ID),     //write data
        Input::new("reg_we", REGISTER_OUT_ID),
    );

    let cs = ComponentStore {
        store: vec![
            // register that holds instr addr
            Register::rc_new("pc", (2000.0, 5000.0), Input::new("mux_jump_merge", "out")),
            // step addr from reg by 4
            Constant::rc_new("+4", (2000.0, 5100.0), 4),
            Add::rc_new(
                "pc+4",
                (2200.0, 5000.0),
                Input::new("pc", "out"),
                Input::new("+4", "out"),
            ),
            //
            //
            Rc::new(
                InstrMem::new(
                    "instr_mem".into(),
                    (200.0, 500.0),
                    Input::new("pc", "out"),
                    Rc::clone(&mem),
                )
                .set_mem_view_reg(rc_reg_file.clone()),
            ),
            //
            //
            // MUX to choose what intruction addr to choose from, branch jump, reg, pc+4
            Mux::rc_new(
                "mux_jump_merge",
                (1800.0, 5000.0),
                Input::new("branch", BRANCH_OUT_ID),
                vec![
                    Input::new("pc_add_branch", ADD_OUT_ID), //TODO: describe origin
                    Input::new("reg_file", reg_file_fields::RT_VALUE_OUT_ID), // goes to addr, RD2
                    Input::new("jump_merge", MERGE_OUT_ID),  //
                    Input::new("pc+4", CLK_OUT_ID),
                ],
            ),
            //
            // merges to find out jump location
            JumpMerge::rc_new(
                "jump_merge",
                (1700.0, 5300.0),
                Input::new("pc", REGISTER_OUT_ID), //input from reg before pc+4
                Input::new("instr_mem", INSTR_MEM_INSTRUCTION_ID), //input from instruction mem
            ),
            //
            // splits intructions from ir to fields
            InstrSplit::rc_new(
                "instruction_split",
                (2400.0, 4000.0),
                Input::new("instr_mem", INSTR_MEM_INSTRUCTION_ID),
            ), //TODO: take im input
            //
            //
            ControlUnit::rc_new(
                "control_unit",
                (5000.0, 500.0),
                Input::new("instr_mem", INSTR_MEM_INSTRUCTION_ID),
            ), //TODO: take im input
            //
            //
            Register::rc_new(
                "reg_we",
                (4400.0, 500.0),
                Input::new("control_unit", cntr_field::REG_WRITE_ENABLE_OUT),
            ),
            //
            // extends immediate field
            SignZeroExtend::rc_new(
                "signzero_extend",
                (2600.0, 5000.0),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_IMMEDIATE_ID),
                Input::new("control_unit", cntr_field::EXTEND_SELECT_OUT), // cu tells it to either sing- or zero- extend
            ),
            //
            //
            BranchLogic::rc_new(
                "branch",
                (3300.0, 2000.0),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_OP_ID),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_RT_ID),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_FUNCT_ID),
                Input::new("reg_file", reg_file_fields::RS_VALUE_OUT_ID),
                Input::new("reg_file", reg_file_fields::RT_VALUE_OUT_ID),
            ),
            //
            //
            ZeroExtend::rc_new(
                "zero_extend_for_chamt",
                (3700.0, 1600.0),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_SHAMT_ID),
            ),
            //
            //
            Constant::rc_new("0_a_inp", (3800.0, 1700.0), 0),
            Mux::rc_new(
                "mux_source_a",
                (3800.0, 1800.0),
                Input::new("control_unit", cntr_field::ALU_SRC_A_OUT),
                vec![
                    Input::new("zero_extend_for_chamt", SIGNZEROEXTEND_OUT_ID),
                    Input::new("reg_file", reg_file_fields::RS_VALUE_OUT_ID), //FIXME should be rs? changed from rt
                    Input::new("0_a_inp", "out"),
                ],
            ),
            //
            //
            Mux::rc_new(
                "mux_source_b",
                (3800.0, 2200.0),
                Input::new("control_unit", cntr_field::ALU_SRC_B_OUT),
                vec![
                    Input::new("reg_file", reg_file_fields::RT_VALUE_OUT_ID), //FIXME should be rt? changed from rs
                    Input::new("pc+4", ADD_OUT_ID),
                    Input::new("signzero_extend", SIGNZEROEXTEND_OUT_ID),
                ],
            ),
            //
            //
            FullAdd::rc_new(
                "alu",
                (4100.0, 2000.0),
                Input::new("mux_source_a", MUX_OUT_ID),
                Input::new("mux_source_b", MUX_OUT_ID),
                Input::new("control_unit", cntr_field::ALU_OP_OUT),
            ),
            //
            //
            Rc::new(
                DataMem::new(
                    "data_mem".into(),
                    (4100.0, 2200.0),
                    Input::new("alu", FULL_ADD_OUT_ID), // calculated from rs and imm
                    Input::new("reg_file", reg_file_fields::RT_VALUE_OUT_ID),
                    Input::new("control_unit", cntr_field::MEM_MODE_OUT),
                    Input::new("control_unit", cntr_field::MEM_WRITE_ENABLE_OUT),
                    Rc::clone(&mem),
                )
                .set_mem_view_reg(rc_reg_file.clone()),
            ),
            //
            //
            Mux::rc_new(
                "mux_write_back",
                (4300.0, 2200.0),
                Input::new("control_unit", cntr_field::REG_WRITE_SRC_OUT),
                vec![
                    Input::new("alu", FULL_ADD_OUT_ID),
                    Input::new("data_mem", DATA_MEM_READ_DATA_OUT_ID), //TODO: data meme output
                ],
            ),
            //
            //
            Register::rc_new(
                "result_reg",
                (4400.0, 2200.0),
                Input::new("mux_write_back", MUX_OUT_ID),
            ),
            //
            //
            ShiftConst::rc_new(
                "branch_shift",
                (2900.0, 5250.0),
                Input::new("signzero_extend", SIGNZEROEXTEND_OUT_ID),
                2,
            ),
            //
            //
            Add::rc_new(
                "pc_add_branch",
                (3000.0, 5200.0),
                Input::new("pc+4", ADD_OUT_ID),
                Input::new("branch_shift", SHIFT_OUT_ID),
            ),
            //
            //
            Constant::rc_new("0x_1F", (3750.0, 5500.0), 0),
            Mux::rc_new(
                "mux_write_addr",
                (3800.0, 5500.0),
                Input::new("control_unit", cntr_field::REG_DEST_OUT),
                vec![
                    Input::new("instruction_split", INSTRUCTION_SPLITTER_RT_ID),
                    Input::new("instruction_split", INSTRUCTION_SPLITTER_RD_ID),
                    Input::new("0x_1F", "out"),
                ],
            ),
            //
            //
            Register::rc_new(
                "reg_write_addr",
                (4400.0, 5500.0),
                Input::new("mux_write_addr", MUX_OUT_ID),
            ),
            //
            //
            Constant::rc_new("dummy", (6000.0, 3000.0), 0),
            rc_reg_file,
        ],
    };

    let cs = autowire(cs);

    let path = PathBuf::from("add.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path, Library::default()).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
