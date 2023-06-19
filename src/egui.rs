use crate::common::{ComponentStore, SimState, Simulator};
use eframe::egui;
use std::rc::Rc;

pub struct Gui {
    pub simulator: Rc<Simulator>,
    pub state: SimState,
    // History, acts like a stack
    pub history: Vec<Vec<u32>>,
    pub scale: f32,
}

pub fn gui(cs: &ComponentStore) -> Result<(), eframe::Error> {
    let (simulator, mut sim_state) = Simulator::new(cs);
    let simulator = Rc::new(simulator);
    simulator.clock(&mut sim_state);
    let options = eframe::NativeOptions::default();
    let gui = Gui {
        simulator: simulator.clone(),
        state: sim_state,
        history: vec![],
        scale: 1.0f32,
    };
    eframe::run_native("SyncRim", options, Box::new(|_cc| Box::new(gui)))
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame = egui::Frame::none().fill(egui::Color32::WHITE);
        let start = egui::Vec2 { x: 0f32, y: 0f32 };
        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            for c in &self.simulator.ordered_components {
                c.render(ui, self.simulator.clone(), start, self.scale);
            }
        });
        egui::TopBottomPanel::top("topBar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("▶").clicked() {
                    //self.history.push(self.state.lens_values.clone());
                    //self.simulator.clock(&mut self.state);
                    println!("run!");
                }
                if ui.button("■").clicked() {
                    //self.history.push(self.state.lens_values.clone());
                    //self.simulator.clock(&mut self.state);
                    println!("paused!");
                }
                if ui.button("⏮").clicked() {
                    //self.history.push(self.state.lens_values.clone());
                    //self.simulator.clock(&mut self.state);
                    println!("stepped back once!");
                }
                if ui.button("⏭").clicked() {
                    //self.history.push(self.state.lens_values.clone());
                    self.simulator.clock(&mut self.state);
                    println!("stepped once!");
                }
                //ui.label("File");
                if ui.button("Scale").clicked() {
                    match self.scale {
                        x if (0.2f32..0.7f32).contains(&x) => self.scale = 1.0f32,
                        x if (0.7f32..1.3f32).contains(&x) => self.scale = 2.0f32,
                        x if (1.7f32..2.3f32).contains(&x) => self.scale = 3.0f32,
                        x if (2.7f32..3.2f32).contains(&x) => self.scale = 4.0f32,
                        _ => self.scale = 0.5f32,
                    }
                }
            });
        });

        egui::SidePanel::left("leftGui").show(ctx, |ui| {
            // Left panel code here
        });

        // (This now runs at the start of the function, since otherwise it will draw on top the
        // other elements)
        // For getting the correct offset for our drawing we need to get the two panels above
        // and to the side of the ui
        /*
        let frame = egui::Frame::none().fill(egui::Color32::WHITE);
        let top = egui::containers::panel::PanelState::load(ctx, egui::Id::from("topBar")).unwrap();
        let side =
            egui::containers::panel::PanelState::load(ctx, egui::Id::from("leftGui")).unwrap();
        let start = egui::Vec2 {
            x: side.rect.max.x,
            y: top.rect.max.y,
        };
        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            for c in &self.simulator.ordered_components {
                c.render(ui, self.simulator.clone(), start, self.scale);
            }
        });
        */
    }
}
