use std::{collections::HashMap, ops::Not};

use bitflags::bitflags;
use speedy2d::{
    color::Color,
    image::{ImageFileFormat, ImageSmoothingMode},
    window::{VirtualKeyCode, WindowHandler, WindowHelper},
    Graphics2D,
};

use crate::{entity::{
        player::{Player, PlayerHat},
        Entity,
    }, ui::img::{Img, ImgManager, get_image_handle}, utility::animation::Animation, world::{Region, Tile, World}};

use super::{camera::Camera, get_resolution, title::TitleScreen, Screen};

// Larger number -> smaller bounds
const CAMERA_SCALE: f32 = 50.0;

bitflags! {
    struct Input: u8 {
        const NONE   = 0b00000000;
        const LEFT   = 0b00000001;
        const RIGHT  = 0b00000010;
        const UP     = 0b00000100;
        const DOWN   = 0b00001000;
        const ATTACK = 0b00010000;
    }
}
impl From<VirtualKeyCode> for Input {
    fn from(key_code: VirtualKeyCode) -> Self {
        match key_code {
            VirtualKeyCode::Up => Input::UP,
            VirtualKeyCode::Left => Input::LEFT,
            VirtualKeyCode::Down => Input::DOWN,
            VirtualKeyCode::Right => Input::RIGHT,
            VirtualKeyCode::X => Input::ATTACK,
            _ => Input::NONE,
        }
    }
}
impl Into<Option<VirtualKeyCode>> for Input {
    fn into(self) -> Option<VirtualKeyCode> {
        match self {
            Input::NONE => None,
            _ => Some(match self {
                Input::UP => VirtualKeyCode::Up,
                Input::LEFT => VirtualKeyCode::Left,
                Input::DOWN => VirtualKeyCode::Down,
                Input::RIGHT => VirtualKeyCode::Right,
                Input::ATTACK => VirtualKeyCode::X,
                _ => panic!("Forgot to implement keycode mappings"), // never occurs
            }),
        }
    }
}

pub struct GameScreen {
    new_screen: Option<Box<dyn Screen>>,
    current_input: Input,
    camera: Camera,
    world: World,
    img_manager: ImgManager,
}

impl WindowHandler<String> for GameScreen {
    fn on_draw(&mut self, helper: &mut WindowHelper<String>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::GREEN);

        self.world.player.update();

        for region in &mut self.world.regions {
            region.draw(graphics, &mut self.img_manager, &self.camera);
        }

        self.world.player.draw(graphics, &mut self.img_manager, &self.camera);

        helper.request_redraw();
    }
    fn on_key_down(
        &mut self,
        _helper: &mut WindowHelper<String>,
        virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
        _scancode: speedy2d::window::KeyScancode,
    ) {
        if let Some(virtual_key_code) = virtual_key_code {
            match virtual_key_code {
                VirtualKeyCode::Escape => {
                    self.new_screen = Some(Box::new(TitleScreen::new()));
                }
                // _ => {
                //     self.world.player.set_hat(match virtual_key_code {
                //         VirtualKeyCode::A => PlayerHat::Acid,
                //         VirtualKeyCode::B => PlayerHat::Helmet,
                //         VirtualKeyCode::C => PlayerHat::Teardrop,
                //         _ => PlayerHat::None,
                //     });
                // },
                _ => {
                    if !self.current_input.contains(virtual_key_code.into()) {
                        self.world.player.moove(match virtual_key_code {
                            VirtualKeyCode::Up => (0.0, -1.0),
                            VirtualKeyCode::Left => (-1.0, 0.0),
                            VirtualKeyCode::Down => (0.0, 1.0),
                            VirtualKeyCode::Right => (1.0, 0.0),
                            _ => (0.0, 0.0),
                        }.into())
                    }
                    self.current_input |= virtual_key_code.into();
                }
            }
        }
    }
    fn on_key_up(
        &mut self,
        _helper: &mut WindowHelper<String>,
        virtual_key_code: Option<VirtualKeyCode>,
        _scancode: speedy2d::window::KeyScancode,
    ) {
        if let Some(virtual_key_code) = virtual_key_code {
            let result: Input = virtual_key_code.into();
            self.current_input &= !result;
        }
    }
    fn on_resize(
        &mut self,
        _helper: &mut WindowHelper<String>,
        size_pixels: speedy2d::dimen::Vector2<u32>,
    ) {
        self.camera.width = size_pixels.x as f32 / CAMERA_SCALE;
        self.camera.height = size_pixels.y as f32 / CAMERA_SCALE;
    }
}

impl Screen for GameScreen {
    fn change_screen(&mut self) -> Option<Box<dyn Screen>> {
        if self.new_screen.is_some() {
            return self.new_screen.take();
        }
        None
    }
}

impl GameScreen {
    pub fn new() -> GameScreen {
        let res = get_resolution();

        let size = 10;

        let mut tiles = Vec::with_capacity(size * size);

        let mut frames = HashMap::new();

        frames.insert(String::from(""), (true, vec![(0, 0)]));

        let tile_anim = Animation::new(
            Img::new(String::from("assets\\img\\tiles.png")),
            (7, 7),
            frames,
            (0, 0),
            100,
        );

        for y in 0..size {
            for x in 0..size {
                tiles.push(Tile::new((x as f32, y as f32).into(), tile_anim.clone()));
            }
        }

        GameScreen {
            new_screen: None,
            current_input: Input { bits: 0 },
            camera: Camera::new(
                (0.0, 0.0).into(),
                res.0 as f32 / CAMERA_SCALE,
                res.1 as f32 / CAMERA_SCALE,
            ),
            world: World::new(vec![Region::new(tiles)], Player::new()),
            img_manager: ImgManager::new()
        }
    }
}
