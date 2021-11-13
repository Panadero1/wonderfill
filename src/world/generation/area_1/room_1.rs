use crate::world::{
    generation,
    tile::{
        core::{
            arrow::Arrow, base_ground::BaseGround, base_pillar::BasePillar, edge::Edge,
            stair::Stair,
        },
        TileVariant,
    },
    TileManager,
};

pub fn add(t: &mut TileManager) {
    add_room(t);
    add_stairs(t);
    add_pedestal(t);
    add_arrows(t);
}

fn add_arrows(t: &mut TileManager) {
    let dist = 2;
    t.push_override(Box::new(Arrow::new((0, dist).into(), TileVariant::Top)));
    t.push_override(Box::new(Arrow::new((0, -dist).into(), TileVariant::Bottom)));
    t.push_override(Box::new(Arrow::new((dist, 0).into(), TileVariant::Left)));
    t.push_override(Box::new(Arrow::new((-dist, 0).into(), TileVariant::Right)));
}

fn add_stairs(t: &mut TileManager) {
    generation::generate_square(t, (-3, -3), 7, |variant, x, y| {
        Some(Box::new(Stair::new((x, y).into(), variant)))
    })
}

fn add_pedestal(t: &mut TileManager) {
    generation::generate_square(t, (-2, -2), 5, |variant, x, y| {
        Some(Box::new(Edge::new((x, y).into(), variant)))
    })
}

fn add_room(t: &mut TileManager) {
    generation::generate_square(t, (-7, -7), 15, |v, x, y| {
        Some(if let TileVariant::Center = v {
            Box::new(BaseGround::default((x, y).into()))
        } else {
            Box::new(BasePillar::default((x, y).into()))
        })
    });
}
