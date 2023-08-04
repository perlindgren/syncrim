use std::path::PathBuf;
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
    fern::fern_setup,
    signal::SignalExpr,
};

fn main() {
    fern_setup();
    let cs = ComponentStore {
        store: vec![
            ProbeEdit::rc_new("probe_edit", (100.0, 100.0)),
            ProbeHalt::rc_new(
                "probe_halt",
                (250.0, 100.0),
                vec![Input::new("probe_edit", "out")],
                SignalExpr::Eq(
                    Box::new(SignalExpr::Input(Input::new("probe_edit", "out"))),
                    Box::new(SignalExpr::Constant(12.into())),
                ),
            ),
        ],
    };

    let path = PathBuf::from("probe_halt.json");
    cs.save_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::gui(&cs, &path).ok();

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&cs, &path);
}
