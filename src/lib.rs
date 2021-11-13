use speedy2d::{
    dimen::Vector2,
    window::{WindowCreationOptions, WindowPosition, WindowSize},
    Window,
};

mod screen;
mod ui;
mod utility;
mod world;

pub fn run() {
    let res = screen::get_resolution();
    let window: Window<String> = Window::new_with_user_events(
        "Sonar",
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
