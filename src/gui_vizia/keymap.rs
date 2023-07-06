use crate::gui_vizia::GuiEvent;
use vizia::prelude::*;

// Dummy action for now
#[derive(Debug, PartialEq, Copy, Clone)]
enum Action {
    Dummy,
}

pub fn init_keymap(cx: &mut Context) {
    Keymap::from(vec![
        (
            KeyChord::new(Modifiers::empty(), Code::F5),
            KeymapEntry::new(Action::Dummy, |ex| {
                println!("Action F5");
                ex.emit(GuiEvent::PlayToggle);
            }),
        ),
        (
            KeyChord::new(Modifiers::SHIFT, Code::F5),
            KeymapEntry::new(Action::Dummy, |ex| {
                println!("Action Shift F5");
                ex.emit(GuiEvent::Pause);
            }),
        ),
        (
            KeyChord::new(Modifiers::SHIFT | Modifiers::CTRL, Code::F5),
            KeymapEntry::new(Action::Dummy, |ex| {
                println!("Action Shift Ctrl F5");
                ex.emit(GuiEvent::Reset);
            }),
        ),
        (
            KeyChord::new(Modifiers::empty(), Code::F10),
            KeymapEntry::new(Action::Dummy, |ex| {
                println!("Action F10");
                ex.emit(GuiEvent::Clock);
            }),
        ),
        (
            KeyChord::new(Modifiers::SHIFT, Code::F10),
            KeymapEntry::new(Action::Dummy, |ex| {
                println!("Action Shift F10");
                ex.emit(GuiEvent::UnClock);
            }),
        ),
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyP),
            KeymapEntry::new(Action::Dummy, |ex| {
                println!("Action Ctrl P");
                ex.emit(GuiEvent::Preferences);
            }),
        ),
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyR),
            KeymapEntry::new(Action::Dummy, |ex| {
                println!("Action Ctrl R");
                ex.emit(GuiEvent::ReOpen);
            }),
        ),
        (
            KeyChord::new(Modifiers::CTRL, Code::KeyO),
            KeymapEntry::new(Action::Dummy, |ex| {
                println!("Action Ctrl O");
                ex.emit(GuiEvent::Open);
            }),
        ),
    ])
    .build(cx);
}
