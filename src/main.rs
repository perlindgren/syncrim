use clap::Parser;
use syncrim::{common::ComponentStore, gui_vizia::gui};

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
    let cs = ComponentStore::load_file(&args.model);
    gui(&cs);
}
