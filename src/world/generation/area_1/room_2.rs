use speedy2d::shape::Rectangle;

use crate::{entity::tile::{TileVariant, moon::Moon, sun::Sun, base_ground::BaseGround, base_pillar::BasePillar}, world::{generation, TileManager}};

pub fn add(t: &mut TileManager) {
    add_room(t);
    add_walkway(t);
    add_door(t);
}

fn add_walkway(t: &mut TileManager) {
    generation::generate_box(t, Rectangle::from_tuples((-1, -14), (2, -5)), |v, x, y| {
        let pos = (x, y).into();
        if v == TileVariant::Left || v == TileVariant::Right {
            return Some(Box::new(BasePillar::default(pos)));
        }
        return Some(Box::new(BaseGround::default(pos)));
    })
}

fn add_door(t: &mut TileManager) {
    t.push_override(Box::new(BaseGround::default((0, -7).into())));
}

fn add_room(t: &mut TileManager) {
    generation::generate_box(t, Rectangle::from_tuples((-7, -13), (8, -6)), |v, x, y| {
        let pos = (x, y).into();
        Some(if let TileVariant::Center = v {
            if (x + y) % 2 == 0 {
                Box::new(Moon::new(pos))
            }
            else {
                Box::new(Sun::new(pos))
            }
        }
        else {
            Box::new(BasePillar::default(pos))
        })
    });
}