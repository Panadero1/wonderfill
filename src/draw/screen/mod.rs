use std::sync::atomic::{AtomicU32, Ordering};

use speedy2d::{
    dimen::Vector2,
    window::{VirtualKeyCode, WindowHandler, WindowHelper},
    Graphics2D,
};

/// movable camera object
pub mod camera;

/// the game screen. Where the game takes place
pub mod game;

/// the options screen
pub mod options;

/// the title screen
pub mod title;

/// current position of the mouse. Use get_mmouse_pos
static MOUSE_POS: (AtomicU32, AtomicU32) = (AtomicU32::new(0), AtomicU32::new(0));

/// current resolution of the window. Use get_resolution and set_resolution
static RESOLUTION: (AtomicU32, AtomicU32) = (AtomicU32::new(400), AtomicU32::new(400));

/// minimum size the window can be shrunk to in any dimension
pub const MIN_WINDOW_SIZE: u32 = 400;

/// returns the current position of the mouse on the screen
pub fn get_mouse_pos() -> (u32, u32) {
    (
        MOUSE_POS.0.load(Ordering::Relaxed),
        MOUSE_POS.1.load(Ordering::Relaxed),
    )
}

/// returns the current resolution of the screen
pub fn get_resolution() -> (u32, u32) {
    (
        RESOLUTION.0.load(Ordering::Relaxed),
        RESOLUTION.1.load(Ordering::Relaxed),
    )
}

/// changes the resolution value of the screen.
pub fn set_resolution(new_width: u32, new_height: u32) {
    RESOLUTION.0.store(new_width, Ordering::Relaxed);
    RESOLUTION.1.store(new_height, Ordering::Relaxed);
}

/// WindowHandler plus helper functions for switching screens
pub trait Screen: WindowHandler<String> {
    /// switch to another screen
    fn change_screen(&mut self) -> Option<Box<dyn Screen>>;

    /// initialize the screen with `helper`
    fn init(&mut self, helper: &mut WindowHelper<String>);
}

/// WindowHandler implementation for redirecting events to a switchable screen
pub struct RedirectHandler {
    /// current screen to redirect events to
    cur_screen: Box<dyn Screen>,
}

impl WindowHandler<String> for RedirectHandler {
    fn on_start(
        &mut self,
        helper: &mut WindowHelper<String>,
        info: speedy2d::window::WindowStartupInfo,
    ) {
        // redirect
        self.cur_screen.on_start(helper, info);
    }

    fn on_user_event(&mut self, helper: &mut WindowHelper<String>, user_event: String) {
        // redirect
        self.cur_screen.on_user_event(helper, user_event);
    }

    fn on_resize(&mut self, helper: &mut WindowHelper<String>, size_pixels: Vector2<u32>) {
        set_resolution(size_pixels.x, size_pixels.y);

        // redirect
        self.cur_screen.on_resize(helper, size_pixels);
    }

    fn on_scale_factor_changed(&mut self, helper: &mut WindowHelper<String>, scale_factor: f64) {
        // redirect
        self.cur_screen
            .on_scale_factor_changed(helper, scale_factor);
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<String>, graphics: &mut Graphics2D) {
        {
            let size = get_resolution();
            self.ensure_min_size(helper, Vector2::new(size.0, size.1));
        }

        // switch screens when requested
        if let Some(mut new_screen) = self.cur_screen.change_screen() {
            new_screen.init(helper);
            self.cur_screen = new_screen;
        }

        // redirect
        self.cur_screen.on_draw(helper, graphics);

        // to keep the window drawing
        helper.request_redraw();
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<String>, position: Vector2<f32>) {
        // setting MOUSE_POS
        MOUSE_POS.0.store(position.x as u32, Ordering::Relaxed);
        MOUSE_POS.1.store(position.y as u32, Ordering::Relaxed);

        // redirect
        self.cur_screen.on_mouse_move(helper, position);
    }

    fn on_mouse_button_down(
        &mut self,
        helper: &mut WindowHelper<String>,
        button: speedy2d::window::MouseButton,
    ) {
        // redirect
        self.cur_screen.on_mouse_button_down(helper, button);
    }

    fn on_mouse_button_up(
        &mut self,
        helper: &mut WindowHelper<String>,
        button: speedy2d::window::MouseButton,
    ) {
        // redirect
        self.cur_screen.on_mouse_button_up(helper, button);
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<String>,
        virtual_key_code: Option<VirtualKeyCode>,
        scancode: speedy2d::window::KeyScancode,
    ) {
        // redirect
        self.cur_screen
            .on_key_down(helper, virtual_key_code, scancode);
    }

    fn on_key_up(
        &mut self,
        helper: &mut WindowHelper<String>,
        virtual_key_code: Option<VirtualKeyCode>,
        scancode: speedy2d::window::KeyScancode,
    ) {
        // redirect
        self.cur_screen
            .on_key_up(helper, virtual_key_code, scancode);
    }

    fn on_keyboard_char(&mut self, helper: &mut WindowHelper<String>, unicode_codepoint: char) {
        // redirect
        self.cur_screen.on_keyboard_char(helper, unicode_codepoint);
    }

    fn on_keyboard_modifiers_changed(
        &mut self,
        helper: &mut WindowHelper<String>,
        state: speedy2d::window::ModifiersState,
    ) {
        // redirect
        self.cur_screen.on_keyboard_modifiers_changed(helper, state);
    }
}

impl RedirectHandler {
    /// constructs a new RedirectHandler with the given screen
    pub fn new(cur_screen: Box<dyn Screen>) -> RedirectHandler {
        RedirectHandler { cur_screen }
    }

    /// resizes the window to maintain a minimum size by [`MIN_WINDOW_SIZE`]
    fn ensure_min_size(
        &mut self,
        helper: &mut WindowHelper<String>,
        mut size_pixels: Vector2<u32>,
    ) {
        let mut change_size = false;

        // ensure min x
        if size_pixels.x < MIN_WINDOW_SIZE {
            change_size = true;
            size_pixels.x = MIN_WINDOW_SIZE;
        }

        // ensure min y
        if size_pixels.y < MIN_WINDOW_SIZE {
            change_size = true;
            size_pixels.y = MIN_WINDOW_SIZE;
        }

        if change_size {
            helper.set_size_pixels(size_pixels);
        }
    }
}
