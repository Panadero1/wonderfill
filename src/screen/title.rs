use std::collections::HashMap;

use speedy2d::{
    color::Color,
    window::{MouseButton, UserEventSender, WindowHandler, WindowHelper},
    Graphics2D,
};

use crate::draw::ui::{button::Button, rect::rect_from_size, text};

use super::{game::GameScreen, options::OptionsScreen, Screen};

pub struct TitleScreen<'a> {
    new_screen: Option<Box<dyn Screen>>,
    mouse_up: bool,
    buttons: HashMap<&'a str, Button<'a>>,
    user_event_sender: Option<UserEventSender<String>>,
}

impl<'a> WindowHandler<String> for TitleScreen<'a> {
    fn on_draw(&mut self, helper: &mut WindowHelper<String>, graphics: &mut Graphics2D) {
        if self.user_event_sender.is_none() {
            self.user_event_sender = Some(helper.create_user_event_sender());
        }

        graphics.clear_screen(Color::BLUE);

        for (name, button) in self.buttons.iter() {
            button.draw(graphics);
        }
    }
    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<String>,
        virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
        scancode: speedy2d::window::KeyScancode,
    ) {
        if let Some(virtual_key_code) = virtual_key_code {
            match virtual_key_code {
                _ => (),
            }
        }
    }
    fn on_mouse_button_up(
        &mut self,
        helper: &mut WindowHelper<String>,
        button: speedy2d::window::MouseButton,
    ) {
        self.mouse_up = true;
    }
    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<String>, button: MouseButton) {
        if self.mouse_up {
            if let MouseButton::Left = button {
                for (_, button) in self.buttons.iter() {
                    let pos = super::get_mouse_pos();
                    let pos = (pos.0 as f32, pos.1 as f32);
                    button.eval_click(pos, &self.user_event_sender.as_ref().unwrap());
                }
            }
        }
        self.mouse_up = false;
    }
    fn on_resize(
        &mut self,
        helper: &mut WindowHelper<String>,
        size_pixels: speedy2d::dimen::Vector2<u32>,
    ) {
        super::set_resolution(size_pixels.x, size_pixels.y);

        let res = super::get_resolution();
        let center = (res.0 / 2, res.1 / 2);
        for (name, button) in self.buttons.iter_mut() {
            button.set_bounds(rect_from_size(
                button.width(),
                button.height(),
                button.get_pos(),
            ));
        }
    }
    fn on_start(
        &mut self,
        helper: &mut WindowHelper<String>,
        info: speedy2d::window::WindowStartupInfo,
    ) {
    }
    fn on_user_event(&mut self, helper: &mut WindowHelper<String>, user_event: String) {
        match &user_event[..] {
            "start" => {
                if let Ok(screen) = GameScreen::load() {
                    self.new_screen = Some(Box::new(screen));
                } else {
                    self.new_screen = Some(Box::new(GameScreen::new()));
                }
            }
            "options" => {
                self.new_screen = Some(Box::new(OptionsScreen::new()));
            }
            "quit" => {
                helper.terminate_loop();
            }
            _ => (),
        }
    }
}

impl<'a> Screen for TitleScreen<'a> {
    fn change_screen(&mut self) -> Option<Box<dyn Screen>> {
        if self.new_screen.is_some() {
            return self.new_screen.take();
        }
        None
    }
    fn init(&mut self, helper: &mut WindowHelper<String>) {}
}

impl<'a> TitleScreen<'a> {
    pub fn new() -> TitleScreen<'a> {
        let font = text::get_font();

        let mut buttons = HashMap::new();

        let res = super::get_resolution();

        let center = (res.0 / 2, res.1 / 2);

        let button_size = (300, 50);

        let button_padding: u32 = 10;

        let button_foreground = Color::BLACK;
        let button_background = Color::WHITE;

        buttons.insert(
            "start",
            Button::new(
                "Start",
                button_size.1 as f32,
                Box::new(|s: &UserEventSender<String>| {
                    s.send_event(String::from("start")).unwrap();
                }),
                button_size.0,
                button_size.1,
                button_background,
                button_foreground,
                font.clone(),
                Box::new(move || {
                    let res = super::get_resolution();
                    (res.0 / 2, res.1 / 2)
                }),
            ),
        );
        buttons.insert(
            "options",
            Button::new(
                "Options",
                button_size.1 as f32,
                Box::new(|s: &UserEventSender<String>| {
                    s.send_event(String::from("options")).unwrap();
                }),
                button_size.0,
                button_size.1,
                button_background,
                button_foreground,
                font.clone(),
                Box::new(move || {
                    let res = super::get_resolution();
                    (res.0 / 2, (res.1 / 2) + (button_padding + button_size.1))
                }),
            ),
        );
        buttons.insert(
            "quit",
            Button::new(
                "Quit",
                button_size.1 as f32,
                Box::new(|s: &UserEventSender<String>| {
                    s.send_event(String::from("quit")).unwrap();
                }),
                button_size.0,
                button_size.1,
                button_background,
                button_foreground,
                font,
                Box::new(move || {
                    let res = super::get_resolution();
                    (
                        res.0 / 2,
                        (res.1 / 2) + 2 * (button_padding + button_size.1),
                    )
                }),
            ),
        );

        TitleScreen {
            new_screen: None,
            mouse_up: true,
            buttons,
            user_event_sender: None,
        }
    }
}
