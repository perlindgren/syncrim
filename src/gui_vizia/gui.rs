use crate::common::{ComponentStore, SimState, Simulator};
use crate::gui_vizia::{grid::Grid, menu::Menu, transport::Transport};
use vizia::prelude::*;

#[derive(Lens, Data, Clone)]
pub struct GuiData {
    pub path: String,
    pub clock: usize,
    pub simulator: Simulator,
    pub sim_state: SimState,
    // History, acts like a stack
    pub history: Vec<Vec<u32>>,
    pub pause: bool,
    pub is_saved: bool,
    pub show_about: bool,
    pub component_ids: Vec<String>,
    pub selected_id: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum GuiEvent {
    ReOpen,
    Clock,
    Reset,
    UnClock,
    Play,
    Pause,
    PlayToggle,
    Preferences,
    ShowAbout,
    HideAbout,
    // SelectComponent(usize),
}

impl Model for GuiData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        #[allow(clippy::single_match)]
        event.map(|window_event, meta| match window_event {
            // Intercept WindowClose event to show a dialog if not 'saved'.
            WindowEvent::WindowClose => {
                if !self.is_saved {
                    // self.show_dialog = true;
                    meta.consume();
                    self.is_saved = true;
                }
            }
            _ => {}
        });

        event.map(|app_event, _meta| match app_event {
            GuiEvent::ReOpen => {
                // Re-Open model
                println!("re-open path {}", self.path);
                let cs = Box::new(ComponentStore::load_file(&self.path));
                let (simulator, mut sim_state) = Simulator::new(&cs);

                // Initial clock to propagate constants
                let mut clock = 0;
                simulator.clock(&mut sim_state, &mut clock);
                let component_ids: Vec<String> = simulator
                    .ordered_components
                    .iter()
                    .map(|c| c.get_id_ports().0)
                    .collect();

                self.path = cs.path.clone();
                self.clock = clock;
                self.history = vec![];
                self.simulator = simulator;
                self.sim_state = sim_state;
                self.component_ids = component_ids;

                println!("re-opened");
            }
            GuiEvent::Clock => {
                // push current state
                self.history.push(self.sim_state.lens_values.clone());
                self.simulator.clock(&mut self.sim_state, &mut self.clock);
            }
            GuiEvent::Reset => {
                self.simulator.reset(&mut self.sim_state, &mut self.clock);
                // clear history
                self.history = vec![];
                // make sure its in paused mode
                self.pause = true;
            }
            GuiEvent::UnClock => {
                if let Some(state) = self.history.pop() {
                    // set old state
                    self.sim_state.lens_values = state;
                }
            }
            GuiEvent::Play => self.pause = false,
            GuiEvent::Pause => self.pause = true,
            GuiEvent::PlayToggle => self.pause = !self.pause,
            GuiEvent::Preferences => println!("Preferences"),
            GuiEvent::ShowAbout => self.show_about = true,
            GuiEvent::HideAbout => self.show_about = false,
            // GuiEvent::SelectComponent(index) => self.selected_id = *index,
        });
    }
}

const STYLE: &str = r#"
    .tt_shortcut {
        color: #c4c4c4;
    }


    submenu.file_menu > popup {
        width: 200px;
    }
    
"#;
// * {
//     border-width: 1px;
//     border-color: red;
//   }
// .menubar {
//     top: 100px
// }

// .menubutton {
//     top: 200px
// }

pub fn gui(cs: &ComponentStore) {
    let (simulator, mut sim_state) = Simulator::new(cs);
    // let simulator = Rc::new(simulator);
    // Initial clock to propagate constants
    let mut clock = 0;
    simulator.clock(&mut sim_state, &mut clock);
    let component_ids: Vec<String> = simulator
        .ordered_components
        .iter()
        .map(|c| c.get_id_ports().0)
        .collect();

    let path = cs.path.clone();
    Application::new(move |cx| {
        // Styling
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        // Create keymap
        crate::gui_vizia::keymap::new(cx);

        GuiData {
            path,
            clock,
            simulator,
            sim_state,
            history: vec![],
            pause: true,
            is_saved: false,
            show_about: false,
            component_ids,
            selected_id: 0,
        }
        .build(cx);

        VStack::new(cx, |cx| {
            // Menu
            Menu::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Transport::new(cx).size(Auto);
                    Label::new(
                        cx,
                        GuiData::sim_state
                            .then(SimState::lens_values)
                            .map(|v| format!("Raw state {:?}", v)),
                    )
                    .top(Stretch(1.0))
                    .bottom(Stretch(1.0))
                    .height(Auto);
                })
                .col_between(Pixels(10.0))
                .top(Stretch(1.0))
                .bottom(Stretch(1.0))
                .size(Auto);
            })
            .background_color(Color::lightgray())
            .height(Auto);

            Grid::new(cx, |cx| {
                // (re-)bind all components when simulator changed
                Binding::new(
                    cx,
                    GuiData::simulator.then(Simulator::ordered_components),
                    |cx, wrapper_oc| {
                        let oc = wrapper_oc.get(cx);
                        for c in oc {
                            // bind all components to be triggered by clock change
                            Binding::new(cx, GuiData::clock, move |cx, _| {
                                c.view(cx);
                            });
                        }
                    },
                )
            });

            //
            // HStack::new(cx, |cx| {
            // Component selector
            // PickList::new(cx, Gui::component_ids, Gui::selected_id, true)
            //     .on_select(|cx, index| cx.emit(GuiEvent::SelectComponent(index)))
            //     .width(Pixels(140.0));

            // About
            Popup::new(cx, GuiData::show_about, true, |cx| {
                Label::new(cx, "About").class("title");
                Label::new(cx, "SyncRim 0.1.0");
                Label::new(cx, "per.lindgren@ltu.se");

                Button::new(
                    cx,
                    |cx| cx.emit(GuiEvent::HideAbout),
                    |cx| Label::new(cx, "Ok"),
                )
                .class("accent");
            })
            .on_blur(|cx| cx.emit(GuiEvent::HideAbout))
            .class("modal");
        });
    })
    .title("SyncRim")
    .run();
}
