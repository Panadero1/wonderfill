use serde::{Deserialize, Serialize};
use speedy2d::Graphics2D;

use crate::{
    draw::{
        animation::{Animation, AnimationSelectError},
        ui::img::{Img, ImgManager},
    },
    screen::camera::Camera,
    world::{space::GamePos, time::Clock},
};

use super::{
    tile::{operation::PostOperation, Tile},
    VIEW_DIST,
};

pub mod player;

#[typetag::serde(tag = "type")]
pub trait Entity {
    fn draw(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        clock: &Clock,
        camera: &Camera,
    );
    fn moove(&mut self, change_pos: GamePos);
    fn get_anim_mut(&mut self) -> &mut Animation;
    fn get_pos(&self) -> GamePos;
    fn update(&mut self);
    fn update_anim(&mut self, clock: &Clock);
    fn on_player_enter(&mut self, move_pos: GamePos) -> PostOperation {
        PostOperation::new_empty().with_block_player(move_pos)
    }
    fn do_turn(&mut self) -> PostOperation {
        PostOperation::new_empty()
    }
}

#[derive(Serialize, Deserialize)]
pub struct EntityManager {
    entities: Vec<Box<dyn Entity>>,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            entities: Vec::new(),
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

    fn draw_where<P: FnMut(&&mut Box<dyn Entity>) -> bool>(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        clock: &Clock,
        camera: &Camera,
        predicate: P,
    ) {
        let mut entities = self
            .entities
            .iter_mut()
            .filter(predicate)
            .collect::<Vec<_>>();

        entities.sort_by(|t1, t2| t1.get_pos().y.partial_cmp(&t2.get_pos().y).unwrap());

        for entity in entities {
            entity.draw(graphics, manager, clock, camera);
        }
    }

    pub fn update_anims(&mut self, clock: &Clock) {
        for entity in &mut self.entities {
            entity.update_anim(clock);
        }
    }

    pub fn do_entity_turn(&mut self) -> Vec<PostOperation> {
        let mut post_ops = Vec::new();
        for entity in &mut self.entities {
            post_ops.push(entity.do_turn());
        }
        return post_ops;
    }

    pub fn entity_at_pos(&mut self, pos: GamePos) -> Option<(usize, &mut Box<dyn Entity>)> {
        self.entities
            .iter_mut()
            .enumerate()
            .find(|(_, e)| e.get_pos() == pos)
    }
}
