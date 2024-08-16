// use crate::src::components::cntr_unit_signals;
use std::path::PathBuf;
use std::rc::Rc;
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
            //
            // MUX to choose what intruction addr to choose from, branch jump, reg, pc+4
            Mux::rc_new(
                "mux_jump_merge",
                (1800.0, 5000.0),
                Input::new("branch", BRANCH_OUT_ID),
                vec![
                    Input::new("pc_add_branch", FULL_ADD_OUT_ID), //TODO: describe origin
                    Input::new("reg_file", reg_file_fields::RT_VALUE_OUT_ID), // goes to addr, RD2
                    Input::new("merge_reg", REGISTER_OUT_ID),     //
                    Input::new("pc+4", CLK_OUT_ID),
                ],
            ),
            //
            // merges to find out jump location
            JumpMerge::rc_new(
                "jump_merge",
                (1700.0, 5300.0),
                Input::new("pc", REGISTER_OUT_ID), //input from reg before pc+4
                Input::new("dummy", "out"),        //input from instruction mem
            ),
            //
            //
            Register::rc_new(
                //TODO: continue the chain of regs
                "pc+4_reg",
                (2300.0, 5000.0),
                Input::new("pc+4", ADD_OUT_ID),
            ),
            //
            Register::rc_new(
                "InMem_reg",
                (2300.0, 5200.0),
                Input::new("dummy", "out"), //TODO: im
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
                (2400.0, 4000.0),
                Input::new("InMem_reg", REGISTER_OUT_ID),
            ), //TODO: take im input
            //
            //TODO: add 3 more contrill units and rewire
            // First CU, handles, selcet for signzero_extend and mux_write_addr
            ControlUnit::rc_new(
                "control_unit_1",
                (3250.0, 1500.0),
                Input::new("InMem_reg", REGISTER_OUT_ID),
            ),
            //
            // Second CU, handles, mux_source_a, mux_source_b and the alu
            ControlUnit::rc_new(
                "control_unit_2",
                (3550.0, 1500.0),
                Input::new("control_EX_reg", REGISTER_OUT_ID),
            ),
            //
            // Third CU, handles, write_back_mux, and DMs memread and memwrite
            ControlUnit::rc_new(
                "control_unit_3",
                (4300.0, 1500.0),
                Input::new("control_MEM_reg", REGISTER_OUT_ID),
            ),
            //
            // Fourth CU, handles, WE for reg_file in the WB stage
            ControlUnit::rc_new(
                "control_unit_4",
                (4500.0, 1500.0),
                Input::new("control_WB_reg", REGISTER_OUT_ID),
            ),
            //
            //
            // extends immediate field
            SignZeroExtend::rc_new(
                "signzero_extend",
                (2600.0, 5100.0),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_IMMEDIATE_ID),
                Input::new("control_unit_1", cntr_field::EXTEND_SELECT_OUT), // cu tells it to either sing- or zero- extend
            ),
            //
            //
            RegFile::rc_new(
                "reg_file",
                (3100.0, 2000.0),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_RS_ID),
                Input::new("instruction_split", INSTRUCTION_SPLITTER_RT_ID),
                Input::new("reg_addr_reg", REGISTER_OUT_ID), //write address
                Input::new("result_reg_EX", REGISTER_OUT_ID), //write data
                Input::new("control_unit_4", cntr_field::REG_WRITE_ENABLE_OUT),
            ),
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
                (3400.0, 2000.0),
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
                Input::new("equals_operand_A_2", MUX_OUT_ID),
            ),
            //
            Register::rc_new(
                //TODO: fix after muxes
                "operand_b_reg",
                (3450.0, 2200.0),
                Input::new("equals_operand_B_2", MERGE_OUT_ID),
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
                (3700.0, 1600.0),
                Input::new("zero_extend_reg", REGISTER_OUT_ID),
            ),
            //
            //
            Constant::rc_new("0_a_inp", (3800.0, 1700.0), 0),
            Mux::rc_new(
                "mux_source_a",
                (3800.0, 1800.0),
                Input::new("control_unit_2", cntr_field::ALU_SRC_A_OUT),
                vec![
                    Input::new("zero_extend_for_chamt", SIGNZEROEXTEND_OUT_ID),
                    Input::new("0_a_inp", "out"),
                    Input::new("operand_a_reg", REGISTER_OUT_ID),
                ],
            ),
            //
            //
            Mux::rc_new(
                "mux_source_b",
                (3800.0, 2200.0),
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
                (4100.0, 2000.0),
                Input::new("mux_source_a", MUX_OUT_ID),
                Input::new("mux_source_b", MUX_OUT_ID),
                Input::new("control_unit_2", cntr_field::ALU_OP_OUT),
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
                (4300.0, 2200.0),
                Input::new("control_unit_3", cntr_field::REG_WRITE_SRC_OUT),
                vec![
                    Input::new("alu", FULL_ADD_OUT_ID),
                    Input::new("dummy", "out"), //TODO: data meme output
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
                (3200.0, 5500.0),
                Input::new("control_unit_1", cntr_field::REG_DEST_OUT),
                vec![
                    Input::new("instruction_split", INSTRUCTION_SPLITTER_RT_ID),
                    Input::new("0x_1F", "out"),
                    Input::new("instruction_split", INSTRUCTION_SPLITTER_RD_ID),
                ],
            ),
            //
            //

            //
            //
            Constant::rc_new("dummy", (6000.0, 3000.0), 0),
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
