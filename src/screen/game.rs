use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{self, BufReader},
    ops::Not,
    path::{Path, PathBuf},
};

use bitflags::bitflags;
use rand::Rng;
use serde::Serialize;
use speedy2d::{
    color::Color,
    image::{ImageFileFormat, ImageSmoothingMode},
    window::{VirtualKeyCode, WindowHandler, WindowHelper},
    Graphics2D,
};

use crate::{entity::{
        player::{Player, PlayerHat},
        tile::{
            test_ground::{self, TestGround},
            test_pillar::TestPillar,
            Tile,
        },
        Entity,
    }, ui::img::{get_image_handle, Img, ImgManager}, utility::animation::Animation, world::{self, Region, World, time::Clock}};

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
    world: World,
    img_manager: ImgManager,
}

impl WindowHandler<String> for GameScreen {
    fn on_draw(&mut self, helper: &mut WindowHelper<String>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::GRAY);

        self.world.player.update();

        for region in &mut self.world.regions {
            region.draw_before_player(
                graphics,
                &mut self.img_manager,
                &self.world.camera,
                self.world.player.get_pos(),
            );
        }

        self.world
            .player
            .draw(graphics, &mut self.img_manager, &self.world.camera);

        for region in &mut self.world.regions {
            region.draw_after_player(
                graphics,
                &mut self.img_manager,
                &self.world.camera,
                self.world.player.get_pos(),
            );
        }
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
                    self.save_world();
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
                        let move_pos = match virtual_key_code {
                            VirtualKeyCode::Up => (0.0, -1.0),
                            VirtualKeyCode::Left => (-1.0, 0.0),
                            VirtualKeyCode::Down => (0.0, 1.0),
                            VirtualKeyCode::Right => (1.0, 0.0),
                            _ => (0.0, 0.0),
                        }
                        .into();
                        self.world.player.moove(move_pos);
                        if let Some(tile) = self
                            .world
                            .regions
                            .get_mut(0)
                            .unwrap()
                            .tile_at_pos(self.world.player.get_pos())
                        {
                            tile.on_player_enter(&mut self.world.player, move_pos);
                        }
                        self.world
                            .camera
                            .moove(self.world.player.get_pos() - self.world.camera.pos);
                        self.world.update();
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
        self.world.camera.width = size_pixels.x as f32 / CAMERA_SCALE;
        self.world.camera.height = size_pixels.y as f32 / CAMERA_SCALE;
    }
    fn on_start(
        &mut self,
        helper: &mut WindowHelper<String>,
        info: speedy2d::window::WindowStartupInfo,
    ) {
    }
}

impl<'a> Screen for GameScreen {
    fn change_screen(&mut self) -> Option<Box<dyn Screen>> {
        if self.new_screen.is_some() {
            return self.new_screen.take();
        }
        None
    }
    fn init(&mut self, helper: &mut WindowHelper<String>) {
        self.on_resize(helper, get_resolution().into());
    }
}

impl GameScreen {
    pub fn new() -> GameScreen {
        let res = get_resolution();

        let size = 50;

        let mut tiles = Vec::with_capacity(size * size);

        let mut r = rand::thread_rng();

        for y in 0..size {
            for x in 0..size {
                let pos = (x as f32, y as f32).into();
                let mut tile: Box<dyn Tile> = if r.gen_ratio(1, 10) {
                    Box::new(TestPillar::new(pos))
                } else {
                    Box::new(TestGround::new(pos))
                };

                tile.get_anim().select("light").unwrap();

                tiles.push(tile);
            }
        }

        GameScreen::with_world(World::new(
            vec![Region::new(tiles)],
            Player::new(),
            Camera::new(
                (0.0, 0.0).into(),
                res.0 as f32 / CAMERA_SCALE,
                res.1 as f32 / CAMERA_SCALE,
            ),
            Clock::new(),
        ))
    }

    pub fn load() -> io::Result<GameScreen> {
        Ok(GameScreen::with_world(GameScreen::load_world()?))
    }

    fn with_world(world: World) -> GameScreen {
        GameScreen {
            new_screen: None,
            current_input: Input::NONE,
            world,
            img_manager: ImgManager::new(),
        }
    }

    fn save_world(&self) {
        let path = GameScreen::get_file_path();
        let file = fs::File::create(path).unwrap();
        let writer = io::LineWriter::new(file);
        serde_json::to_writer(writer, &self.world).unwrap();
    }

    fn get_file_path() -> PathBuf {
        let dir = env::current_dir().unwrap();
        let path = Path::new(&dir).join("saves\\");
        if !path.exists() {
            fs::create_dir(&path).unwrap();
        }
        path.join("save.json")
    }

    fn load_world() -> io::Result<World> {
        let path = GameScreen::get_file_path();
        let file: File = File::open(path)?;
        let rdr = BufReader::new(file);

        Ok(serde_json::from_reader(rdr)?)
    }
}
