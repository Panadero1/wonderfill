use std::{
    env,
    fs::{self, File},
    io::{self, BufReader},
    path::{Path, PathBuf},
};

use bitflags::bitflags;

use speedy2d::{Graphics2D, color::Color, window::{MouseButton, VirtualKeyCode, WindowHandler, WindowHelper}};

use crate::{entity::{Entity, tile::{Tile, TileEnum, TileVariant, base_ground::BaseGround}}, ui::img::ImgManager, world::{generation, World}};

use super::{Screen, get_mouse_pos, get_resolution, title::TitleScreen};

bitflags! {
    struct Input: u8 {
        const NONE   = 0b00000000;
        const LEFT   = 0b00000001;
        const RIGHT  = 0b00000010;
        const UP     = 0b00000100;
        const DOWN   = 0b00001000;
        const ROTATE = 0b00010000;
        const TILE   = 0b00100000;
    }
}
impl From<VirtualKeyCode> for Input {
    fn from(key_code: VirtualKeyCode) -> Self {
        match key_code {
            VirtualKeyCode::W => Input::UP,
            VirtualKeyCode::A => Input::LEFT,
            VirtualKeyCode::S => Input::DOWN,
            VirtualKeyCode::D => Input::RIGHT,
            VirtualKeyCode::R => Input::ROTATE,
            VirtualKeyCode::T => Input::TILE,
            _ => Input::NONE,
        }
    }
}
impl Into<Option<VirtualKeyCode>> for Input {
    fn into(self) -> Option<VirtualKeyCode> {
        match self {
            Input::NONE => None,
            _ => Some(match self {
                Input::UP => VirtualKeyCode::W,
                Input::LEFT => VirtualKeyCode::A,
                Input::DOWN => VirtualKeyCode::S,
                Input::RIGHT => VirtualKeyCode::D,
                Input::ROTATE => VirtualKeyCode::R,
                Input::TILE => VirtualKeyCode::T,
                _ => panic!("Forgot to implement keycode mappings"), // never occurs
            }),
        }
    }
}

// Larger number -> smaller bounds
pub const CAMERA_SCALE: f32 = 50.0;

pub struct GameScreen {
    new_screen: Option<Box<dyn Screen>>,
    current_input: Input,
    world: World,
    img_manager: ImgManager,
    // For editing
    draw_tile: TileEnum,
    tile_variant: TileVariant,
}

impl WindowHandler<String> for GameScreen {
    fn on_draw(&mut self, helper: &mut WindowHelper<String>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::GRAY);

        self.world.player.update();

        self.world.tile_mgr.draw_before_player(
            graphics,
            &mut self.img_manager,
            &self.world.clock,
            &self.world.camera,
            self.world.player.get_pos(),
        );

        self.world.player.draw(
            graphics,
            &mut self.img_manager,
            &self.world.clock,
            &self.world.camera,
        );

        self.world.tile_mgr.draw_after_player(
            graphics,
            &mut self.img_manager,
            &self.world.clock,
            &self.world.camera,
            self.world.player.get_pos(),
        );
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
                            VirtualKeyCode::W => (0.0, -1.0),
                            VirtualKeyCode::A => (-1.0, 0.0),
                            VirtualKeyCode::S => (0.0, 1.0),
                            VirtualKeyCode::D => (1.0, 0.0),
                            VirtualKeyCode::R => {
                                self.tile_variant.rotate_cw();
                                return;
                            }
                            VirtualKeyCode::T => {
                                self.draw_tile.cycle();
                                return;
                            }
                            _ => (0.0, 0.0),
                        }
                        .into();
                        self.world.player.moove(move_pos);
                        if let Some((_, tile)) =
                            self.world.tile_mgr.tile_at_pos(self.world.player.get_pos())
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
    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<String>, button: MouseButton) {
        let pos = self.world.camera.pix_to_game(get_mouse_pos()).round();
        if let MouseButton::Left = button {
            let tile = self.draw_tile.create(pos, self.tile_variant);
            self.world.tile_mgr.push_override(tile);
        }
        else if let MouseButton::Right = button {
            self.world.tile_mgr.remove_at(pos);
        }
        else if let MouseButton::Middle = button {
            if let Some(tile) = self.world.tile_mgr.tile_at_pos(pos) {
                self.draw_tile = tile.1.get_tile_enum();
            }
        }
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
        GameScreen::with_world(generation::make_new_world())
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
            draw_tile: TileEnum::BaseGround,
            tile_variant: TileVariant::Top,
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
