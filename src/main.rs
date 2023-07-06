use clap::Parser;
use std::path::PathBuf;
use syncrim::common::ComponentStore;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the model to load on startup
    #[arg(short, long)]
    model: String,
}

fn main() {
    let args = Args::parse();
    let path = PathBuf::from(args.model);

    let _cs = ComponentStore::load_file(&path);

    #[cfg(feature = "gui-egui")]
    syncrim::gui_egui::egui::gui(&cs);

    #[cfg(feature = "gui-vizia")]
    syncrim::gui_vizia::gui(&_cs, &path);
}
