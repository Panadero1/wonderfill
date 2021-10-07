use bitflags::bitflags;
use speedy2d::{
    color::Color,
    image::{ImageFileFormat, ImageSmoothingMode},
    window::{VirtualKeyCode, WindowHandler, WindowHelper},
    Graphics2D,
};

use crate::{entity::{Entity, player::{Player, PlayerHat}}, ui::img::get_image_handle};

use super::{camera::Camera, get_resolution, title::TitleScreen, Screen};

const CAMERA_SCALE: f32 = 100.0;

// bitflags! {
//     struct Input: u8 {
//         const NONE   = 0b00000000;
//         const LEFT   = 0b00000001;
//         const RIGHT  = 0b00000010;
//         const UP     = 0b00000100;
//         const DOWN   = 0b00001000;
//         const ATTACK = 0b00010000;
//     }
// }

pub struct Sprites {
    player: Player,
}

impl Sprites {
    pub fn init(graphics: &mut Graphics2D) -> Sprites {
        Sprites {
            player: Player::new(get_image_handle(graphics, "assets\\img\\player.png").unwrap()),
        }
    }
}

pub struct GameScreen {
    new_screen: Option<Box<dyn Screen>>,
    // current_input: Input,
    camera: Camera,
    sprites: Option<Sprites>,
}

impl WindowHandler<String> for GameScreen {
    fn on_draw(&mut self, helper: &mut WindowHelper<String>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::GREEN);

        if let Some(sprites) = &mut self.sprites {
            sprites.player.update();
            sprites.player.draw(graphics, &self.camera);
        } else {
            self.sprites = Some(Sprites::init(graphics));
        }

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
                },
                _ => {
                    if let Some(sprites) = &mut self.sprites {
                        sprites.player.set_hat(match virtual_key_code {
                            VirtualKeyCode::A => PlayerHat::Acid,
                            VirtualKeyCode::B => PlayerHat::Helmet,
                            VirtualKeyCode::C => PlayerHat::Teardrop,
                            _ => PlayerHat::None,
                        })
                    }
                },
                // _ => {
                //     self.current_input |= match virtual_key_code {
                //         VirtualKeyCode::Left => Input::LEFT,
                //         VirtualKeyCode::Up => Input::UP,
                //         VirtualKeyCode::Down => Input::DOWN,
                //         VirtualKeyCode::Right => Input::RIGHT,
                //         VirtualKeyCode::X => Input::ATTACK,
                //         _ => Input::NONE,
                //     }
                // }
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
            // self.current_input &= !match virtual_key_code {
            //     VirtualKeyCode::Right => Input::RIGHT,
            //     VirtualKeyCode::Left => Input::LEFT,
            //     VirtualKeyCode::Up => Input::UP,
            //     VirtualKeyCode::Down => Input::DOWN,
            //     VirtualKeyCode::X => Input::ATTACK,
            //     _ => Input::NONE,
            // }
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
        GameScreen {
            new_screen: None,
            // current_input: Input { bits: 0 },
            camera: Camera::new(
                (0.0, 0.0).into(),
                res.0 as f32 / CAMERA_SCALE,
                res.1 as f32 / CAMERA_SCALE,
            ),
            sprites: None,
        }
    }
}
