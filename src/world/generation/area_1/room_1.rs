use crate::{
    entity::tile::{
        arrow::Arrow, edge::Edge, stair::Stair, test_ground::TestGround, test_pillar::TestPillar,
        TileVariant,
    },
    world::{generation, space::Direction, TileManager},
};

pub fn add(t: &mut TileManager) {
    add_room(t);
    add_stairs(t);
    add_pedestal(t);
    add_arrows(t);
}

fn add_arrows(t: &mut TileManager) {
    let dist = 2;
    t.push_override(Box::new(Arrow::new((0, dist).into(), Direction::Up)));
    t.push_override(Box::new(Arrow::new((0, -dist).into(), Direction::Down)));
    t.push_override(Box::new(Arrow::new((dist, 0).into(), Direction::Left)));
    t.push_override(Box::new(Arrow::new((-dist, 0).into(), Direction::Right)));
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
            Box::new(TestGround::new((x, y).into()))
        } else {
            Box::new(TestPillar::new((x, y).into()))
        })
    });
}
