use std::path::PathBuf;
use std::rc::Rc;
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
    let cs = ComponentStore {
        store: vec![
            ProbeEdit::rc_new("rs_addr", (60.0, 100.0)),
            ProbeEdit::rc_new("rt_addr", (60.0, 140.0)),
            ProbeEdit::rc_new("write_addr", (60.0, 180.0)),
            ProbeEdit::rc_new("write_data", (60.0, 220.0)),
            ProbeEdit::rc_new("write_enable", (60.0, 260.0)),
            RegFile::rc_new(
                "reg_file",
                (200.0, 200.0),
                Input::new("rs_addr", "out"),
                Input::new("rt_addr", "out"),
                Input::new("write_addr", "out"),
                Input::new("write_data", "out"),
                Input::new("write_enable", "out"),
            ),
            Probe::rc_new(
                "rs",
                (300.0, 120.0),
                Input::new("reg_file", reg_file_fields::RS_VALUE_OUT_ID),
            ),
            Probe::rc_new(
                "rt",
                (300.0, 160.0),
                Input::new("reg_file", reg_file_fields::RT_VALUE_OUT_ID),
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

fn autowire(mut cs: ComponentStore) -> ComponentStore {
    let mut wires: Vec<Rc<dyn EguiComponent>> = vec![];

    // for each component
    for destination_component in &cs.store {
        let dest_comp_id = destination_component.get_id_ports().0;
        // for each port in destination component
        for input_port in destination_component.get_id_ports().1.inputs.iter() {
            let source_port = &input_port.input;
            let dest_port_id = &input_port.port_id;

            // find component with correct source id
            let source_component = cs
                .store
                .iter()
                .filter(|comp| comp.get_id_ports().0 == source_port.id) // compare id
                .next()
                .unwrap();

            // create wire with correct source destination and positions
            let s_id = &source_port.id;
            let s_field = &source_port.field;
            let d_id = &dest_comp_id;
            let d_field = &dest_port_id;
            wires.push(Wire::rc_new(
                &format!("from_{}:{}_to_{}:{}", s_id, s_field, d_id, d_field),
                vec![source_component.get_pos(), destination_component.get_pos()],
                source_port.clone(),
            ))
        }
    }
    cs.store.append(&mut wires);
    return cs;
}
