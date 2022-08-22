use std::cmp::Ordering;

use crate::game::{tilt, Board, Direction, MovingTile};

macro_rules! board {
    [ $($row: expr),+ ] => {
        board_from_list(&[
            $(&($row),)*
        ])
    };
}

pub(crate) use board;

// transpose because tiles is indexed by [x][y]
pub fn board_from_list(list: &[&[u32]]) -> Board {
    let width = list[0].len();
    let height = list.len();

    let mut board = Board {
        tiles: vec![vec![0; height]; width],
        width,
        height,
    };

    for x in 0..width {
        for y in 0..height {
            board.tiles[x][y] = list[y][x];
        }
    }
    board
}

pub fn test_tilt(
    before: &Board,
    dir: Direction,
    after: &Board,
    mut moves: Vec<MovingTile>,
    score_increment: u32,
) {
    let mut res = tilt(before, dir).expect("Board should have changed");

    let expected_board = format!("{:?}", after);
    let result_board = format!("{:?}", res.0);
    if expected_board != result_board {
        panic!(
            "board doesn't match expected result. Expected {} but got {}",
            expected_board, result_board
        );
    }

    assert_eq!(res.2, score_increment);

    moves.sort();
    res.1.sort();
    match moves.cmp(&res.1) {
        Ordering::Equal => {}
        _ => panic!(
            "moves don't match expected result. Expected {:#?} but got {:#?}",
            moves, res.1
        ),
    }
}

pub fn test_tilt_no_changes(before: &Board, dir: Direction) {
    let res = tilt(before, dir);
    assert!(res.is_none());
}
