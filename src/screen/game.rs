use std::{
    env,
    fs::{self, File},
    io::{self, BufReader},
    path::{Path, PathBuf}, collections::HashMap,
};

use bitflags::bitflags;

use speedy2d::{
    color::Color,
    window::{MouseButton, VirtualKeyCode, WindowHandler, WindowHelper},
    Graphics2D,
};

use crate::{
    screen::{title::TitleScreen, Screen},
    ui::img::ImgManager,
    world::{
        entity::Entity,
        generation,
        tile::{core::base_ground::BaseGround, Tile, TileVariant, PostOperation},
        World,
    },
};

// Larger number -> smaller bounds
pub const CAMERA_SCALE: f32 = 50.0;

enum State {
    Play,
    Dialogue,
    // For editing
    Build,
}

pub struct GameScreen {
    new_screen: Option<Box<dyn Screen>>,
    current_input: HashMap<VirtualKeyCode, bool>,
    world: World,
    img_manager: ImgManager,
    // For editing
    draw_tile: Box<dyn Tile>,
    tile_variant: TileVariant,
    state: State,
}

impl WindowHandler<String> for GameScreen {
    fn on_draw(&mut self, helper: &mut WindowHelper<String>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::GRAY);

        self.world.player.update();

        self.world.draw(graphics, &mut self.img_manager);
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
                VirtualKeyCode::R => {
                    self.tile_variant.rotate_cw();
                }
                VirtualKeyCode::T => {
                    self.draw_tile = self.draw_tile.cycle();
                }
                _ => {
                    if !self.current_input.get(&virtual_key_code).unwrap_or(&false) {
                        self.world.send_input_down(&virtual_key_code);
                    }
                }
            }
            self.current_input.insert(virtual_key_code, true);
        }
    }
    fn on_key_up(
        &mut self,
        _helper: &mut WindowHelper<String>,
        virtual_key_code: Option<VirtualKeyCode>,
        _scancode: speedy2d::window::KeyScancode,
    ) {
        if let Some(virtual_key_code) = virtual_key_code {
            self.world.send_input_up(&virtual_key_code);
            self.current_input.insert(virtual_key_code, false);
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
    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<String>, button: MouseButton) {
        let pos = self
            .world
            .camera
            .pix_to_game(super::get_mouse_pos())
            .round();
        if let MouseButton::Left = button {
            let tile = self.draw_tile.create(pos, self.tile_variant);
            self.world.tile_mgr.push_override(tile);
        } else if let MouseButton::Right = button {
            self.world.tile_mgr.remove_at(pos);
        } else if let MouseButton::Middle = button {
            if let Some((_, tile)) = self.world.tile_mgr.tile_at_pos(pos) {
                self.draw_tile = tile.pick_tile();
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
        self.on_resize(helper, super::get_resolution().into());
    }
}

impl GameScreen {
    pub fn new() -> GameScreen {
        GameScreen::with_world(generation::make_new_empty_world())
    }

    pub fn load() -> io::Result<GameScreen> {
        let mut result = GameScreen::load_world().unwrap();
        if let Some(minigame) = &mut result.minigame {
            minigame.reset();
        }
        Ok(GameScreen::with_world(result))
    }

    fn with_world(world: World) -> GameScreen {
        GameScreen {
            new_screen: None,
            current_input: HashMap::new(),
            world,
            img_manager: ImgManager::new(),
            draw_tile: Box::new(BaseGround::default((0, 0).into())),
            tile_variant: TileVariant::Top,
            state: State::Build,
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
