// A small bug to document

// Bug may occur since these minigames serialize
// Especially when using NInstant, which is a "serializable" wrapper over Instant
// - Note: it does not serialize, but rather resets to NInstant::now() on deserialize
// - It only serializes to `true` in the JSON file
// This may be an oversight on my part: minigames reliant on time may break


// A "solution"

// But I can't be bothered to fix it
// Instead, don't make any minigames that break upon a timer reset midway :))
// Let's treat it as an intended feature to punish players that excessively savestate


// What this means specifically:

// If the player preses <ESC> during a minigame that involves some timer using NInstant,
// ...it will serialize in its current state except the timer
// The timer will be reset
// EG: player is halfway through survival-type minigame and chooses to exit
// - Whole world is serialized as normal including minigame
// - On loading, the player is where they left off, but the timer is reset
// >> More specifically, the NInstant is reset to NInstant::now()


// If I weren't lazy:

// Maybe find a crate that has a serializable replacement for Instant

use speedy2d::{Graphics2D, window::VirtualKeyCode};

use crate::{ui::img::ImgManager, screen::camera::Camera};

use super::{time::Clock, space::GamePos};

pub mod smiley_win;

pub enum GameResult {
    Success,
    Processing,
    Failure,
}

#[typetag::serde(tag = "type")]
pub trait Minigame {

    fn update(&mut self) -> GameResult;

    fn draw(
        &mut self,
        graphics: &mut Graphics2D,
        manager: &mut ImgManager,
        camera: &Camera,
    );

    fn key_down(&mut self, key: &VirtualKeyCode);

    fn key_up(&mut self, key: &VirtualKeyCode);
}
