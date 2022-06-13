#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use skia_safe::{utils::text_utils::Align, Color};
use yalem::{widgets::*, App, Window};

fn main() {
    yalem::run(
        App::new().with_window(
            Window::new()
                .with_title("yalem Demo")
                .root(Padding::from(
                PaddingBuilder::new((0.0, 0.0, 0.0, 0.0)).child(List::from(
                    ListBuilder::new()
                        .child(Text::from(
                            TextBuilder::new("yalem Demo").color(Color::BLACK),
                        ))
                        .child(Button::from(
                            ButtonBuilder::new()
                                .background(Color::RED)
                                .child(Expand::from(
                                    ExpandBuilder::new().child(Text::from(
                                        TextBuilder::new("Expanded")
                                            .color(Color::from_rgb(240, 240, 240)),
                                    )),
                                )),
                        ))
                        .child(Button::from(
                            ButtonBuilder::new()
                                .background(Color::BLUE)
                                .child(Text::from(
                                    TextBuilder::new("Fixed width and height")
                                        .color(Color::from_rgb(240, 240, 240)),
                                ))
                                .width(200.0)
                                .height(50.0),
                        ))
                        .child(Padding::from(
                            PaddingBuilder::new((0.0, 0.0, 0.0, 0.0)).child(Button::from(
                                ButtonBuilder::new()
                                    .background(Color::BLACK)
                                    .child(Padding::from(
                                        PaddingBuilder::new((50.0, 50.0, 25.0, 25.0)).child(
                                            Text::from(
                                                TextBuilder::new("Fixed paddings")
                                                    .color(Color::YELLOW),
                                            ),
                                        ),
                                    )),
                            )),
                        ))
                        .child(Padding::from(
                            PaddingBuilder::new((10.0, 10.0, 10.0, 10.0)).child(Button::from(
                                ButtonBuilder::new()
                                    .background(Color::GREEN)
                                    .child(Expand::from(
                                        ExpandBuilder::new().child(Center::from(
                                            CenterBuilder::new().child(Text::from(
                                                TextBuilder::new(
                                                    "Expanded horizontally + centered + paddings",
                                                )
                                                .color(Color::BLACK)
                                                .align(Align::Center),
                                            )),
                                        )),
                                    )),
                            )),
                        ))
                        .child(Button::from(
                            ButtonBuilder::new()
                                .background(Color::MAGENTA)
                                .child(Expand::from(
                                    ExpandBuilder::new()
                                        .child(Center::from(
                                            CenterBuilder::new()
                                                .child(List::from(
                                                    ListBuilder::new()
                                                        .child(Text::from(
                                                            TextBuilder::new(
                                                                "Expanded both sides and centered",
                                                            )
                                                            .color(Color::BLACK)
                                                            .align(Align::Center),
                                                        ))
                                                        .child(Triangle::from(
                                                            TriangleBuilder::new(),
                                                        )),
                                                ))
                                                .direction(Direction::Both),
                                        ))
                                        .direction(Direction::Both),
                                )),
                        )),
                )),
            )),
        ),
    )
}
