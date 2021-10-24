use crate::{entity::{Entity, player::Player, tile::Tile}, screen::camera::Camera, ui::img::{Img, ImgManager}, utility::animation::{Animation, AnimationSelectError}};

use self::{space::GamePos, time::Clock};
use serde::{Deserialize, Serialize};
use speedy2d::{color::Color, Graphics2D};

pub mod generation;
pub mod space;
pub mod time;

#[derive(Serialize, Deserialize)]
pub struct World {
    pub tile_mgr: TileManager,
    pub player: Player,
    pub camera: Camera,
    pub clock: Clock,
}

const VIEW_DIST: f32 = 40.0;

impl World {
    pub fn new(tile_mgr: TileManager, player: Player, camera: Camera, clock: Clock) -> World {
        World { tile_mgr, player, camera, clock }
    }
    pub fn update(&mut self) {
        self.clock.tick();
        self.tile_mgr.update(&self.clock);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileManager {
    tiles: Vec<Box<dyn Tile>>,
}

impl TileManager {
    pub fn new(tiles: Vec<Box<dyn Tile>>) -> TileManager {
        TileManager { tiles }
    }

    pub fn draw_before_player(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        camera: &Camera,
        player_pos: GamePos,
    ) {
        for tile in self
            .tiles
            .iter_mut()
            .filter(|t| t.get_pos().y <= player_pos.y && (player_pos - t.get_pos()).magnitude() < VIEW_DIST)
        {
            tile.draw(graphics, manager, camera);
        }
    }
    pub fn draw_after_player(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        camera: &Camera,
        player_pos: GamePos,
    ) {
        for tile in self
            .tiles
            .iter_mut()
            .filter(|t| t.get_pos().y > player_pos.y && (player_pos - t.get_pos()).magnitude() < VIEW_DIST)
        {
            tile.draw(graphics, manager, camera);
        }
    }
    pub fn tile_at_pos(&mut self, pos: GamePos) -> Option<&mut Box<dyn Tile>> {
        self.tiles.iter_mut().find(|t| t.get_pos() == pos)
    }
    pub fn update(&mut self, clock: &Clock) {
        for t in &mut self.tiles {
            t.update_anim(clock);
        }
    }
}
