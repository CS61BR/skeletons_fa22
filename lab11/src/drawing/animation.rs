use crate::{bindings::RenderingContext, game::Game};

use super::{
    board::{from_board, DrawingBoard},
    colors::{Color, BLACK, DARKER_GRAY},
    tiles::{Tile, TileContents},
};

/// Draws a DrawingBoard on the screen
fn draw_board(ctx: &RenderingContext, db: &DrawingBoard, background: Color) {
    ctx.set_fill_color(background);
    ctx.fill_rect(
        db.start_x,
        db.start_y,
        db.render_width(),
        db.render_height(),
    );
    ctx.set_font(&format!("{}px ui-monospace", db.tile_size));

    for tx in 0..db.width() {
        for ty in 0..db.height() {
            let x = db.start_x + db.tile_size * tx as f64;
            let y = db.start_y + db.tile_size * ty as f64;
            draw_tile(db.get_tile(tx, ty), ctx, x, y, db.tile_size);
        }
    }
}

/// Finds the coordinates of the mouse in the DrawingBoard
fn mouse_pos(btd: &DrawingBoard, mouse_x: f64, mouse_y: f64) -> Option<(usize, usize)> {
    let pos_x = (mouse_x - btd.start_x) / btd.tile_size;
    let pos_y = (mouse_y - btd.start_y) / btd.tile_size;
    if pos_x >= 0.
        && pos_y >= 0.
        && (pos_x as usize) < btd.width()
        && (pos_y as usize) < btd.height()
    {
        Some((pos_x as usize, pos_y as usize))
    } else {
        None
    }
}

/// Draws a tile on the screen
fn draw_tile(tile: &Tile, ctx: &RenderingContext, x: f64, y: f64, size: f64) {
    if let Some(color) = tile.background_color {
        ctx.set_fill_color(color);
        ctx.fill_rect(x, y, size, size);
    }
    match tile.contents {
        TileContents::Text { text, text_color } => {
            ctx.set_fill_color(text_color);
            // text draws from middle instead of corner
            ctx.fill_text(text, x + size / 2., y + size / 2.);
        }
        TileContents::Image { image_id } => {
            ctx.draw_image(image_id, x, y, size, size);
        }
        TileContents::None => {}
    }
}

/* Draw the current state of the game */
/* This function is called 60 times per second, so try to be reasonably efficient */
pub fn draw(
    game: &mut Game,
    ctx: &RenderingContext,
    window_width: f64,
    window_height: f64,
    _timestamp: f64,
) {
    // This is some starter code that just draws a grid of tiles
    ctx.set_fill_color(DARKER_GRAY);
    ctx.fill_rect(0., 0., window_width, window_height);
    let board = game.render();
    let mdb = from_board(board, 0., 0., window_width, window_height, true);
    if let Some(db) = mdb {
        draw_board(ctx, &db, BLACK);
    }
}
