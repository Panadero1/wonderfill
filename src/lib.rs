use speedy2d::{
    dimen::Vector2,
    window::{WindowCreationOptions, WindowPosition, WindowSize},
    Window,
};

use draw::screen;

/// all drawing functionality
pub mod draw;
/// useful misc things
pub mod utility;
/// everything to do with the game itself
pub mod world;

/// point of entry for the program
pub fn run() {
    let res = screen::get_resolution();

    let window: Window<String> = Window::new_with_user_events(
        "wonderfill",
        WindowCreationOptions::new_windowed(
            WindowSize::PhysicalPixels(Vector2::new(res.0, res.1)),
            Some(WindowPosition::Center),
        ),
    )
    .unwrap();

    window.run_loop(screen::RedirectHandler::new(Box::new(
        screen::title::TitleScreen::new(),
    )));
}
