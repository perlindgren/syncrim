use crate::common::Simulator;
use crate::gui_vizia::{GuiData, GuiEvent};
use vizia::{icons, prelude::*};
pub(crate) struct Transport {}

impl View for Transport {}

impl Transport {
    pub(crate) fn new(cx: &mut Context) -> Handle<Self> {
        View::build(Transport {}, cx, |cx| {
            HStack::new(cx, |cx| {
                // Reset
                Button::new(
                    cx,
                    //|ex| ex.emit(GuiEvent::Reset),
                    |cx| {
                        Label::new(cx, icons::ICON_PLAYER_SKIP_BACK)
                            .class("icon")
                            .on_press(move |cx| {
                                cx.emit(GuiEvent::Reset);
                            })
                    },
                )
                .tooltip(|cx| {
                    Tooltip::new(cx, |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Reset");
                            Label::new(cx, " Shift + Ctrl + F5").class("tt_shortcut");
                        })
                        .size(Auto);
                    })
                });

                // UnClock (step back)
                Button::new(
                    cx,
                    //|ex| ex.emit(GuiEvent::UnClock),
                    |cx| {
                        Label::new(cx, icons::ICON_CHEVRON_LEFT)
                            .class("icon")
                            .on_press(|cx| cx.emit(GuiEvent::UnClock))
                    },
                )
                .tooltip(|cx| {
                    Tooltip::new(cx, |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, "UnClock");
                            Label::new(cx, " Shift + F10").class("tt_shortcut");
                        })
                        .size(Auto);
                    })
                });

                // Clock (step forward)
                Button::new(
                    cx,
                    //|ex| ex.emit(GuiEvent::Clock),
                    |cx| {
                        Label::new(cx, icons::ICON_CHEVRON_RIGHT)
                            .class("icon")
                            .on_press(|cx| cx.emit(GuiEvent::Clock))
                    },
                )
                .tooltip(|cx| {
                    Tooltip::new(cx, |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Clock");
                            Label::new(cx, " F10").class("tt_shortcut");
                        })
                        .size(Auto);
                    })
                });

                // Play (continuous mode)
                Button::new(
                    cx,
                    //|ex| ex.emit(GuiEvent::Play),
                    |cx| {
                        Label::new(
                            cx,
                            GuiData::simulator.then(Simulator::running).map(|running| {
                                if *running {
                                    icons::ICON_PLAYER_PLAY_FILLED
                                } else {
                                    icons::ICON_PLAYER_PLAY
                                }
                            }),
                        )
                        .on_press(|cx| cx.emit(GuiEvent::Play))
                        .class("icon")
                    },
                )
                .tooltip(|cx| {
                    Tooltip::new(cx, |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Play");
                            Label::new(cx, " F5 (Toggle)").class("tt_shortcut");
                        })
                        .size(Auto);
                    })
                });

                // Pause (step mode)
                Button::new(
                    cx,
                    //|ex| ex.emit(GuiEvent::Pause),
                    |cx| {
                        Label::new(
                            cx,
                            GuiData::simulator.then(Simulator::running).map(|running| {
                                if *running {
                                    icons::ICON_PLAYER_PAUSE
                                } else {
                                    icons::ICON_PLAYER_PAUSE_FILLED
                                }
                            }),
                        )
                        .on_press(|cx| cx.emit(GuiEvent::Pause))
                        .class("icon")
                    },
                )
                .tooltip(|cx| {
                    Tooltip::new(cx, |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, "Pause");
                            Label::new(cx, " F5 (Toggle)").class("tt_shortcut");
                        })
                        .size(Auto);
                    })
                });
            })
            .col_between(Pixels(5.0))
            .size(Auto);
        })
    }
}
