use speedy2d::shape::Rectangle;

use crate::{entity::{player::{self, Player}, tile::{Tile, arrow::Arrow, test_ground::TestGround, test_pillar::TestPillar}}, screen::{
        self,
        camera::{self, Camera},
        game::CAMERA_SCALE,
    }, world::{time::Clock, TileManager}};

use super::{World, space::{Direction, GamePos}};

/*

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


World::new(
    TileManager::new(tiles),
    Player::new(),
    Camera::new(
        (0.0, 0.0).into(),
        res.0 as f32 / CAMERA_SCALE,
        res.1 as f32 / CAMERA_SCALE,
    ),
    Clock::new(),
)

*/

pub fn make_new_world() -> World {
    let res = screen::get_resolution();
    let mut tile_mgr = TileManager::new(vec![]);
    let player = Player::new();
    let camera = Camera::new(
        (0.0, 0.0).into(),
        res.0 as f32 / CAMERA_SCALE,
        res.1 as f32 / CAMERA_SCALE,
    );

    generate_starting_world(&mut tile_mgr);

    World::new(tile_mgr, player, camera, Clock::new())
}

fn generate_starting_world(tile_mgr: &mut TileManager) {

    generate_square(tile_mgr, (-4, -4), 9, |edge, x, y| {
        let pos = (x, y).into();
        if edge {
            Box::new(TestPillar::new(pos))
        }
        else {
            Box::new(TestGround::new(pos))
        }
    });
    add_arrows(tile_mgr);
}

fn generate_square<F: Fn(bool, i32, i32) -> Box<dyn Tile>>(tile_mgr: &mut TileManager, top_left: (i32, i32), size: i32, gen: F) {
    generate_box(tile_mgr, Rectangle::from_tuples(top_left, (top_left.0 + size, top_left.1 + size)), gen);
}

fn generate_box<F: Fn(bool, i32, i32) -> Box<dyn Tile>> (
    tile_mgr: &mut TileManager,
    bounds: Rectangle<i32>,
    gen: F
) {
    let tl = bounds.top_left();
    let br = bounds.bottom_right();

    for x in tl.x..br.y {
        for y in tl.y..br.y {
            if x == tl.x || x == br.x - 1 || y == tl.y || y == br.y - 1 {
                tile_mgr.push_override(gen(true, x, y));
            }
            else {
                tile_mgr.push_override(gen(false, x, y));
            }
        }
    }
}

fn add_arrows(tile_mgr: &mut TileManager) {
    tile_mgr.push_override(Box::new(Arrow::new((0.0, 1.0).into(), Direction::Up)));
    tile_mgr.push_override(Box::new(Arrow::new((0.0, -1.0).into(), Direction::Down)));
    tile_mgr.push_override(Box::new(Arrow::new((1.0, 0.0).into(), Direction::Left)));
    tile_mgr.push_override(Box::new(Arrow::new((-1.0, 0.0).into(), Direction::Right)));
}
