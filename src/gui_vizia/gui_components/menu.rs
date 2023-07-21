use crate::gui_vizia::GuiEvent;
use vizia::{icons, prelude::*};

use log::*;

pub(crate) struct Menu {}
impl View for Menu {}

impl Menu {
    pub fn new<F>(cx: &mut Context, content: F) -> Handle<'_, Self>
    where
        F: FnOnce(&mut Context),
    {
        View::build(Menu {}, cx, |cx| {
            HStack::new(cx, |cx| {
                // Menu bar
                MenuBar::new(cx, |cx| {
                    Submenu::new(
                        cx,
                        |cx| Label::new(cx, "File"),
                        |cx| {
                            MenuButton::new(
                                cx,
                                |_| trace!("File"),
                                |cx| {
                                    HStack::new(cx, |cx| {
                                        Label::new(cx, "New");
                                        Label::new(cx, "Ctrl + N").class("shortcut");
                                    })
                                },
                            );
                            MenuButton::new(
                                cx,
                                |cx| {
                                    trace!("Open");
                                    cx.emit(GuiEvent::Open);
                                },
                                |cx| {
                                    HStack::new(cx, |cx| {
                                        Label::new(cx, "Open");
                                        Label::new(cx, "Ctrl + O").class("shortcut");
                                    })
                                },
                            );
                            MenuButton::new(
                                cx,
                                |cx| {
                                    trace!("Re-Open");
                                    cx.emit(GuiEvent::ReOpen);
                                },
                                |cx| {
                                    HStack::new(cx, |cx| {
                                        Label::new(cx, "Re Open");
                                        Label::new(cx, "Ctrl + R").class("shortcut");
                                    })
                                },
                            );
                            Submenu::new(
                                cx,
                                |cx| Label::new(cx, "Open Recent"),
                                |cx| {
                                    MenuButton::new(
                                        cx,
                                        |_| trace!("Doc 1"),
                                        |cx| Label::new(cx, "Doc 1"),
                                    );
                                    Submenu::new(
                                        cx,
                                        |cx| Label::new(cx, "Doc 2"),
                                        |cx| {
                                            MenuButton::new(
                                                cx,
                                                |_| trace!("Version 1"),
                                                |cx| Label::new(cx, "Version 1"),
                                            );
                                            MenuButton::new(
                                                cx,
                                                |_| trace!("Version 2"),
                                                |cx| Label::new(cx, "Version 2"),
                                            );
                                            MenuButton::new(
                                                cx,
                                                |_| trace!("Version 3"),
                                                |cx| Label::new(cx, "Version 3"),
                                            );
                                        },
                                    );
                                    MenuButton::new(
                                        cx,
                                        |_| trace!("Doc 3"),
                                        |cx| Label::new(cx, "Doc 3"),
                                    );
                                },
                            );
                            MenuDivider::new(cx);
                            MenuButton::new(cx, |_| trace!("Save"), |cx| Label::new(cx, "Save"));
                            MenuButton::new(
                                cx,
                                |_| trace!("Save As"),
                                |cx| Label::new(cx, "Save As"),
                            );
                            MenuDivider::new(cx);
                            MenuButton::new(
                                cx,
                                |cx| cx.emit(GuiEvent::Preferences),
                                |cx| {
                                    HStack::new(cx, |cx| {
                                        Label::new(cx, "Preferences");
                                        Label::new(cx, "Ctrl + P").class("shortcut");
                                    })
                                },
                            );
                            MenuButton::new(
                                cx,
                                |cx| cx.emit(WindowEvent::WindowClose),
                                |cx| {
                                    HStack::new(cx, |cx| {
                                        Label::new(cx, "Quit");
                                        Label::new(cx, "Alt + F4").class("shortcut");
                                    })
                                },
                            );
                        },
                    )
                    .class("file_menu");

                    Submenu::new(
                        cx,
                        |cx| Label::new(cx, "Edit"),
                        |cx| {
                            MenuButton::new(
                                cx,
                                |_| trace!("Cut"),
                                |cx| {
                                    HStack::new(cx, |cx| {
                                        Label::new(cx, icons::ICON_CUT).class("icon");
                                        Label::new(cx, "Cut");
                                    })
                                },
                            );
                            MenuButton::new(
                                cx,
                                |_| trace!("Copy"),
                                |cx| {
                                    HStack::new(cx, |cx| {
                                        Label::new(cx, icons::ICON_COPY).class("icon");
                                        Label::new(cx, "Copy");
                                    })
                                },
                            );
                            MenuButton::new(
                                cx,
                                |_| trace!("Paste"),
                                |cx| {
                                    HStack::new(cx, |cx| {
                                        Label::new(cx, icons::ICON_CLIPBOARD).class("icon");
                                        Label::new(cx, "Paste");
                                    })
                                },
                            );
                        },
                    );

                    Submenu::new(
                        cx,
                        |cx| Label::new(cx, "View"),
                        |cx| {
                            MenuButton::new(
                                cx,
                                |_| trace!("Zoom In"),
                                |cx| Label::new(cx, "Zoom In"),
                            );
                            MenuButton::new(
                                cx,
                                |_| trace!("Zoom Out"),
                                |cx| Label::new(cx, "Zoom Out"),
                            );
                            Submenu::new(
                                cx,
                                |cx| Label::new(cx, "Zoom Level"),
                                |cx| {
                                    MenuButton::new(
                                        cx,
                                        |_| trace!("10%"),
                                        |cx| Label::new(cx, "10%"),
                                    );
                                    MenuButton::new(
                                        cx,
                                        |_| trace!("20%"),
                                        |cx| Label::new(cx, "20%"),
                                    );
                                    MenuButton::new(
                                        cx,
                                        |_| trace!("50%"),
                                        |cx| Label::new(cx, "50%"),
                                    );
                                    MenuButton::new(
                                        cx,
                                        |_| trace!("100%"),
                                        |cx| Label::new(cx, "100%"),
                                    );
                                    MenuButton::new(
                                        cx,
                                        |_| trace!("150%"),
                                        |cx| Label::new(cx, "150%"),
                                    );
                                    MenuButton::new(
                                        cx,
                                        |_| trace!("200%"),
                                        |cx| Label::new(cx, "200%"),
                                    );
                                },
                            );
                        },
                    );

                    Submenu::new(
                        cx,
                        |cx| Label::new(cx, "Help"),
                        |cx| {
                            MenuButton::new(
                                cx,
                                |_| trace!("Show License"),
                                |cx| Label::new(cx, "Show License"),
                            );
                            MenuButton::new(
                                cx,
                                |cx| {
                                    cx.emit(GuiEvent::ShowAbout);
                                },
                                |cx| Label::new(cx, "About"),
                            );
                        },
                    );
                })
                .size(Auto);
                content(cx);
            })
            .height(Auto);
        })
    }
}
