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
            Rc::new(PhysicalMem::new("phys_mem", (0.0, 0.0))),
            // register that holds instr addr
            Register::rc_new(
                "pc",
                (150.0, 390.0),
                Input::new("mux_jump_merge", MUX_OUT_ID),
            ),
            // step addr from reg by 4
            Constant::rc_new("+4", (150.0, 440.0), 4),
            Add::rc_new_with_scale(
                "pc+4",
                (200.0, 400.0),
                Input::new("+4", CONSTANT_OUT_ID),
                Input::new("pc", REGISTER_OUT_ID),
                0.6,
            ),
            //
            //
            Rc::new(InstrMem::new(
                "instr_mem".into(),
                (250.0, 575.0),
                Input::new("pc", REGISTER_OUT_ID),
                "phys_mem".into(),
                "reg_file".into(),
            )),
            //
            //
            // MUX to choose what intruction addr to choose from, branch jump, reg, pc+4
            Mux::rc_new_with_scale(
                "mux_jump_merge",
                (100.0, 390.0),
                Input::new("branch", BRANCH_OUT_ID),
                vec![
                    Input::new("pc_add_branch", ADD_OUT_ID), //TODO: describe origin
                    Input::new("reg_file", reg_file_fields::RS_VALUE_OUT_ID), // goes to addr, RD2
                    Input::new("jump_merge", MERGE_OUT_ID),  //
                    Input::new("pc+4", ADD_OUT_ID),
                ],
                0.6,
            ),
            //
            // merges to find out jump location
            JumpMerge::rc_new(
                "jump_merge",
                (125.0, 475.0),
                Input::new("pc", REGISTER_OUT_ID), //input from reg before pc+4
                Input::new("instr_mem", INSTR_MEM_INSTRUCTION_ID), //input from instruction mem
            ),
            //
            // splits intructions from ir to fields
            InstrSplit::rc_new(
                "instruction_split",
                (200.0, 225.0),
                Input::new("instr_mem", INSTR_MEM_INSTRUCTION_ID),
            ),
            //
            //
            ControlUnit::rc_new(
                "control_unit",
                (500.0, 50.0),
                Input::new("instr_mem", INSTR_MEM_INSTRUCTION_ID),
            ),
            //
            //
            Register::rc_new(
                "reg_we",
                (850.0, 50.0),
                Input::new("control_unit", cntr_field::REG_WRITE_ENABLE_OUT),
            ),
            //
            // extends immediate field
            SignZeroExtend::rc_new(
                "signzero_extend",
                (325.0, 425.0),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_IMMEDIATE_ID),
                Input::new("control_unit", cntr_field::EXTEND_SELECT_OUT), // cu tells it to either sing- or zero- extend
            ),
            //
            //
            BranchLogic::rc_new(
                "branch",
                (475.0, 225.0),
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
                (600.0, 150.0),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_SHAMT_ID),
            ),
            //
            //
            Constant::rc_new("0_a_inp", (610.0, 220.0), 0),
            Mux::rc_new_with_scale(
                "mux_source_a",
                (650.0, 195.0),
                Input::new("control_unit", cntr_field::ALU_SRC_A_OUT),
                vec![
                    Input::new("zero_extend_for_chamt", ZEROEXTEND_OUT_ID),
                    Input::new("reg_file", reg_file_fields::RS_VALUE_OUT_ID),
                    Input::new("0_a_inp", CONSTANT_OUT_ID),
                ],
                0.6,
            ),
            //
            //
            Mux::rc_new_with_scale(
                "mux_source_b",
                (650.0, 255.0),
                Input::new("control_unit", cntr_field::ALU_SRC_B_OUT),
                vec![
                    Input::new("reg_file", reg_file_fields::RT_VALUE_OUT_ID),
                    Input::new("pc+4", ADD_OUT_ID),
                    Input::new("signzero_extend", SIGNZEROEXTEND_OUT_ID),
                ],
                0.6,
            ),
            //
            //
            ALU::rc_new(
                "alu",
                (720.0, 225.0),
                Input::new("mux_source_a", MUX_OUT_ID),
                Input::new("mux_source_b", MUX_OUT_ID),
                Input::new("control_unit", cntr_field::ALU_OP_OUT),
            ),
            //
            //
            Rc::new(DataMem::new(
                "data_mem".into(),
                (600.0, 575.0),
                Input::new("alu", FULL_ADD_OUT_ID), // calculated from rs and imm
                Input::new("reg_file", reg_file_fields::RT_VALUE_OUT_ID),
                Input::new("control_unit", cntr_field::MEM_MODE_OUT),
                Input::new("control_unit", cntr_field::MEM_WRITE_ENABLE_OUT),
                "phys_mem".into(),
                "reg_file".into(),
            )),
            //
            //
            Mux::rc_new_with_scale(
                "mux_write_back",
                (800.0, 225.0),
                Input::new("control_unit", cntr_field::REG_WRITE_SRC_OUT),
                vec![
                    Input::new("alu", FULL_ADD_OUT_ID),
                    Input::new("data_mem", DATA_MEM_READ_DATA_OUT_ID), //TODO: data meme output
                ],
                0.6,
            ),
            //
            //
            Register::rc_new(
                "result_reg",
                (850.0, 225.0),
                Input::new("mux_write_back", MUX_OUT_ID),
            ),
            //
            //
            ShiftConst::rc_new(
                "branch_shift",
                (325.0, 480.0),
                Input::new("signzero_extend", SIGNZEROEXTEND_OUT_ID),
                2,
            ),
            //
            //
            Add::rc_new_with_scale(
                "pc_add_branch",
                (400.0, 475.0),
                Input::new("pc+4", ADD_OUT_ID),
                Input::new("branch_shift", SHIFT_OUT_ID),
                0.6,
            ),
            //
            //
            Constant::rc_new("0x_1F", (350.0, 550.0), 0x_1F),
            Mux::rc_new_with_scale(
                "mux_write_addr",
                (400.0, 525.0),
                Input::new("control_unit", cntr_field::REG_DEST_OUT),
                vec![
                    Input::new("instruction_split", INSTRUCTION_SPLITTER_RT_ID),
                    Input::new("instruction_split", INSTRUCTION_SPLITTER_RD_ID),
                    Input::new("0x_1F", CONSTANT_OUT_ID),
                ],
                0.6,
            ),
            //
            //
            Register::rc_new(
                "reg_write_addr",
                (850.0, 525.0),
                Input::new("mux_write_addr", MUX_OUT_ID),
            ),
            //
            //
            RegFile::rc_new(
                "reg_file",
                (350.0, 225.0),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_RS_ID),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_RT_ID),
                Input::new("reg_write_addr", REGISTER_OUT_ID), //write address
                Input::new("result_reg", REGISTER_OUT_ID),     //write data
                Input::new("reg_we", REGISTER_OUT_ID),
            ),
        ],
    };

    let path = PathBuf::from("mips_single_ex.json");
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
