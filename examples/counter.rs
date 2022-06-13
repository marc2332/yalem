#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::cell::Cell;

use skia_safe::Color;
use yalem::{widgets::*, App, Window};

static COUNTER: state::LocalStorage<Cell<u16>> = state::LocalStorage::new();

fn main() {
    COUNTER.set(|| Cell::new(1));

    yalem::run(
        App::new().with_window(
            Window::new()
                .with_title("yalem Demo")
                .root(
                    List::builder()
                        .child(
                            Button::builder()
                                .child(Stateful::new(|_| {
                                    Box::new(
                                        Text::builder(format!("Click -> {}", COUNTER.get().get()))
                                            .color(Color::YELLOW)
                                            .build(),
                                    )
                                }))
                                .background(Color::BLACK)
                                .on_click(|| {
                                    let count = COUNTER.get();
                                    count.set(count.get() + 1);
                                })
                                .build(),
                        )
                        .build(),
                ),
        ),
    )
}
