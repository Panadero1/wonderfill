use speedy2d::{
    dimen::Vector2,
    window::{WindowCreationOptions, WindowPosition, WindowSize},
    Window,
};

/// for dealing with anything related to the screen or window
pub mod screen;
pub mod ui;
pub mod utility;
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
