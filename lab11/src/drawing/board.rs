use super::tiles::{Tile, NOTHING};

#[derive(Clone)]
/// A grid of tiles
pub struct Board {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![NOTHING; height]; width];
        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.tiles[x][y] = tile;
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[x][y]
    }
}

/// A Board that is ready to be drawn on the screen
pub struct DrawingBoard {
    board: Board,
    pub start_x: f64,
    pub start_y: f64,
    pub tile_size: f64,
}

impl DrawingBoard {
    pub fn width(&self) -> usize {
        self.board.width()
    }

    pub fn height(&self) -> usize {
        self.board.height()
    }

    pub fn render_width(&self) -> f64 {
        self.width() as f64 * self.tile_size
    }

    pub fn render_height(&self) -> f64 {
        self.height() as f64 * self.tile_size
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        self.board.get_tile(x, y)
    }
}

/// calculates and returns optimal DrawingBoard
///
/// (x, y): upper left corner
///
/// (width, height): maximum possible width and height
///
/// align: whether to round to whole numbers. Aligning will make the board appear
/// slightly smaller, but more crisp. Aligning will also make performance better.
pub fn from_board(
    board: Board,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    align: bool,
) -> Option<DrawingBoard> {
    if board.width() == 0 || board.height() == 0 {
        return None;
    }
    let bw = board.width() as f64;
    let bh = board.height() as f64;
    let mut tile_size = (width / bw).min(height / bh);
    if align {
        tile_size = tile_size.floor();
    }
    let mut start_x = x + (width - tile_size * bw) / 2.;
    let mut start_y = y + (height - tile_size * bh) / 2.;
    if align {
        start_x = start_x.floor();
        start_y = start_y.floor();
    }
    Some(DrawingBoard {
        board,
        start_x,
        start_y,
        tile_size,
    })
}
