use std::{
    env,
    fs::{self, File},
    io::{self, BufReader},
    path::{Path, PathBuf},
};

use crate::{
    draw::{
        screen::{self, camera::Camera},
        ui::img::ImgManager,
    },
    world::{entity::player::Player, space::GamePos, tile::{Tile, TileManager}, time::Clock}, utility::key::match_wasd_directions,
};

use serde::{Deserialize, Serialize};
use speedy2d::{
    window::{MouseButton, VirtualKeyCode, WindowHelper},
    Graphics2D,
};

use self::{
    entity::{Entity, Enemy, Friend},
    minigame::{GameResult, Minigame},
    tile::{core::BaseGround, operation::*, TileVariant},
};

pub mod entity;
pub mod generation;
pub mod minigame;
pub mod space;
pub mod tile;
pub mod time;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub tile_mgr: TileManager,
    pub player: Player,
    pub camera: Camera,
    pub clock: Clock,
    pub minigame: Option<Box<dyn Minigame>>,
    pub enemies: Vec<Box<dyn Enemy>>,
    pub friends: Vec<Box<dyn Friend>>,
    // For editing
    draw_tile: Box<dyn Tile>,
    tile_variant: TileVariant,
    post_ops: Vec<PostOperation>,
    mouse_buttons: u8,
}

const MOUSE_LEFT: u8 = 0b10000000;
const MOUSE_RIGHT: u8 = 0b01000000;
const MOUSE_MID: u8 = 0b00100000;
const MOUSE_OTHER: u8 = 0b00000000;

impl World {
    pub fn new(tile_mgr: TileManager, player: Player, camera: Camera, clock: Clock) -> World {
        World {
            tile_mgr,
            player,
            camera,
            clock,
            minigame: None,
            enemies: Vec::new(),
            friends: Vec::new(),
            draw_tile: Box::new(BaseGround::default((0, 0).into())),
            tile_variant: TileVariant::Top,
            post_ops: Vec::new(),
            mouse_buttons: 0,
        }
    }

    pub fn update_overworld(&mut self) {
        // Player moved ✅
        // Tile checked ✅
        // Camera moves ⬇️
        // Player enter entity 😬
        // Entity turn 😬
        // Execute postops ⬇️
        // 
        // Tick clock ⬇️

        self.camera.pos = self.player.get_pos();

        while let Some(op) = self.post_ops.pop() {
            op.execute(self);
        }

        self.update_anims();
        self.clock.tick();
    }

    fn update_anims(&mut self) {
        self.player.update_anim();
        self.tile_mgr.update(&self.clock);
    }

    pub fn draw(&mut self, graphics: &mut Graphics2D, manager: &mut ImgManager) {
        match &mut self.minigame {
            Some(minigame) => match minigame.update() {
                GameResult::Processing => minigame.draw(graphics, manager, &self.camera),
                GameResult::Success => {
                    self.minigame = None;
                }
                GameResult::Failure => {
                    self.minigame = None;
                }
            },
            None => {
                self.player.update_anim();
                self.draw_world(graphics, manager);
            }
        }
    }

    fn create_tiles(&mut self) {
        let pos = self.camera.pix_to_game(screen::get_mouse_pos()).round();

        if self.mouse_buttons & MOUSE_LEFT > 0 {
            let tile = self.draw_tile.create(pos, self.tile_variant);
            self.tile_mgr.push_override(tile);
        } else if self.mouse_buttons & MOUSE_RIGHT > 0 {
            self.tile_mgr.remove_at(pos);
        }
    }

    fn draw_world(&mut self, graphics: &mut Graphics2D, manager: &mut ImgManager) {
        self.create_tiles();

        self.tile_mgr.draw_before_player(
            graphics,
            manager,
            &self.clock,
            &self.camera,
            self.player.get_pos(),
        );
        self.player
            .draw(graphics, manager, &self.clock, &self.camera);
        self.tile_mgr.draw_after_player(
            graphics,
            manager,
            &self.clock,
            &self.camera,
            self.player.get_pos(),
        );
    }

    pub fn load_region(&mut self, name: &String) -> io::Result<()> {
        self.save_region();

        let path = get_file_path(name);
        let file = File::open(path)?;
        let rdr = BufReader::new(file);

        self.tile_mgr = serde_json::from_reader(rdr).unwrap();

        Ok(())
    }

    pub fn new_region(&mut self, name: String) {
        self.save_region();

        self.tile_mgr = TileManager::new(name, vec![]);
    }

    pub fn save_region(&self) {
        let path = get_file_path(&self.tile_mgr.name);
        let file = fs::File::create(path).unwrap();
        let writer = io::LineWriter::new(file);

        serde_json::to_writer(writer, &self.tile_mgr).unwrap();
    }

    pub fn send_input_down(&mut self, key: &VirtualKeyCode) {
        match &mut self.minigame {
            Some(minigame) => {
                minigame.key_down(key);
            }
            None => match key {
                // Need to remove this (V) before release
                VirtualKeyCode::N | VirtualKeyCode::B | VirtualKeyCode::Q | VirtualKeyCode::R | VirtualKeyCode::T => self.handle_editor_controls(key),
                VirtualKeyCode::W | VirtualKeyCode::A | VirtualKeyCode::S | VirtualKeyCode::D => self.handle_movement_controls(key),
                _ => (),
            },
        }
    }

    fn handle_movement_controls(&mut self, key: &VirtualKeyCode) {

        let move_pos = match_wasd_directions(key);

        if let Some((_, tile)) = self.tile_mgr.tile_at_pos(self.player.get_pos() + move_pos) {
            if !tile.block_movement() {
                self.player.moove(move_pos);
            }
        }

        self.update_overworld();
    }

    fn handle_editor_controls(&mut self, key: &VirtualKeyCode) {
        match key {
            VirtualKeyCode::N => {
                println!("Please enter name of new region: ");

                let mut line = String::new();

                std::io::stdin().read_line(&mut line).unwrap();

                self.new_region(line.trim().to_string());
            }
            VirtualKeyCode::B => {
                let pos = self.camera.pix_to_game(screen::get_mouse_pos()).round();
                println!("({},{})", pos.x, pos.y);
            }
            VirtualKeyCode::Q => {
                let pos = self.camera.pix_to_game(screen::get_mouse_pos()).round();
                self.player.moove(pos - self.player.get_pos());
                self.camera.moove(self.player.get_pos() - self.camera.pos);
            }
            VirtualKeyCode::R => {
                self.tile_variant.rotate_cw();
            }
            VirtualKeyCode::T => {
                self.draw_tile = self.draw_tile.cycle();
            }
            _ => unreachable!(),
        }
    }

    pub fn send_input_up(&mut self, key: &VirtualKeyCode) {
        match &mut self.minigame {
            Some(minigame) => {
                minigame.key_up(key);
            }
            None => {
                // Put overworld key-up handling here if needed
            }
        }
    }

    pub fn on_mouse_button_down(&mut self, _helper: &mut WindowHelper<String>, button: MouseButton) {
        let pos = self.camera.pix_to_game(screen::get_mouse_pos()).round();

        // No line-dragging for this action. Keep it here
        if let MouseButton::Middle = button {
            if let Some((_, tile)) = self.tile_mgr.tile_at_pos(pos) {
                self.draw_tile = tile.pick_tile();
            }
        }

        self.mouse_buttons |= match button {
            MouseButton::Left => MOUSE_LEFT,
            MouseButton::Right => MOUSE_RIGHT,
            MouseButton::Middle => MOUSE_MID,
            MouseButton::Other(_) => MOUSE_OTHER,
        };
    }

    pub fn on_mouse_button_up(&mut self, _helper: &mut WindowHelper<String>, button: MouseButton) {
        self.mouse_buttons &= match button {
            MouseButton::Left => (!MOUSE_LEFT),
            MouseButton::Right => (!MOUSE_RIGHT),
            MouseButton::Middle => (!MOUSE_MID),
            MouseButton::Other(_) => (!MOUSE_OTHER),
        };
        // Mouse up handling if needed
    }
}

fn get_file_path(file_name: &String) -> PathBuf {
    assert!(*file_name != "save");
    let dir = env::current_dir().unwrap();
    let path = Path::new(&dir).join("saves/");
    if !path.exists() {
        fs::create_dir(&path).unwrap();
    }
    let file_name = format!("{}.json", file_name);
    path.join(file_name)
}
