use std::collections::HashMap;

use speedy2d::{dimen::Vector2, shape::Rectangle};

use crate::{
    entity::{
        player::Player,
        tile::{
            arrow::Arrow, edge::Edge, stair::Stair, test_ground::TestGround,
            test_pillar::TestPillar, Tile, TileVariant,
        },
    },
    screen::{self, camera::Camera, game::CAMERA_SCALE},
    world::{time::Clock, TileManager},
};

use super::{space::Direction, World};

mod area_1;

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
    add_areas(tile_mgr);
}

fn generate_square<F: Fn(TileVariant, i32, i32) -> Option<Box<dyn Tile>>>(
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

fn generate_box<F: Fn(TileVariant, i32, i32) -> Option<Box<dyn Tile>>>(
    tile_mgr: &mut TileManager,
    bounds: Rectangle<i32>,
    gen: F,
) {
    let tl = bounds.top_left();
    let br = bounds.bottom_right();

    for x in tl.x..br.x {
        for y in tl.y..br.y {
            if let Some(tile) = gen(get_tile_variant(x, y, tl, br), x, y) {
                tile_mgr.push_override(tile);
            }
        }
    }
}

fn get_tile_variant(x: i32, y: i32, tl: &Vector2<i32>, br: &Vector2<i32>) -> TileVariant {
    match (x == br.x - 1, x == tl.x, y == br.y - 1, y == tl.y) {
        (true, _, true, _) => TileVariant::CornerBR,
        (true, _, _, true) => TileVariant::CornerTR,
        (_, true, true, _) => TileVariant::CornerBL,
        (_, true, _, true) => TileVariant::CornerTL,
        (true, _, _, _) => TileVariant::Right,
        (_, true, _, _) => TileVariant::Left,
        (_, _, true, _) => TileVariant::Bottom,
        (_, _, _, true) => TileVariant::Top,
        _ => TileVariant::Center,
    }
}

fn add_areas(tile_mgr: &mut TileManager) {
    area_1::add(tile_mgr);
}