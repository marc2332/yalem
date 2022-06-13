#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::cell::Cell;

use skia_safe::{utils::text_utils::Align, Color};
use yalem::{widgets::*, App, Widget, Window};

static COUNTER: state::LocalStorage<Cell<u16>> = state::LocalStorage::new();

fn counter() -> impl Widget {
    Stateful::new(|_| {
        Box::new(
            Button::builder()
                .child(
                    Expand::builder()
                        .child(
                            Center::builder()
                                .child(
                                    Padding::builder((0.0, 0.0, 10.0, 0.0))
                                        .child(
                                            Text::builder(format!(
                                                "Counter -> {}",
                                                COUNTER.get().get()
                                            ))
                                            .align(Align::Center)
                                            .color(Color::BLACK)
                                            .build(),
                                        )
                                        .build(),
                                )
                                .direction(Direction::Both)
                                .build(),
                        )
                        .direction(Direction::Horizontal)
                        .build(),
                )
                .height(50.0)
                .build(),
        )
    })
}

fn increment() -> impl Widget {
    Button::builder()
        .child(
            Expand::builder()
                .child(
                    Center::builder()
                        .child(
                            Padding::builder((0.0, 0.0, 20.0, 0.0))
                                .child(
                                    Text::builder("Click me")
                                        .color(Color::YELLOW)
                                        .align(Align::Center)
                                        .build(),
                                )
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .background(Color::BLACK)
        .on_click(|| {
            let count = COUNTER.get();
            count.set(count.get() + 1);
        })
        .height(50.0)
        .build()
}

fn main() {
    COUNTER.set(|| Cell::new(1));

    yalem::run(
        App::new().with_window(
            Window::new()
                .with_title("yalem Demo")
                .root(
                    List::builder()
                        .child(counter())
                        .child(increment())
                        .build(),
                ),
        ),
    )
}
