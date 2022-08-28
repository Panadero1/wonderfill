use crate::{
    draw::{
        screen::{self, camera::Camera},
        ui::img::ImgManager,
    },
    utility::key::match_wasd_directions,
    world::{entity::player::Player, space::GamePos, tile::Tile, time::Clock},
};

use serde::{Deserialize, Serialize};
use speedy2d::{
    window::{MouseButton, VirtualKeyCode, WindowHelper},
    Graphics2D,
};

use self::{
    data::DataManager,
    entity::{utility::Button, Entity},
    minigame::{GameResult, Minigame},
    operation::PostOperation,
    tile::{core::BaseGround, TileVariant},
};

pub mod data;
pub mod entity;
pub mod generation;
pub mod minigame;
pub mod operation;
pub mod space;
pub mod tile;
pub mod time;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub mgr: DataManager,
    pub player: Player,
    pub camera: Camera,
    pub clock: Clock,
    pub minigame: Option<Box<dyn Minigame>>,
    // For editing
    draw_item: DrawItem,
    tile_variant: TileVariant,
    post_ops: Vec<PostOperation>,
    mouse_buttons: u8,
}

#[derive(Serialize, Deserialize)]
enum DrawItem {
    Tile(Box<dyn Tile>),
    Entity(Box<dyn Entity>),
}
impl DrawItem {
    fn default() -> DrawItem {
        DrawItem::Tile(Box::new(BaseGround::default((0, 0).into())))
    }
}

const MOUSE_LEFT: u8 = 0b10000000;
const MOUSE_RIGHT: u8 = 0b01000000;
const MOUSE_MID: u8 = 0b00100000;
/// No bitflag value so it doesn't affect anything
const MOUSE_OTHER: u8 = 0b00000000;

const VIEW_DIST: f32 = 40.0;

impl World {
    pub fn new() -> World {
        World {
            mgr: DataManager::new(String::from("start")),
            player: Player::new(),
            camera: Camera::new((0, 0).into(), 10.0, 10.0),
            clock: Clock::new(),
            minigame: None,
            draw_item: DrawItem::default(),
            tile_variant: TileVariant::Top,
            post_ops: Vec::new(),
            mouse_buttons: 0,
        }
    }

    pub fn update_overworld(&mut self) {
        // Player moved ✅
        // Tile checked ✅
        // Player enter entity ⬇️
        // Entity turn ⬇️
        // Execute postops ⬇️
        // Camera moves ⬇️
        // Update anims ⬇️
        // Tick clock ⬇️

        let player_pos = self.player.get_pos();

        // Player enter entity
        if let Some((_, entity)) = self.mgr.get_entity_at_pos(player_pos) {
            self.post_ops
                .push(entity.on_player_enter(self.player.get_last_move_pos()));
        }

        // Entity turn
        self.post_ops.extend(self.mgr.do_entity_turn());

        // Execute postops
        while let Some(op) = self.post_ops.pop() {
            op.execute(self);
        }

        // Camera moves
        self.camera.pos = player_pos;

        // Update anims & tick clock
        self.update_anims();
        self.clock.tick();
    }

    pub fn update_anims(&mut self) {
        self.player.update_anim(&self.clock);
        self.mgr.update_anims(&self.clock);
    }

    /// Every frame. Draws world to screen
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
                self.draw_world(graphics, manager);
            }
        }
    }

    fn create_tiles(&mut self) {
        let pos = self.camera.pix_to_game(screen::get_mouse_pos()).round();

        if self.mouse_buttons & MOUSE_LEFT > 0 {
            match &self.draw_item {
                DrawItem::Tile(tile) => {
                    let tile = tile.create(pos, self.tile_variant);
                    self.mgr.push_tile_override(tile);
                }
                DrawItem::Entity(entity) => {
                    let entity = entity.create(pos);
                    self.mgr.push_entity_override(entity);
                }
            }
        } else if self.mouse_buttons & MOUSE_RIGHT > 0 {
            self.mgr.remove_tile_at(pos);
        }
    }

    fn draw_world(&mut self, graphics: &mut Graphics2D, manager: &mut ImgManager) {
        self.create_tiles();

        self.mgr.draw_before_player(
            graphics,
            manager,
            &self.clock,
            &self.camera,
            self.player.get_pos(),
        );
        self.player
            .draw(graphics, manager, &self.clock, &self.camera);
        self.mgr.draw_after_player(
            graphics,
            manager,
            &self.clock,
            &self.camera,
            self.player.get_pos(),
        );
    }

    pub fn send_input_down(&mut self, key: &VirtualKeyCode) {
        match &mut self.minigame {
            Some(minigame) => {
                minigame.key_down(key);
            }
            None => match key {
                // Need to remove this (V) before release
                VirtualKeyCode::N | VirtualKeyCode::B | VirtualKeyCode::Q | VirtualKeyCode::R | VirtualKeyCode::T | VirtualKeyCode::Z => self.handle_editor_controls(key),
                VirtualKeyCode::W | VirtualKeyCode::A | VirtualKeyCode::S | VirtualKeyCode::D /*| VirtualKeyCode::H*/=> self.handle_movement_controls(key),
                _ => (),
            },
        }
    }

    /// Only update world on movement key press
    fn handle_movement_controls(&mut self, key: &VirtualKeyCode) {
        // if let VirtualKeyCode::H = key {
        //     self.player.cycle_hat();
        // }
        let move_pos = match_wasd_directions(key);

        if let Some((_, tile)) = self.mgr.get_tile_at_pos(self.player.get_pos() + move_pos) {
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

                self.mgr.new_region(line.trim().to_string());
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
                match &mut self.draw_item {
                    DrawItem::Tile(tile) => {
                        *tile = tile.cycle();
                    }
                    DrawItem::Entity(entity) => {
                        // todo add cycling
                        *entity = Box::new(Button::default());
                    }
                }
            }
            VirtualKeyCode::Z => {
                self.draw_item = match self.draw_item {
                    DrawItem::Entity(_) => DrawItem::Tile(Box::new(BaseGround::default((0, 0).into()))),
                    DrawItem::Tile(_) => DrawItem::Entity(Box::new(Button::default())),
                }
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

    pub fn on_mouse_button_down(
        &mut self,
        _helper: &mut WindowHelper<String>,
        button: MouseButton,
    ) {
        let pos = self.camera.pix_to_game(screen::get_mouse_pos()).round();

        // No line-dragging for this action. Keep it here
        if let MouseButton::Middle = button {
            if let Some((_, tile)) = self.mgr.get_tile_at_pos(pos) {
                self.draw_item = DrawItem::Tile(tile.pick_tile());
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
