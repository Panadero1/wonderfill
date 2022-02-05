use std::collections::HashMap;

use speedy2d::{color::Color, shape::Rectangle, Graphics2D};

use serde::{Deserialize, Serialize};

use crate::{
    ui::img::{Img, ImgManager},
    world::time::Clock,
};

use super::time::NInstant;

#[derive(Debug)]
pub enum AnimationSelectError {
    NotFound,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Animation {
    src: Img,
    frame_size: (u16, u16),
    frames: HashMap<String, (bool, Vec<(u16, u16)>)>,
    default: (u16, u16),
    pub frame_loop: Option<(bool, Vec<(u16, u16)>)>,
    start: NInstant,
    iter_speed_ms: u16,
}
impl Animation {
    pub fn new(
        src: Img,
        frame_size: (u16, u16),
        frames: HashMap<String, (bool, Vec<(u16, u16)>)>,
        default: (u16, u16),
        iter_speed_ms: u16,
    ) -> Animation {
        Animation {
            src,
            frame_size,
            frames,
            default,
            frame_loop: None,
            start: NInstant::now(),
            iter_speed_ms,
        }
    }
    pub fn select(&mut self, anim_name: &str) -> Result<(), AnimationSelectError> {
        self.start = NInstant::now();
        self.intercept(anim_name)
    }
    pub fn intercept(&mut self, anim_name: &str) -> Result<(), AnimationSelectError> {
        match self.frames.get(anim_name) {
            Some(frames) => {
                self.frame_loop = Some(frames.clone());
                Ok(())
            }
            None => Err(AnimationSelectError::NotFound),
        }
    }
    pub fn deselect(&mut self) {
        self.frame_loop = None;
    }

    fn draw_helper(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        window_rect: Rectangle<f32>,
        do_offset: bool,
        color: Color,
    ) {
        if self.src.state.is_none() {
            self.src.init(graphics, manager);
        }
        let frame_pos = match &self.frame_loop {
            Some((do_loop, frame_loop)) => {
                let duration_ms = self.start.get_instant().elapsed().as_millis();
                let frame_count = duration_ms / self.iter_speed_ms as u128;
                if !do_loop && frame_count > frame_loop.len() as u128 {
                    self.deselect();
                    self.default
                } else {
                    let frame_offset = (frame_count % frame_loop.len() as u128) as usize;

                    let mut result = frame_loop[frame_offset];
                    if do_offset {
                        result.0 += 1;
                    }
                    result
                }
            }
            None => self.default,
        };

        if let Some(img) = &self.src.state {
            graphics.draw_rectangle_image_subset_tinted(
                window_rect,
                color,
                self.get_bounds_rect_from_pos(frame_pos),
                img,
            );
        }
    }

    pub fn draw_overworld(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        clock: &Clock,
        window_rect: Rectangle<f32>,
        color: Color,
    ) {
        self.draw_helper(graphics, manager, window_rect, !clock.is_day(), color);
    }

    pub fn draw(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        window_rect: Rectangle<f32>,
    ) {
        self.draw_helper(graphics, manager, window_rect, false, Color::WHITE);
    }

    fn get_bounds_rect_from_pos(&self, pos: (u16, u16)) -> Rectangle {
        let img_bounds = self.src.state.as_ref().unwrap().size();
        let top_left = (
            ((pos.0 as f32) * (self.frame_size.0 as f32 + 2.0) + 1.0) / (img_bounds.x as f32),
            ((pos.1 as f32) * (self.frame_size.1 as f32 + 2.0) + 1.0) / (img_bounds.y as f32),
        );
        let bottom_right = (
            ((pos.0 as f32 + 1.0) * (self.frame_size.0 as f32 + 2.0) - 1.0) / (img_bounds.x as f32),
            ((pos.1 as f32 + 1.0) * (self.frame_size.1 as f32 + 2.0) - 1.0) / (img_bounds.y as f32),
        );
        return Rectangle::from_tuples(top_left, bottom_right);
    }
}
