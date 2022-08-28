use std::{
    cmp::Ordering,
    env,
    fs::{self, File},
    io::{self, BufReader},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use speedy2d::Graphics2D;

use crate::draw::{screen::camera::Camera, ui::img::ImgManager};

use super::{
    entity::Entity,
    space::GamePos,
    tile::Tile,
    time::Clock,
    VIEW_DIST, operation::PostOperation,
};

#[derive(Serialize, Deserialize)]
pub struct DataManager {
    entities: Vec<Box<dyn Entity>>,
    tiles: Vec<Box<dyn Tile>>,
    name: String,
}

enum TileOrEntity<'a> {
    Tile(&'a mut Box<dyn Tile>),
    Entity(&'a mut Box<dyn Entity>),
}
impl<'a> TileOrEntity<'a> {
    fn get_pos(&self) -> GamePos {
        match self {
            Self::Tile(tile) => tile.get_pos(),
            Self::Entity(entity) => entity.get_pos(),
        }
    }
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (TileOrEntity::Tile(_), TileOrEntity::Tile(_)) => Ordering::Equal,
            (TileOrEntity::Tile(_), TileOrEntity::Entity(_)) => Ordering::Less,
            (TileOrEntity::Entity(_), TileOrEntity::Tile(_)) => Ordering::Greater,
            (TileOrEntity::Entity(_), TileOrEntity::Entity(_)) => Ordering::Equal,
        }
    }
    fn from_tile(tile: &'a mut Box<dyn Tile>) -> Self {
        TileOrEntity::Tile(tile)
    }
    fn from_entity(entity: &'a mut Box<dyn Entity>) -> Self {
        TileOrEntity::Entity(entity)
    }
}

impl DataManager {
    pub fn new(name: String) -> DataManager {
        DataManager {
            entities: Vec::new(),
            tiles: Vec::new(),
            name,
        }
    }

    pub fn draw_before_player(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        clock: &Clock,
        camera: &Camera,
        player_pos: GamePos,
    ) {
        self.draw_where(graphics, manager, clock, camera, |te| {
            te.get_pos().y <= player_pos.y
                && te.get_pos().largest_component_difference(player_pos) < VIEW_DIST
        })
    }
    pub fn draw_after_player(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        clock: &Clock,
        camera: &Camera,
        player_pos: GamePos,
    ) {
        self.draw_where(graphics, manager, clock, camera, |te| {
            te.get_pos().y > player_pos.y
                && te.get_pos().largest_component_difference(player_pos) < VIEW_DIST
        })
    }

    /// Draws tiles and entities line-by-line by some predicate provided
    fn draw_where<P: Fn(&mut TileOrEntity) -> bool>(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        clock: &Clock,
        camera: &Camera,
        predicate: P,
    ) {
        let mut all_things = self
            .tiles
            .iter_mut()
            .filter_map(|t| {
                let mut t = TileOrEntity::from_tile(t);
                if predicate(&mut t) {
                    Some(t)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        all_things.extend(
            self.entities
                .iter_mut()
                .filter_map(|e| {
                    let mut e = TileOrEntity::from_entity(e);
                    if predicate(&mut e) {
                        Some(e)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>(),
        );

        all_things.sort_by(|t1, t2| t1.cmp(t2));

        all_things.sort_by(|t1, t2| t1.get_pos().y.partial_cmp(&t2.get_pos().y).unwrap());

        for te in all_things {
            match te {
                TileOrEntity::Tile(tile) => tile.draw(graphics, manager, clock, camera),
                TileOrEntity::Entity(entity) => entity.draw(graphics, manager, clock, camera),
            }
        }
    }

    pub fn update_anims(&mut self, clock: &Clock) {
        for tile in &mut self.tiles {
            tile.update_state(clock);
            tile.update_anim();
        }

        // Entities already have had a turn, so they don't need to update their state
        for entity in &mut self.entities {
            entity.update_anim(clock);
        }
    }

    // Tile stuff

    pub fn push_tile_override(&mut self, tile: Box<dyn Tile>) {
        if let Some((to_remove, _)) = self.get_tile_at_pos(tile.get_pos()) {
            self.tiles.remove(to_remove);
        }
        self.push_tile(tile);
    }

    /// !Warning! Possibility of overlapping tiles. Use push_tile_override unless you know what you're doing
    pub fn push_tile(&mut self, mut tile: Box<dyn Tile>) {
        tile.get_anim_mut().select("base").unwrap();
        self.tiles.push(tile);
    }

    pub fn get_tile_at_pos(&mut self, pos: GamePos) -> Option<(usize, &mut Box<dyn Tile>)> {
        self.tiles
            .iter_mut()
            .enumerate()
            .find(|(_, t)| t.get_pos() == pos)
    }

    pub fn remove_tile_where<P: Fn(&Box<dyn Tile>) -> bool>(&mut self, predicate: P) {
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

    pub fn remove_tile_at(&mut self, pos: GamePos) {
        self.remove_tile_where(|t| t.get_pos() == pos);
    }

    // Entity stuff

    pub fn push_entity_override(&mut self, entity: Box<dyn Entity>) {
        if let Some((to_remove, _)) = self.get_entity_at_pos(entity.get_pos()) {
            self.entities.remove(to_remove);
        }
        self.push_entity(entity);
    }

    /// !Warning! Possibility of overlapping entities. Use push_entity_override unless you know what you're doing
    pub fn push_entity(&mut self, mut entity: Box<dyn Entity>) {
        entity.get_anim_mut().select("base").unwrap();
        self.entities.push(entity);
    }

    pub fn get_entity_at_pos(&mut self, pos: GamePos) -> Option<(usize, &mut Box<dyn Entity>)> {
        self.entities
            .iter_mut()
            .enumerate()
            .find(|(_, e)| e.get_pos() == pos)
    }

    pub fn remove_entity_where<P: Fn(&Box<dyn Entity>) -> bool>(&mut self, predicate: P) {
        let mut remove_indices = vec![];
        for (i, entity) in self.entities.iter().enumerate() {
            if predicate(entity) {
                remove_indices.push(i);
            }
        }
        for i in remove_indices {
            self.entities.remove(i);
        }
    }

    pub fn remove_entity_at(&mut self, pos: GamePos) {
        self.remove_entity_where(|t| t.get_pos() == pos);
    }

    pub fn do_entity_turn(&mut self) -> Vec<PostOperation> {
        let mut post_ops = Vec::new();
        for entity in &mut self.entities {
            post_ops.push(entity.do_turn());
        }
        return post_ops;
    }

    // File stuff

    pub fn load_region(&mut self, name: &String) -> io::Result<()> {
        self.save_region();

        let path = get_file_path(name);
        let file = File::open(path)?;
        let rdr = BufReader::new(file);

        *self = serde_json::from_reader(rdr).unwrap();

        Ok(())
    }

    pub fn new_region(&mut self, name: String) {
        self.save_region();

        *self = DataManager::new(name);
    }

    pub fn save_region(&self) {
        let path = get_file_path(&self.name);
        let file = fs::File::create(path).unwrap();
        let writer = io::LineWriter::new(file);

        serde_json::to_writer(writer, &self).unwrap();
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
