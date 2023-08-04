use std::path::PathBuf;
use syncrim::{
    common::{ComponentStore, Input},
    components::*,
    fern::fern_setup,
    signal::{BinOp, CmpOp, SignalExpr},
};

fn main() {
    fern_setup();
    let cs = ComponentStore {
        store: vec![
            ProbeEdit::rc_new("probe_edit", (100.0, 100.0)),
            ProbeEdit::rc_new("probe_edit2", (100.0, 140.0)),
            Add::rc_new(
                "add",
                (100.0, 200.0),
                Input::new("probe_edit", "out"),
                Input::new("probe_edit2", "out"),
            ),
            ProbeHalt::rc_new(
                "probe_halt",
                (250.0, 100.0),
                // just an example
                vec![
                    Input::new("probe_edit", "out"),
                    Input::new("probe_edit2", "out"),
                    Input::new("add", "out"),
                    Input::new("add", "overflow"),
                ],
                // just an example
                SignalExpr::BinOp(
                    BinOp::CmpOp(CmpOp::Eq),
                    Box::new(SignalExpr::Input(Input::new("probe_edit2", "out"))),
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
