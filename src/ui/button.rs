use std::borrow::BorrowMut;

use speedy2d::{Graphics2D, color::Color, font::{Font, TextAlignment, TextLayout, TextOptions}, shape::Rectangle, window::UserEventSender};

use super::rect::rect_from_size;

pub struct Button<'a> {
    text: &'a str,
    font_size: f32,
    on_click: Box<dyn Fn(&UserEventSender<String>)>,
    bounds: Rectangle,
    background: Color,
    foreground: Color,
    font: Font,
}

impl<'a> Button<'a> {
    pub fn new<T: Fn(&UserEventSender<String>) + 'static>(
        text: &'a str,
        font_size: f32,
        on_click: T,
        width: u32,
        height: u32,
        pos: (u32, u32),
        background: Color,
        foreground: Color,
        font: Font,
    ) -> Button<'a> {
        Button {
            text,
            font_size,
            on_click: Box::new(on_click),
            bounds: rect_from_size(width, height, pos),
            background,
            foreground,
            font,
        }
    }
    pub fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_rectangle(self.bounds.clone(), self.background);

        graphics.draw_text(
            (self.bounds.top_left().x, self.bounds.top_left().y),
            self.foreground,
            &self.font.layout_text(
                self.text,
                self.font_size,
                TextOptions::new().with_wrap_to_width(
                    self.bounds.bottom_right().x - self.bounds.top_left().x,
                    TextAlignment::Center,
                ),
            ),
        );
    }
    pub fn set_bounds(&mut self, new_bounds: Rectangle) {
        self.bounds = new_bounds;
    }
    pub fn width(&self) -> u32 {
        self.bounds.width() as u32
    }
    pub fn height(&self) -> u32 {
        self.bounds.height() as u32
    }
    pub fn click(&self, sender: &UserEventSender<String>) {
        (self.on_click)(sender);
    }
    pub fn in_bounds(&self, pos: (f32, f32)) -> bool {
        let top_left = self.bounds.top_left();
        let bottom_right = self.bounds.bottom_right();
        pos.0 >= top_left.x && pos.1 >= top_left.y && pos.0 <= bottom_right.x && pos.1 <= bottom_right.y
    }
    pub fn eval_click(&self, pos: (f32, f32), sender: &UserEventSender<String>) {
        if self.in_bounds(pos) {
            self.click(sender);
        }
    }
}
