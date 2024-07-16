use std::{
    any::{Any, TypeId},
    path::PathBuf,
};
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
            FullAdd::rc_new(
                "full_adder",
                (200.0, 120.0),
                Input::new("c1", "out"),
                Input::new("c2", "out"),
                Input::new("c3", "out"),
            ),
            // Constant::rc_new("c1", (60.0, 100.0), 10),
            // Constant::rc_new("c2", (60.0, 140.0), 5),
            // Constant::rc_new("c3", (60.0, 180.0), 1),

            // ProbeEdit::rc_new("c1", (60.0, 100.0)),
            // ProbeEdit::rc_new("c2", (60.0, 140.0)),
            // ProbeEdit::rc_new("c3", (60.0, 180.0)),

            // Wire::rc_new(
            //     "w1",
            //     vec![(110.0, 100.0), (180.0, 100.0)],
            //     Input::new("c1", "out"),
            // ),
            // Wire::rc_new(
            //     "w2",
            //     vec![(110.0, 140.0), (180.0, 140.0)],
            //     Input::new("c2", "out"),
            // ),
            // Wire::rc_new(
            //     "w3",
            //     vec![(110.0, 180.0), (180.0, 180.0)],
            //     Input::new("c3", "out"),
            // ),
            // Wire::rc_new(
            //     "w4",
            //     vec![(220.0, 120.0), (260.0, 120.0)],
            //     Input::new("full_adder", FULL_ADD_OUT_ID),
            // ),
            // Probe::rc_new(
            //     "p1",
            //     (270.0, 120.0),
            //     Input::new("full_adder", FULL_ADD_OUT_ID),
            // ),
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

fn autowire(cs: ComponentStore) -> ComponentStore {
    let mut x = 1;
    // for component in cs {
    //     println!("{}", x);
    //     x = x + 1;
    // }
    // while x < 10 {

    // }

    //let cs_copy = cs.clone();

    for c in &cs.store {
        let (id, ports) = c.get_id_ports();
        println!("{}", id);

        //println!("{:?}", c.get_id_ports().1.inputs);

        let number_of_inputs = ports.inputs.len();

        for n in 0..number_of_inputs {
            println!("{:?}", ports.inputs[n]);

            let id_tmp = format!("w{n}");
            //let pos_temp = vec![];
            let input = ports.inputs[n].input.clone();
            //println!("{}", id_tmp);
            //let w = Wire::rc_new("w{}", pos, input)
        }

        //cs_copy.store.push("wow");

        // Wire::rc_new(
        //     "w4",
        //     vec![(220.0, 120.0), (260.0, 120.0)],
        //     Input::new("full_adder", FULL_ADD_OUT_ID),
        // ),

        // Ports {
        //     inputs: [
        //         InputPort {
        //             port_id: "full_add_a_in",
        //             input: Input {
        //                 id: "c1",
        //                 field: "out",
        //             },
        //         },
        //         InputPort {
        //             port_id: "full_add_b_in",
        //             input: Input {
        //                 id: "c2",
        //                 field: "out",
        //             },
        //         },
        //         InputPort {
        //             port_id: "full_add_op_in",
        //             input: Input {
        //                 id: "c3",
        //                 field: "out",
        //             },
        //         },
        //     ],

        //     out_type: Combinatorial,
        //     outputs: ["out"],
        // }
    }

    return cs;
}
