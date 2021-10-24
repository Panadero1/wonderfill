use speedy2d::shape::Rectangle;

use crate::{entity::{player::{self, Player}, tile::{Tile, test_ground::TestGround, test_pillar::TestPillar}}, screen::{
        self,
        camera::{self, Camera},
        game::CAMERA_SCALE,
    }, world::{time::Clock, TileManager}};

use super::{space::GamePos, World};

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
    let mut tiles = vec![];
    let player = Player::new();
    let camera = Camera::new(
        (0.0, 0.0).into(),
        res.0 as f32 / CAMERA_SCALE,
        res.1 as f32 / CAMERA_SCALE,
    );

    generate_box(&mut tiles, Rectangle::from_tuples((-10, -10), (10, 10)), |edge, x, y| {
        let pos = (x as f32, y as f32).into();
        if edge {
            Box::new(TestPillar::new(pos))
        }
        else {
            Box::new(TestGround::new(pos))
        }
    });

    World::new(TileManager::new(tiles), player, camera, Clock::new())
}

fn generate_box<F: Fn(bool, i32, i32) -> Box<dyn Tile>> (
    tiles: &mut Vec<Box<dyn Tile>>,
    bounds: Rectangle<i32>,
    gen: F
) {
    let tl = bounds.top_left();
    let br = bounds.bottom_right();

    for x in tl.x..br.y {
        for y in tl.y..br.y {
            if x == tl.x || x == br.x - 1 || y == tl.y || y == br.y - 1 {
                tiles.push(gen(true, x, y));
            }
            else {
                tiles.push(gen(false, x, y));
            }
        }
    }
}
