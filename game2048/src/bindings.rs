use crate::{
    animation::{canvas_size, draw},
    game::{add_tile, game_over, tilt, Board, Direction, MovingTile},
    random::Random,
};
use wasm_bindgen::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
pub fn console_log_str(s: &str) {
    println!("{}", s);
}

macro_rules! log {
    ($($t:tt)*) => ($crate::bindings::console_log_str(&format_args!($($t)*).to_string()))
}
pub(crate) use log; // make log macro public

#[wasm_bindgen(start)]
pub fn main() {
    // allows Rust to have actual error messages in webassembly
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
extern "C" {
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn console_log_str(s: &str);

    // fun fact: all javascript numbers are f64 under the hood (except bigint)
    // canvas can handle floating point coords so might as well use them
    fn set_canvas_size(width: f64, height: f64);
    fn request_animation_frame();
    pub fn draw_rectangle(x: f64, y: f64, width: f64, height: f64, color: &str);
    pub fn draw_text(text: &str, x: f64, y: f64, color: &str, font: &str);
    fn set_score_text(text: &str);
}

#[wasm_bindgen]
pub struct Game {
    board: Board,                  // current state of the board
    random: Random,                // used for generating new board tiles
    moving_tiles: Vec<MovingTile>, // for movement animation
    animation_progress: u8,
    game_over: bool,
    score: u32,
    high_score: u32,
}

#[wasm_bindgen]
impl Game {
    pub fn new(width: usize, height: usize, seed: &str) -> Game {
        log!("New Game created!");
        let mut random = Random::new(seed);
        let game = Game {
            board: Board::new(width, height, &mut random),
            random,
            moving_tiles: Vec::new(),
            animation_progress: 0,
            game_over: false,
            score: 0,
            high_score: 0,
        };
        let (canvas_width, canvas_height) = canvas_size(&game.board);
        set_canvas_size(canvas_width, canvas_height);
        request_animation_frame();
        game
    }

    pub fn reset(&mut self, width: usize, height: usize) {
        self.board = Board::new(width, height, &mut self.random);
        self.moving_tiles = Vec::new();
        self.animation_progress = 0;
        self.game_over = false;
        self.score = 0;
        let (canvas_width, canvas_height) = canvas_size(&self.board);
        set_canvas_size(canvas_width, canvas_height);
        request_animation_frame();
    }

    pub fn handle_keypress(&mut self, keycode: &str) {
        let dir: Direction = match keycode {
            "ArrowUp" | "KeyW" => Direction::North,
            "ArrowRight" | "KeyD" => Direction::East,
            "ArrowDown" | "KeyS" => Direction::South,
            "ArrowLeft" | "KeyA" => Direction::West,
            _ => {
                return;
            }
        };

        if self.game_over {
            return;
        }

        if let Some(changes) = tilt(&self.board, dir) {
            (self.board, self.moving_tiles, _) = changes;
            self.score += changes.2;
            if self.score > self.high_score {
                self.high_score = self.score;
            }

            self.game_over = game_over(&self.board);
            if !self.game_over {
                add_tile(&mut self.board, &mut self.random);
                self.game_over = game_over(&self.board);
            }

            self.animation_progress = 0;
            request_animation_frame();
        }
    }

    pub fn draw_animation_frame(&mut self) {
        set_score_text(&format!(
            "score: {} / high score: {}",
            self.score, self.high_score
        ));
        if draw(
            &self.board,
            &self.moving_tiles,
            self.animation_progress,
            self.game_over,
        ) {
            self.animation_progress += 1;
            request_animation_frame();
        }
    }
}
