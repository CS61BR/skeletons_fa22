use crate::{drawing::animation::draw, drawing::images::IMAGES, game::Game};
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
pub fn console_log_str(s: &str) {
    println!("{}", s);
}

macro_rules! log {
    ($($t:tt)*) => ($crate::bindings::console_log_str(&format_args!($($t)*).to_string()))
}
pub(crate) use log; // make log macro public

#[wasm_bindgen]
extern "C" {
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn console_log_str(s: &str);

    pub fn register_image(path: &str, image_id: usize);

    #[wasm_bindgen(js_name = CanvasRenderingContext2D)]
    pub type RenderingContext;

    #[wasm_bindgen(method, setter)]
    pub fn set_font(this: &RenderingContext, font: &str);
    #[wasm_bindgen(method, js_name = fillRect)]
    pub fn fill_rect(this: &RenderingContext, x: f64, y: f64, width: f64, height: f64);
    #[wasm_bindgen(method, js_name = fillText)]
    pub fn fill_text(this: &RenderingContext, text: &str, x: f64, y: f64);

    // not really canvas methods, but we'll pretend they are
    fn set_fill_color(ctx: &RenderingContext, color: u32); //more efficient than using .fillStyle= with a string
    fn draw_image(ctx: &RenderingContext, image_id: usize, x: f64, y: f64, width: f64, height: f64);

    #[wasm_bindgen]
    pub type Storage;

    #[wasm_bindgen(method, js_name = getItem)]
    pub fn get_item(this: &Storage, key: &str) -> Option<String>;
    #[wasm_bindgen(method, js_name = setItem)]
    pub fn set_item(this: &Storage, key: &str, value: &str);
    #[wasm_bindgen(method, js_name = removeItem)]
    pub fn remove_item(this: &Storage, key: &str);
    #[wasm_bindgen(method)]
    pub fn clear(this: &Storage);
}

impl RenderingContext {
    pub fn set_fill_color(&self, color: u32) {
        set_fill_color(self, color)
    }

    pub fn draw_image(&self, image_id: usize, x: f64, y: f64, width: f64, height: f64) {
        draw_image(self, image_id, x, y, width, height)
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    // allows Rust to have actual error messages in webassembly
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub struct Binding {
    game: Game,
}

#[wasm_bindgen]
impl Binding {
    pub fn new(storage: Storage) -> Binding {
        log!("New Binding object created!");
        for (path, image_id) in IMAGES {
            register_image(path, image_id);
        }
        Binding {
            game: Game::new(storage),
        }
    }

    pub fn handle_keydown(&mut self, keycode: &str) {
        // TODO: you can also choose to handle arrow keys and stuff
        if keycode.len() == 4 && keycode.starts_with("Key")
            || keycode.len() == 6 && keycode.starts_with("Digit")
        {
            self.game
                .respond_to_keyboard_input(keycode.chars().last().unwrap().to_ascii_lowercase());
        } else if keycode == "Semicolon" {
            self.game.respond_to_keyboard_input(':');
        }
    }

    pub fn handle_keyup(&mut self, _keycode: &str) {
        // TODO: if holding keys down is important for your game, implement this function
    }

    pub fn handle_mousedown(&mut self, _x: f64, _y: f64) {
        // TODO: if mouse clicks are important for your game, implement this function
    }

    pub fn handle_mousemove(&mut self, _x: f64, _y: f64) {
        // TODO: if mouseovers are important for your game, implement this function
    }

    pub fn handle_mouseup(&mut self, _x: f64, _y: f64) {
        // TODO: if holding mouse down is important for your game, implement this function
    }

    pub fn draw_animation_frame(
        &mut self,
        ctx: &RenderingContext,
        window_width: f64,
        window_height: f64,
        timestamp: f64,
    ) {
        draw(&mut self.game, ctx, window_width, window_height, timestamp);
    }
}
