use std::fmt::Debug;

use crate::{
    bindings::{console_log_str, log},
    random::Random,
};

const GENERATE_2_PROBABILITY: f64 = 0.9; // otherwise generates a 4

pub struct Board {
    pub tiles: Vec<Vec<u32>>, // indexed by [x][y], (0,0) at top left
    pub width: usize,
    pub height: usize,
}

struct RotatedBoard {
    tiles: Vec<Vec<u32>>, // indexed by [x][y], (0,0) at top left
    width: usize,
    height: usize,
    dir: Direction, // direction that is facing up
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct MovingTile {
    pub start_x: usize,
    pub start_y: usize,
    pub end_x: usize,
    pub end_y: usize,
    pub value: u32,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Board {
    pub fn new(width: usize, height: usize, random: &mut Random) -> Self {
        let mut board = Self {
            tiles: vec![vec![0; height]; width],
            width,
            height,
        };
        add_tile(&mut board, random);
        add_tile(&mut board, random);
        board
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tile_strs = (0..self.height)
            .map(|h| {
                (0..self.width)
                    .map(|w| self.tiles[w][h].to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>();

        writeln!(
            f,
            "Board(width:{}, height:{}){:#?}",
            self.width, self.height, tile_strs
        )
    }
}

impl MovingTile {
    pub fn new(start_x: usize, start_y: usize, end_x: usize, end_y: usize, value: u32) -> Self {
        MovingTile {
            start_x,
            start_y,
            end_x,
            end_y,
            value,
        }
    }
}

// given a location (x,y) on the rotated board,
// returns the original location of the tile on the unrotated board
fn unrotate(board: &Board, x: usize, y: usize, dir: Direction) -> (usize, usize) {
    match dir {
        Direction::North => (x, y),
        Direction::East => (board.width - 1 - y, x),
        Direction::South => (board.width - 1 - x, board.height - 1 - y),
        Direction::West => (y, board.height - 1 - x),
    }
}

// Rotates the board such that the given direction faces up.
// Tiles should "fall" upwards after this rotation
fn rotate_board(board: &Board, dir: Direction) -> RotatedBoard {
    let (new_width, new_height) = match dir {
        Direction::North | Direction::South => (board.width, board.height),
        Direction::East | Direction::West => (board.height, board.width),
    };
    let mut rot = RotatedBoard {
        tiles: vec![vec![0; new_height]; new_width],
        width: new_width,
        height: new_height,
        dir,
    };
    for x in 0..new_width {
        for y in 0..new_height {
            let (old_a, old_b) = unrotate(board, x, y, dir);
            rot.tiles[x][y] = board.tiles[old_a][old_b];
        }
    }
    rot
}

// Rotate the board back to its original position (north facing up)
fn unrotate_board(rotated: RotatedBoard) -> Board {
    let (old_width, old_height) = match rotated.dir {
        Direction::North | Direction::South => (rotated.width, rotated.height),
        Direction::East | Direction::West => (rotated.height, rotated.width),
    };
    let mut board = Board {
        tiles: vec![vec![0; old_height]; old_width],
        width: old_width,
        height: old_height,
    };
    for a in 0..rotated.width {
        for b in 0..rotated.height {
            let (old_a, old_b) = unrotate(&board, a, b, rotated.dir);
            board.tiles[old_a][old_b] = rotated.tiles[a][b];
        }
    }
    board
}

// Given moves constructed on a rotated board, rotate them to match
// the board's original position
fn unrotate_move(board: &Board, mt: &mut MovingTile, dir: Direction) {
    (mt.start_x, mt.start_y) = unrotate(board, mt.start_x, mt.start_y, dir);
    (mt.end_x, mt.end_y) = unrotate(board, mt.end_x, mt.end_y, dir);
}

// Given the current state of the board and the direction in which to tilt,
// return the new state of the board,
// a list of where all the current tiles have moved (even the ones that stay stationary),
// and how much to increment the score by
// If the board doesn't change, return None
pub fn tilt(board: &Board, dir: Direction) -> Option<(Board, Vec<MovingTile>, u32)> {
    log!("This is an example log message. Use log messages to help you debug!");

    unimplemented!(); // TODO: implement this function

    // HINT: rotate_board, unrotate_board, and unrotate_move should be useful here
}

// Add a random tile to the given board
// does nothing if there are no spaces on the board
pub fn add_tile(board: &mut Board, random: &mut Random) {
    let mut open_positions = 0;
    for x in 0..board.width {
        for y in 0..board.height {
            if board.tiles[x][y] == 0 {
                open_positions += 1;
            }
        }
    }

    if open_positions == 0 {
        return;
    }
    let new_value = if random.next_f64() > GENERATE_2_PROBABILITY {
        4
    } else {
        2
    };
    let mut idx = random.next_below(open_positions);
    for x in 0..board.width {
        for y in 0..board.height {
            if board.tiles[x][y] == 0 {
                if idx == 0 {
                    board.tiles[x][y] = new_value;
                    return;
                } else {
                    idx -= 1;
                }
            }
        }
    }
}

// returns whether the game is over
// the game is over if there are no possible moves left
pub fn game_over(board: &Board) -> bool {
    unimplemented!(); // TODO: implement this function
}
