use crate::{
    entity::{player::Player, tile::Tile},
    screen::camera::Camera,
    ui::img::ImgManager,
};


use self::{space::GamePos, time::Clock};
use serde::{Deserialize, Serialize};
use speedy2d::Graphics2D;

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
        World {
            tile_mgr,
            player,
            camera,
            clock,
        }
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
        clock: &Clock,
        camera: &Camera,
        player_pos: GamePos,
    ) {
        self.draw_where(graphics, manager, clock, camera, |t| {
            t.get_pos().y <= player_pos.y && (player_pos - t.get_pos()).magnitude() < VIEW_DIST
        });
    }
    pub fn draw_after_player(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        clock: &Clock,
        camera: &Camera,
        player_pos: GamePos,
    ) {
        self.draw_where(graphics, manager, clock, camera, |t| {
            t.get_pos().y > player_pos.y && (player_pos - t.get_pos()).magnitude() < VIEW_DIST
        });
    }

    fn draw_where<P: FnMut(&&mut Box<dyn Tile>) -> bool> (&mut self, graphics: &mut Graphics2D, manager: &mut ImgManager, clock: &Clock, camera: &Camera, predicate: P) {
        
        let mut tiles = self
            .tiles
            .iter_mut()
            .filter(predicate)
            .collect::<Vec<_>>();

        tiles.sort_by(|t1, t2| t1.get_pos().y.partial_cmp(&t2.get_pos().y).unwrap());

        for tile in tiles {
            tile.draw(graphics, manager, clock, camera);
        }
    }

    pub fn tile_at_pos(&mut self, pos: GamePos) -> Option<(usize, &mut Box<dyn Tile>)> {
        self.tiles
            .iter_mut()
            .enumerate()
            .find(|(_, t)| t.get_pos() == pos)
    }
    pub fn update(&mut self, clock: &Clock) {
        for t in &mut self.tiles {
            t.update_anim(clock);
            t.on_update(clock);
        }
    }
    pub fn push(&mut self, mut tile: Box<dyn Tile>) {
        tile.get_anim().select("base").unwrap();
        self.tiles.push(tile);
    }
    pub fn push_override(&mut self, tile: Box<dyn Tile>) {
        if let Some((to_remove, _)) = self.tile_at_pos(tile.get_pos()) {
            self.tiles.remove(to_remove);
        }
        self.push(tile);
    }
    pub fn remove_where<P: Fn(&Box<dyn Tile>) -> bool>(&mut self, predicate: P) {
        let mut remove_indices = vec![];
        for (i, tile) in self.tiles.iter().enumerate() {
            if predicate(tile) {
                remove_indices.push(i);
            }
        }
        for i in remove_indices {
            self.tiles.remove(i);
        }
    }
    pub fn remove_at(&mut self, pos: GamePos) {
        self.remove_where(|t| {t.get_pos() == pos});
    }
}
