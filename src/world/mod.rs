use std::{
    env,
    fs::{self, File},
    io::{self, BufReader},
    path::{Path, PathBuf},
};

use crate::{
    screen::{camera::Camera, get_mouse_pos},
    ui::img::ImgManager,
    world::{entity::player::Player, space::GamePos, tile::Tile, time::Clock},
};

use serde::{Deserialize, Serialize};
use speedy2d::{window::VirtualKeyCode, Graphics2D};

use self::{
    entity::Entity,
    minigame::{GameResult, Minigame},
    tile::PostOperation,
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
}

const VIEW_DIST: f32 = 40.0;

impl World {
    pub fn new(tile_mgr: TileManager, player: Player, camera: Camera, clock: Clock) -> World {
        World {
            tile_mgr,
            player,
            camera,
            clock,
            minigame: None,
        }
    }

    pub fn update_overworld(&mut self) {
        self.clock.tick();
        self.tile_mgr.update(&self.clock);
    }

    pub fn draw(&mut self, graphics: &mut Graphics2D, manager: &mut ImgManager) {
        if let Some(minigame) = &mut self.minigame {
            let result = minigame.update();
            if let GameResult::Processing = result {
                minigame.draw(graphics, manager, &self.camera);
            } else {
                // TODO: do something with result
                self.minigame = None;
            }
        } else {
            self.draw_world(graphics, manager);
        }
    }

    fn draw_world(&mut self, graphics: &mut Graphics2D, manager: &mut ImgManager) {
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

    pub fn process_operation(&mut self, op: PostOperation) {
        match op {
            PostOperation::MovePlayer(change_pos) => self.player.moove(change_pos),
            PostOperation::LoadRegion(name) => self.load_region(&name).unwrap(),
            PostOperation::UpdateTile(pos) => {
                if let Some((_, tile)) = self.tile_mgr.tile_at_pos(pos) {
                    tile.update_self();
                }
            }
            PostOperation::Minigame(game) => self.minigame = Some(game),
        }
    }

    pub fn send_input_down(&mut self, key: &VirtualKeyCode) {
        match &mut self.minigame {
            Some(minigame) => {
                minigame.key_down(key);
            }
            None => match key {
                VirtualKeyCode::N => {
                    println!("Please enter name of new region: ");

                    let mut line = String::new();

                    std::io::stdin().read_line(&mut line).unwrap();

                    self.new_region(line.trim().to_string());
                }
                VirtualKeyCode::B => {
                    let pos = self.camera.pix_to_game(get_mouse_pos()).round();
                    println!("({},{})", pos.x, pos.y);
                }
                VirtualKeyCode::Q => {
                    let pos = self.camera.pix_to_game(get_mouse_pos()).round();
                    self.player.moove(pos - self.player.get_pos());
                    self.camera.moove(self.player.get_pos() - self.camera.pos);
                }
                _ => {
                    let move_pos = match key {
                        VirtualKeyCode::W => (0.0, -1.0),
                        VirtualKeyCode::A => (-1.0, 0.0),
                        VirtualKeyCode::S => (0.0, 1.0),
                        VirtualKeyCode::D => (1.0, 0.0),
                        _ => (0.0, 0.0),
                    }
                    .into();
                    self.player.moove(move_pos);
                    let post_ops =
                        if let Some((_, tile)) = self.tile_mgr.tile_at_pos(self.player.get_pos()) {
                            tile.on_player_enter(&mut self.player, move_pos)
                        } else {
                            Vec::new()
                        };

                    for post_op in post_ops {
                        self.process_operation(post_op);
                    }

                    self.camera.moove(self.player.get_pos() - self.camera.pos);
                    self.update_overworld();
                }
            },
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TileManager {
    name: String,
    tiles: Vec<Box<dyn Tile>>,
}

impl TileManager {
    pub fn new(name: String, tiles: Vec<Box<dyn Tile>>) -> TileManager {
        TileManager { name, tiles }
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

    fn draw_where<P: FnMut(&&mut Box<dyn Tile>) -> bool>(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        clock: &Clock,
        camera: &Camera,
        predicate: P,
    ) {
        let mut tiles = self.tiles.iter_mut().filter(predicate).collect::<Vec<_>>();

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
        tile.get_anim_mut().select("base").unwrap();
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
        self.remove_where(|t| t.get_pos() == pos);
    }
}
