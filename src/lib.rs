use std::sync::atomic::Ordering;

use screen::RESOLUTION;
use speedy2d::{Window, dimen::Vector2, window::{WindowCreationOptions, WindowPosition, WindowSize}};

mod entity;
mod screen;
mod ui;
mod world;
mod utility;

pub fn run() {
    let res = screen::get_resolution();
    let window: Window<String> = Window::new_with_user_events(
        "Sonar",
        WindowCreationOptions::new_windowed(
            WindowSize::PhysicalPixels(Vector2::new(
                res.0, res.1
            )),
            Some(WindowPosition::Center),
        ),
    )
    .unwrap();

    window.run_loop(screen::RedirectHandler::new(Box::new(
        screen::title::TitleScreen::new(),
    )));
}
