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
        (360.0, 170.0),
        Input::new("instruction_split", INSTRUCTION_SPLITTER_RS_ID),
        Input::new("instruction_split", INSTRUCTION_SPLITTER_RT_ID),
        Input::new("reg_addr_reg", REGISTER_OUT_ID), //write address
        Input::new("result_reg_EX", REGISTER_OUT_ID), //write data
        Input::new("control_unit_4", cntr_field::REG_WRITE_ENABLE_OUT),
    );

    // RegFile::rc_new(
    //     "reg_file",
    //     (3100.0, 2000.0),
    //     Input::new("instruction_split", INSTRUCTION_SPLITTER_RS_ID),
    //     Input::new("instruction_split", INSTRUCTION_SPLITTER_RT_ID),
    //     Input::new("reg_addr_reg", REGISTER_OUT_ID), //write address
    //     Input::new("result_reg_EX", REGISTER_OUT_ID), //write data
    //     Input::new("control_unit_4", cntr_field::REG_WRITE_ENABLE_OUT),
    // ),

    let cs = ComponentStore {
        store: vec![
            // register that holds instr addr
            Register::rc_new(
                "pc",
                (170.0, 410.0),
                Input::new("mux_jump_merge", MUX_OUT_ID),
            ),
            // step addr from reg by 4
            Constant::rc_new("+4", (170.0, 380.0), 4),
            Add::rc_new(
                "pc+4",
                (220.0, 380.0),
                Input::new("pc", REGISTER_OUT_ID),
                Input::new("+4", CONSTANT_OUT_ID),
            ),
            //
            //
            Rc::new(
                InstrMem::new(
                    "instr_mem".into(),
                    (280.0, 600.0),
                    Input::new("pc", REGISTER_OUT_ID),
                    Rc::clone(&mem),
                )
                .set_mem_view_reg(rc_reg_file.clone()),
            ),
            //
            //
            // MUX to choose what intruction addr to choose from, branch jump, reg, pc+4
            Mux::rc_new(
                "mux_jump_merge",
                (140.0, 390.0),
                Input::new("branch", BRANCH_OUT_ID),
                vec![
                    Input::new("pc_add_branch", ADD_OUT_ID), // describe origin
                    Input::new("reg_file", reg_file_fields::RS_VALUE_OUT_ID), // goes to addr, RD2
                    Input::new("merge_reg", REGISTER_OUT_ID), //
                    Input::new("pc+4", ADD_OUT_ID),          //clk
                ],
            ),
            //
            // merges to find out jump location
            JumpMerge::rc_new(
                "jump_merge",
                (125.0, 500.0),
                Input::new("pc", REGISTER_OUT_ID), //input from reg before pc+4
                Input::new("instr_mem", INSTR_MEM_INSTRUCTION_ID), //input from instruction mem
            ),
            //
            //
            Register::rc_new("pc+4_reg", (2300.0, 5000.0), Input::new("pc+4", ADD_OUT_ID)),
            //
            Register::rc_new(
                "InMem_reg",
                (2300.0, 5200.0),
                Input::new("instr_mem", INSTR_MEM_INSTRUCTION_ID),
            ),
            //
            Register::rc_new(
                "merge_reg",
                (2300.0, 5300.0),
                Input::new("jump_merge", MERGE_OUT_ID),
            ),
            //
            // splits intructions from ir to fields
            InstrSplit::rc_new(
                "instruction_split",
                (280.0, 140.0),
                Input::new("InMem_reg", REGISTER_OUT_ID),
            ),
            //
            // First CU, handles, selcet for signzero_extend and mux_write_addr
            ControlUnit::rc_new(
                "control_unit_1",
                (480.0, 100.0),
                Input::new("InMem_reg", REGISTER_OUT_ID),
            ),
            //
            // Second CU, handles, mux_source_a, mux_source_b and the alu
            ControlUnit::rc_new(
                "control_unit_2",
                (580.0, 100.0),
                Input::new("control_EX_reg", REGISTER_OUT_ID),
            ),
            //
            // Third CU, handles, write_back_mux, and DMs memread and memwrite
            ControlUnit::rc_new(
                "control_unit_3",
                (680.0, 100.0),
                Input::new("control_MEM_reg", REGISTER_OUT_ID),
            ),
            //
            // Fourth CU, handles, WE for reg_file in the WB stage
            ControlUnit::rc_new(
                "control_unit_4",
                (780.0, 100.0),
                Input::new("control_WB_reg", REGISTER_OUT_ID),
            ),
            //
            //
            // extends immediate field
            SignZeroExtend::rc_new(
                "signzero_extend",
                (310.0, 410.0),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_IMMEDIATE_ID),
                Input::new("control_unit_1", cntr_field::EXTEND_SELECT_OUT), // cu tells it to either sing- or zero- extend
            ),
            //
            //
            //
            //
            Equal::rc_new(
                "equals_operand_A",
                (3200.0, 1700.0),
                Input::new("reg_addr_MEM_reg", REGISTER_OUT_ID),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_RS_ID),
            ),
            //
            Mux::rc_new(
                "operand_A_mux_1",
                (3200.0, 1800.0),
                Input::new("equals_operand_A", EQUAL_OUT_ID),
                vec![
                    Input::new("reg_file", reg_file_fields::RS_VALUE_OUT_ID),
                    Input::new("write_back_mux", MUX_OUT_ID),
                ],
            ),
            //
            //
            Equal::rc_new(
                "equals_operand_B",
                (3200.0, 2300.0),
                Input::new("reg_addr_MEM_reg", REGISTER_OUT_ID),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_RT_ID),
            ),
            //
            Mux::rc_new(
                "operand_B_mux_1",
                (3200.0, 2200.0),
                Input::new("equals_operand_B", EQUAL_OUT_ID),
                vec![
                    Input::new("reg_file", reg_file_fields::RT_VALUE_OUT_ID),
                    Input::new("write_back_mux", MUX_OUT_ID),
                ],
            ),
            //
            //
            Equal::rc_new(
                "equals_operand_A_2",
                (3300.0, 1700.0),
                Input::new("reg_addr_EX_reg", REGISTER_OUT_ID),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_RS_ID),
            ),
            //
            Mux::rc_new(
                "operand_A_mux_2",
                (3300.0, 1800.0),
                Input::new("equals_operand_A_2", EQUAL_OUT_ID),
                vec![
                    Input::new("operand_A_mux_1", MUX_OUT_ID),
                    Input::new("alu", FULL_ADD_OUT_ID),
                ],
            ),
            //
            //
            Equal::rc_new(
                "equals_operand_B_2",
                (3300.0, 2300.0),
                Input::new("reg_addr_EX_reg", REGISTER_OUT_ID),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_RT_ID),
            ),
            //
            Mux::rc_new(
                "operand_B_mux_2",
                (3300.0, 2200.0),
                Input::new("equals_operand_B_2", EQUAL_OUT_ID),
                vec![
                    Input::new("operand_B_mux_1", MUX_OUT_ID),
                    Input::new("alu", FULL_ADD_OUT_ID),
                ],
            ),
            //
            //
            BranchLogic::rc_new(
                "branch",
                (525.0, 300.0),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_OP_ID),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_RT_ID),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_FUNCT_ID),
                Input::new("reg_file", reg_file_fields::RS_VALUE_OUT_ID),
                Input::new("reg_file", reg_file_fields::RT_VALUE_OUT_ID),
            ),
            //
            //
            Register::rc_new(
                //TODO: make 2 more controll units
                "control_EX_reg",
                (3450.0, 1400.0),
                Input::new("InMem_reg", REGISTER_OUT_ID),
            ),
            //
            Register::rc_new(
                "zero_extend_reg",
                (3450.0, 1600.0),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_SHAMT_ID),
            ),
            //
            Register::rc_new(
                //TODO: fix after adding 4 muxes
                "operand_a_reg",
                (3450.0, 1800.0),
                Input::new("operand_A_mux_2", MUX_OUT_ID),
            ),
            //
            Register::rc_new(
                //TODO: fix after muxes
                "operand_b_reg",
                (3450.0, 2200.0),
                Input::new("operand_B_mux_2", MUX_OUT_ID),
            ),
            //
            Register::rc_new(
                "mux_b2_reg",
                (3450.0, 5000.0),
                Input::new("pc+4_reg", REGISTER_OUT_ID),
            ),
            Register::rc_new(
                "mux_b3_reg",
                (3450.0, 5100.0),
                Input::new("signzero_extend", SIGNZEROEXTEND_OUT_ID),
            ),
            //
            Register::rc_new(
                "reg_addr_EX_reg",
                (3450.0, 5500.0),
                Input::new("mux_write_addr", MUX_OUT_ID),
            ),
            //
            //
            ZeroExtend::rc_new(
                "zero_extend_for_chamt",
                (550.0, 170.0),
                Input::new("zero_extend_reg", REGISTER_OUT_ID),
            ),
            //
            //
            Constant::rc_new("0_a_inp", (600.0, 230.0), 0),
            Mux::rc_new(
                "mux_source_a",
                (650.0, 220.0),
                Input::new("control_unit_2", cntr_field::ALU_SRC_A_OUT),
                vec![
                    Input::new("zero_extend_for_chamt", SIGNZEROEXTEND_OUT_ID),
                    Input::new("operand_a_reg", REGISTER_OUT_ID),
                    Input::new("0_a_inp", CONSTANT_OUT_ID),
                ],
            ),
            //
            //
            Mux::rc_new(
                "mux_source_b",
                (650.0, 300.0),
                Input::new("control_unit_2", cntr_field::ALU_SRC_B_OUT),
                vec![
                    Input::new("operand_b_reg", REGISTER_OUT_ID),
                    Input::new("mux_b2_reg", REGISTER_OUT_ID),
                    Input::new("mux_b3_reg", REGISTER_OUT_ID),
                ],
            ),
            //
            //
            //
            //
            FullAdd::rc_new(
                "alu",
                (720.0, 220.0),
                Input::new("mux_source_a", MUX_OUT_ID),
                Input::new("mux_source_b", MUX_OUT_ID),
                Input::new("control_unit_2", cntr_field::ALU_OP_OUT),
            ),
            //
            //
            Rc::new(
                DataMem::new(
                    "data_mem".into(),
                    (600.0, 580.0),
                    Input::new("alu_reg", REGISTER_OUT_ID), // calculated from rs and imm
                    Input::new("data_MEM_reg", REGISTER_OUT_ID),
                    Input::new("control_unit_3", cntr_field::MEM_MODE_OUT),
                    Input::new("control_unit_3", cntr_field::MEM_WRITE_ENABLE_OUT),
                    Rc::clone(&mem),
                )
                .set_mem_view_reg(rc_reg_file.clone()),
            ),
            //
            //
            Register::rc_new(
                "control_MEM_reg",
                (4200.0, 1400.0),
                Input::new("control_EX_reg", REGISTER_OUT_ID),
            ),
            //
            Register::rc_new(
                "alu_reg",
                (4200.0, 2000.0),
                Input::new("alu", FULL_ADD_OUT_ID),
            ),
            //
            Register::rc_new(
                "data_MEM_reg",
                (4200.0, 2500.0),
                Input::new("operand_b_reg", REGISTER_OUT_ID),
            ),
            //
            Register::rc_new(
                "reg_addr_MEM_reg",
                (4200.0, 5500.0),
                Input::new("reg_addr_EX_reg", REGISTER_OUT_ID),
            ),
            //
            //
            Mux::rc_new(
                "write_back_mux",
                (800.0, 270.0),
                Input::new("control_unit_3", cntr_field::REG_WRITE_SRC_OUT),
                vec![
                    Input::new("alu", FULL_ADD_OUT_ID),
                    Input::new("data_mem", DATA_MEM_READ_DATA_OUT_ID),
                ],
            ),
            //
            //
            Register::rc_new(
                "control_WB_reg",
                (4400.0, 1400.0),
                Input::new("control_MEM_reg", REGISTER_OUT_ID),
            ),
            //
            Register::rc_new(
                "result_reg_EX",
                (4400.0, 2200.0),
                Input::new("write_back_mux", MUX_OUT_ID),
            ),
            //
            Register::rc_new(
                "reg_addr_reg",
                (4400.0, 5500.0),
                Input::new("reg_addr_MEM_reg", REGISTER_OUT_ID),
            ),
            //
            //
            ShiftConst::rc_new(
                "branch_shift",
                (380.0, 460.0),
                Input::new("signzero_extend", SIGNZEROEXTEND_OUT_ID),
                2,
            ),
            //
            //
            Add::rc_new(
                "pc_add_branch",
                (420.0, 440.0),
                Input::new("pc+4", ADD_OUT_ID),
                Input::new("branch_shift", SHIFT_OUT_ID),
            ),
            //
            //
            Constant::rc_new("0x_1F", (500.0, 510.0), 0x_1F),
            Mux::rc_new(
                "mux_write_addr",
                (560.0, 500.0),
                Input::new("control_unit_1", cntr_field::REG_DEST_OUT),
                vec![
                    Input::new("instruction_split", INSTRUCTION_SPLITTER_RT_ID),
                    Input::new("instruction_split", INSTRUCTION_SPLITTER_RD_ID),
                    Input::new("0x_1F", CONSTANT_OUT_ID),
                ],
            ),
            //
            //

            //
            //
            rc_reg_file,
        ],
    };

    //let cs = autowire(cs);

    let path = PathBuf::from("add.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path, Library::default()).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}
