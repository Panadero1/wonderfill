use std::{
    env,
    fs::{self, File},
    io::{self, BufReader},
    path::{Path, PathBuf}, collections::HashMap,
};

use speedy2d::{
    color::Color,
    window::{MouseButton, VirtualKeyCode, WindowHandler, WindowHelper},
    Graphics2D,
};

use crate::{
    screen::{title::TitleScreen, Screen},
    draw::ui::img::ImgManager,
    world::{
        entity::Entity,
        generation,
        World,
    },
};

/// Scale factor for the camera. Larger number -> smaller bounds
pub const CAMERA_SCALE: f32 = 50.0;

/// The screen that handles all drawing for the game
pub struct GameScreen {
    /// for switching to another screen
    new_screen: Option<Box<dyn Screen>>,

    /// map that maintains the state of all input keys
    current_input: HashMap<VirtualKeyCode, bool>,

    /// World struct for managing game logic
    world: World,

    /// for managing sprites
    img_manager: ImgManager,
}

impl WindowHandler<String> for GameScreen {
    fn on_draw(&mut self, _helper: &mut WindowHelper<String>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::GRAY);

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
                    // on escape, save game and return to title screen
                    self.save_world();
                    self.new_screen = Some(Box::new(TitleScreen::new()));
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
        self.world.on_mouse_button_down(helper, button);
    }

    fn on_mouse_button_up(
        &mut self,
        helper: &mut WindowHelper<String>,
        button: MouseButton
    ) {
        self.world.on_mouse_button_up(helper, button);
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
        let mut result = GameScreen::load_world()?;
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
        let path = Path::new(&dir).join("saves/");
        if !path.exists() {
            fs::create_dir(&path).unwrap();
        }
        let path = path.join("save.json");
        path
    }

    fn load_world() -> io::Result<World> {
        let path = GameScreen::get_file_path();
        let file: File = File::open(path)?;
        let rdr = BufReader::new(file);

        let result = Ok(serde_json::from_reader(rdr)?);

        result
    }
}
