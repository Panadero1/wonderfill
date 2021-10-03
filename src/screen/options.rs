use std::{
    collections::{HashMap, HashSet},
    sync::atomic::Ordering,
};

use speedy2d::{
    color::Color,
    font::{Font, TextAlignment, TextLayout, TextOptions},
    shape::Rectangle,
    window::{MouseButton, UserEventSender, VirtualKeyCode, WindowHandler, WindowHelper},
    Graphics2D,
};

use crate::{
    screen::RESOLUTION,
    ui::{button::Button, rect::rect_from_size},
};

use super::{RedirectHandler, Screen, game::GameScreen, title::TitleScreen};

pub struct OptionsScreen<'a> {
    new_screen: Option<Box<dyn Screen>>,
    mouse_up: bool,
    buttons: HashMap<&'a str, Button<'a>>,
    user_event_sender: Option<UserEventSender<String>>,
}

impl<'a> WindowHandler<String> for OptionsScreen<'a> {
    fn on_draw(&mut self, helper: &mut WindowHelper<String>, graphics: &mut Graphics2D) {
        if self.user_event_sender.is_none() {
            self.user_event_sender = Some(helper.create_user_event_sender());
        }

        graphics.clear_screen(Color::BLUE);

        for (name, button) in self.buttons.iter() {
            button.draw(graphics);
        }

        helper.request_redraw();
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
                match *name {
                    "back" => (center.0, center.1 + 160),
                    _ => panic!("Not implemented button center scheme!!")
                },
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
            "back" => {
                self.new_screen = Some(Box::new(TitleScreen::new()));
            },
            _ => (),
        }
    }
}

impl<'a> Screen for OptionsScreen<'a> {
    fn change_screen(&mut self) -> Option<Box<dyn Screen>> {
        if self.new_screen.is_some() {
            return self.new_screen.take();
        }
        None
    }
}

impl<'a> OptionsScreen<'a> {
    pub fn new() -> OptionsScreen<'a> {
        let font = Font::new(include_bytes!("../../assets/font/Cabal-w5j3.ttf")).unwrap();

        let mut buttons = HashMap::new();

        let res = super::get_resolution();

        let center = (res.0 / 2, res.1 / 2);
        buttons.insert(
            "back",
            Button::new(
                "Back",
                64.0,
                Box::new(|s: &UserEventSender<String>| {
                    s.send_event(String::from("back")).unwrap();
                }),
                180,
                60,
                (center.0, center.1 + 160),
                Color::WHITE,
                Color::BLACK,
                font,
            ),
        );

        OptionsScreen {
            new_screen: None,
            mouse_up: true,
            buttons,
            user_event_sender: None,
        }
    }
}
