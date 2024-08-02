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
            JumpMerge::rc_new(
                "jump_merge",
                (160.0, 120.0),
                Input::new("c0", "out"),
                Input::new("c1", "out"),
            ),
            // Constant::rc_new("c0", (60.0, 100.0), 10),
            // Constant::rc_new("c1", (60.0, 140.0), 5),
            // Constant::rc_new("c2", (60.0, 180.0), 1),
            ProbeEdit::rc_new("c0", (60.0, 100.0)),
            ProbeEdit::rc_new("c1", (60.0, 140.0)),
            Probe::rc_new("p1", (270.0, 120.0), Input::new("jump_merge", MERGE_OUT_ID)),
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
