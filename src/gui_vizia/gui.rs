use crate::common::{ComponentStore, SimState, Simulator};
use crate::gui_vizia::{grid::Grid, menu::Menu, transport::Transport};

use std::rc::Rc;
use vizia::prelude::*;

#[derive(Lens)]
pub struct Gui {
    pub simulator: Rc<Simulator>,
    pub state: SimState,
    // History, acts like a stack
    pub history: Vec<Vec<u32>>,
    pub pause: bool,
    pub is_saved: bool,
    pub show_about: bool,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum GuiEvent {
    Clock,
    Reset,
    UnClock,
    Play,
    Pause,
    PlayToggle,
    Preferences,
    ShowAbout,
    HideAbout,
}

impl Model for Gui {
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
            GuiEvent::Clock => {
                // push current state
                self.history.push(self.state.lens_values.clone());
                self.simulator.clock(&mut self.state);
            }
            GuiEvent::Reset => {
                self.simulator.reset(&mut self.state);
                // clear history
                self.history = vec![];
                // make sure its in paused mode
                self.pause = true;
            }
            GuiEvent::UnClock => {
                if let Some(state) = self.history.pop() {
                    // set old state
                    self.state.lens_values = state;
                }
            }
            GuiEvent::Play => self.pause = false,
            GuiEvent::Pause => self.pause = true,
            GuiEvent::PlayToggle => self.pause = !self.pause,
            GuiEvent::Preferences => println!("Preferences"),
            GuiEvent::ShowAbout => self.show_about = true,
            GuiEvent::HideAbout => self.show_about = false,
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
    let simulator = Rc::new(simulator);
    // Initial clock to propagate constants
    simulator.clock(&mut sim_state);

    Application::new(move |cx| {
        // Styling
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        // Create keymap
        crate::gui_vizia::keymap::new(cx);

        Gui {
            simulator: simulator.clone(),
            state: sim_state,
            history: vec![],
            pause: true,
            is_saved: false,
            show_about: false,
        }
        .build(cx);

        VStack::new(cx, |cx| {
            // Menu
            Menu::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Transport::new(cx).size(Auto);
                    Label::new(
                        cx,
                        Gui::state
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

            // Grid
            Grid::new(cx, |cx| {
                for c in &simulator.ordered_components {
                    c.view(cx, simulator.clone());
                }
            });

            //
            Popup::new(cx, Gui::show_about, true, |cx| {
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
