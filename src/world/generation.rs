use speedy2d::shape::Rectangle;

use crate::{
    entity::{
        player::{self, Player},
        tile::{
            arrow::Arrow,
            stair::{Stair, StairDirection},
            test_ground::TestGround,
            test_pillar::TestPillar,
            Tile,
        },
    },
    screen::{
        self,
        camera::{self, Camera},
        game::CAMERA_SCALE,
    },
    world::{time::Clock, TileManager},
};

use super::{
    space::{Direction, GamePos},
    World,
};

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
    generate_square(tile_mgr, (-5, -5), 11, |edge, x, y| {
        let pos = (x, y).into();
        Some(if edge {
            Box::new(TestPillar::new(pos))
        } else {
            Box::new(TestGround::new(pos))
        })
    });
    add_arrows(tile_mgr);
    add_stairs(tile_mgr);
}

fn generate_square<F: Fn(bool, i32, i32) -> Option<Box<dyn Tile>>>(
    tile_mgr: &mut TileManager,
    top_left: (i32, i32),
    size: i32,
    gen: F,
) {
    generate_box(
        tile_mgr,
        Rectangle::from_tuples(top_left, (top_left.0 + size, top_left.1 + size)),
        gen,
    );
}

fn generate_box<F: Fn(bool, i32, i32) -> Option<Box<dyn Tile>>>(
    tile_mgr: &mut TileManager,
    bounds: Rectangle<i32>,
    gen: F,
) {
    let tl = bounds.top_left();
    let br = bounds.bottom_right();

    for x in tl.x..br.y {
        for y in tl.y..br.y {
            if let Some(tile) = gen(
                x == tl.x || x == br.x - 1 || y == tl.y || y == br.y - 1,
                x,
                y,
            ) {
                tile_mgr.push_override(tile);
            }
        }
    }
}

fn add_arrows(tile_mgr: &mut TileManager) {
    let dist = 2;
    tile_mgr.push_override(Box::new(Arrow::new((0, dist).into(), Direction::Up)));
    tile_mgr.push_override(Box::new(Arrow::new((0, -dist).into(), Direction::Down)));
    tile_mgr.push_override(Box::new(Arrow::new((dist, 0).into(), Direction::Left)));
    tile_mgr.push_override(Box::new(Arrow::new((-dist, 0).into(), Direction::Right)));
}

fn add_stairs(tile_mgr: &mut TileManager) {
    generate_square(tile_mgr, (-3, -3), 7, |edge, x, y| {
        if edge {
            Some(Box::new(Stair::new(
                (x, y).into(),
                match (x, y) {
                    (-3, -3) => StairDirection::CornerTL,
                    (-3, 3) => StairDirection::CornerBL,
                    (3, -3) => StairDirection::CornerTR,
                    (3, 3) => StairDirection::CornerBR,
                    (a, _) if a == 3 || a == -3 => StairDirection::Vertical,
                    (_, a) if a == 3 || a == -3 => StairDirection::Horizontal,
                    e => panic!("Stair direction animation logic broken: {:?}", e),
                },
            )))
        } else {
            None
        }
    })
}
