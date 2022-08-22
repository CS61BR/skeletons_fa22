use crate::{
    bindings::{draw_rectangle, draw_text},
    game::{Board, MovingTile},
};

const MOVING_FRAMES: u8 = 5;
const COMBINE_START: u8 = 5;
const COMBINE_FRAMES: u8 = 10; // must be even (animation divides in half)
const SPAWN_START: u8 = 10;
const SPAWN_FRAMES: u8 = 10; // must be even (animation divides in half)
const GAMEOVER_START: u8 = 60;
const GAMEOVER_FRAMES: u8 = 40;

const BLOOM: f64 = 1.1; // max size of tile during animation
const TILE_WIDTH: f64 = 100.0;
const TILE_HEIGHT: f64 = 100.0;
const BORDER_THICKNESS: f64 = 10.0;

// draws the whole thing, returning whether it has more frames to draw
pub fn draw(
    board: &Board,
    moving_tiles: &Vec<MovingTile>,
    animation_progress: u8,
    game_over: bool,
) -> bool {
    let (canvas_width, canvas_height) = canvas_size(board);
    draw_rectangle(0.0, 0.0, canvas_width, canvas_height, "#bbada0");

    // fill board with empty tiles first
    for a in 0..board.width {
        for b in 0..board.height {
            draw_tile(a as f64, b as f64, 0, 1.0);
        }
    }

    // animate all moving tiles
    for t in moving_tiles {
        let progress = if animation_progress >= MOVING_FRAMES {
            1.0
        } else {
            animation_progress as f64 / MOVING_FRAMES as f64
        };
        let x = (t.start_x as f64) * (1.0 - progress) + (t.end_x as f64) * progress;
        let y = (t.start_y as f64) * (1.0 - progress) + (t.end_y as f64) * progress;
        draw_tile(x, y, t.value, 1.0);
    }

    // animate all combined tiles
    if animation_progress >= COMBINE_START {
        for (x, y) in get_tiles(board, moving_tiles, 2) {
            draw_tile(
                x as f64,
                y as f64,
                board.tiles[x][y],
                blooming_size(animation_progress, COMBINE_START, COMBINE_FRAMES),
            );
        }
    }

    // animate all new tiles
    if animation_progress >= SPAWN_START {
        for (x, y) in get_tiles(board, moving_tiles, 0) {
            draw_tile(
                x as f64,
                y as f64,
                board.tiles[x][y],
                blooming_size(animation_progress, SPAWN_START, SPAWN_FRAMES),
            );
        }
    }

    // animate game over screen
    if game_over && animation_progress >= GAMEOVER_START {
        let progress = (animation_progress - GAMEOVER_START) as f64 / GAMEOVER_FRAMES as f64;
        let alpha = progress * (2.0 - progress);
        let overlay_color = format!("rgba(238, 228, 218, {})", 0.73 * alpha);
        let text_color = format!("rgba(119, 110, 101, {})", alpha);
        draw_rectangle(0.0, 0.0, canvas_width, canvas_height, &overlay_color);
        draw_text(
            "Game over!",
            canvas_width / 2.0,
            canvas_height / 2.0,
            &text_color,
            "bold 60px sans-serif",
        );
    }

    animation_progress < SPAWN_FRAMES + SPAWN_START
        || (game_over && animation_progress < GAMEOVER_START + GAMEOVER_FRAMES)
}

pub fn canvas_size(board: &Board) -> (f64, f64) {
    (
        (board.width as f64) * (TILE_WIDTH + BORDER_THICKNESS) + BORDER_THICKNESS,
        (board.height as f64) * (TILE_HEIGHT + BORDER_THICKNESS) + BORDER_THICKNESS,
    )
}

fn get_tiles(
    board: &Board,
    moving_tiles: &Vec<MovingTile>,
    num_moved_to: u32,
) -> Vec<(usize, usize)> {
    let mut moved_to = vec![vec![0; board.height]; board.width];
    for m in moving_tiles {
        moved_to[m.end_x][m.end_y] += 1;
    }
    (0..board.width)
        .flat_map(|a| {
            (0..board.height)
                .filter(|&b| moved_to[a][b] == num_moved_to && board.tiles[a][b] != 0)
                .map(move |b| (a, b))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}

fn blooming_size(animation_progress: u8, start: u8, frames: u8) -> f64 {
    let animation_progress = animation_progress as f64;
    let start = start as f64;
    let hf = (frames as f64) / 2.0;

    if animation_progress < start + hf {
        BLOOM * (animation_progress - start) / hf
    } else if animation_progress < start + hf * 2.0 {
        1.0 + (BLOOM - 1.0) * (start + hf * 2.0 - animation_progress) / hf
    } else {
        1.0
    }
}

fn draw_tile(x_pos: f64, y_pos: f64, value: u32, size: f64) {
    let (textcolor, color, font_size) = match value {
        0 => ("#776e65", "#cdc1b4", 60.),
        2 => ("#776e65", "#eee4da", 60.),
        4 => ("#776e65", "#ede0c8", 60.),
        8 => ("#f9f6f2", "#f2b179", 60.),
        16 => ("#f9f6f2", "#f59563", 60.),
        32 => ("#f9f6f2", "#f67c5f", 60.),
        64 => ("#f9f6f2", "#f65e3b", 60.),
        128 => ("#f9f6f2", "#edcf72", 50.),
        256 => ("#f9f6f2", "#edcc61", 50.),
        512 => ("#f9f6f2", "#edc850", 50.),
        1024 => ("#f9f6f2", "#edc53f", 40.),
        2048 => ("#f9f6f2", "#edc22e", 40.),
        _ => ("#f9f6f2", "#000000", 40.), // nether region. All tiles are black
    };

    let cx = x_pos * (TILE_WIDTH + BORDER_THICKNESS) + BORDER_THICKNESS + 0.5 * TILE_WIDTH;
    let cy = y_pos * (TILE_HEIGHT + BORDER_THICKNESS) + BORDER_THICKNESS + 0.5 * TILE_HEIGHT;
    draw_rectangle(
        cx - 0.5 * TILE_WIDTH * size,
        cy - 0.5 * TILE_HEIGHT * size,
        TILE_WIDTH * size,
        TILE_HEIGHT * size,
        color,
    );
    if value != 0 {
        draw_text(
            &format!("{}", value),
            cx,
            cy,
            textcolor,
            &format!("bold {}px sans-serif", font_size * size),
        );
    }
}
