use std::rc::Rc;
use std::{
    any::{Any, TypeId},
    path::PathBuf,
};
use syncrim::common::EguiComponent;
#[cfg(feature = "gui-egui")]
use syncrim::gui_egui::editor::Library;
use syncrim::{
    common::{ComponentStore, Input},
    component_store,
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

    let cs = autowire(cs);

    let path = PathBuf::from("add.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(cs, &path, Library::default()).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(cs, &path);
}

fn autowire(mut cs: ComponentStore) -> ComponentStore {
    let mut tmp_vec: Vec<Rc<dyn EguiComponent>> = vec![];

    // look through the list of components
    for c in &cs.store {
        let (id, ports) = c.get_id_ports();
        println!("{}", id);

        let number_of_inputs = ports.inputs.len();

        for n in 0..number_of_inputs {
            // println!("{:?}", ports.inputs[n].input.id);
            // println!("{:?}", ports.inputs[n]);

            let id_tmp = format!("{id}_w{n}");
            let input = ports.inputs[n].input.clone();
            let starting_pos = c.get_pos();

            // creates temporary vales to use when constatnt inputs are there for testing
            let mut destination_pos = (starting_pos.0 - 50.0, starting_pos.1);
            let default_input = Input::new(&ports.inputs[n].input.id, "out");

            // look through the list again and act when you find a matching id
            for d in &cs.store {
                let (id2, ports2) = d.get_id_ports();
                // look if the id matches the one you are looking for
                if id2 == ports.inputs[n].input.id {
                    // collect the components destination and use it to make a complete wire
                    destination_pos = d.get_pos();
                    let w =
                        Wire::rc_new(&id_tmp, vec![starting_pos, destination_pos], input.clone());
                    tmp_vec.push(w);
                }
            }
        }
    }
    cs.store.append(&mut tmp_vec);

    return cs;
}
