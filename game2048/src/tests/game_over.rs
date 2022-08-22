use crate::game::game_over;

use super::utils::{board, board_from_list};

/*

Adapted from TestAtLeastOneMoveExists

*/

#[test]
#[rustfmt::skip::macros(board)]
fn test_empty_space() {
    let board = board![
        [0, 0, 4, 0],
        [0, 0, 0, 0],
        [0, 2, 0, 0],
        [0, 0, 0, 0]
    ];
    assert_eq!(
        game_over(&board),
        false,
        "There is an empty space on the board"
    );
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_any_dir() {
    let board = board![
        [2, 4, 2, 2],
        [4, 2, 4, 2],
        [2, 4, 2, 4],
        [4, 2, 4, 2]
    ];
    assert_eq!(
        game_over(&board),
        false,
        "A tilt in any direction will change the board"
    );
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_left_or_right() {
    let board = board![
        [2, 4, 2, 4],
        [4, 8, 4, 2],
        [2, 2, 2, 4],
        [4, 8, 4, 2]
    ];
    assert_eq!(
        game_over(&board),
        false,
        "A tilt left or right will change the board"
    );
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_up_or_down() {
    let board = board![
        [2, 4, 2, 4],
        [4, 8, 4, 2],
        [2, 16, 4, 8],
        [4, 8, 4, 2]
    ];
    assert_eq!(
        game_over(&board),
        false,
        "A tilt up or down will change the board"
    );
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_move_exists_big_piece() {
    let board = board![
        [2, 4, 2, 4],
        [4, 2, 4, 2],
        [2, 2, 2, 4],
        [4, 2, 4, 2048]
    ];
    assert_eq!(
        game_over(&board),
        false,
        "A tilt in any direction will change the board"
    );
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_no_move_exists_1() {
    let board = board![
        [2, 4, 2, 4],
        [4, 2, 4, 2],
        [2, 4, 2, 4],
        [4, 2, 4, 2]
    ];
    assert_eq!(game_over(&board), true, "No move exists");
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_no_move_exists_2() {
    let board = board![
        [2, 1024, 2, 4],
        [4, 2, 4, 2],
        [2, 8, 16, 4],
        [512, 2, 4, 2]
    ];
    assert_eq!(game_over(&board), true, "No move exists");
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_no_move_exists_3() {
    let board = board![
        [8, 4, 2, 32],
        [32, 2, 4, 2],
        [2, 8, 2, 4],
        [4, 64, 4, 64]
    ];
    assert_eq!(game_over(&board), true, "No move exists");
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_no_move_exists_4() {
    let board = board![
        [2, 4, 2, 32],
        [32, 2, 4, 2],
        [2, 128, 2, 4],
        [4, 2, 4, 2]
    ];
    assert_eq!(game_over(&board), true, "No move exists");
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_no_move_exists_5() {
    let board = board![
        [8, 16, 2, 32],
        [32, 2, 64, 2],
        [2, 256, 128, 256],
        [1024, 8, 4, 2]
    ];
    assert_eq!(game_over(&board), true, "No move exists");
}

/*

Adapted from TestModel

*/

#[test]
#[rustfmt::skip::macros(board)]
fn test_game_over_no_change_1() {
    let board = board![
        [2, 4, 2, 4],
        [4, 2, 4, 2],
        [2, 4, 8, 4],
        [4, 2, 4, 2]
    ];
    assert_eq!(game_over(&board), true);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_game_over_no_change_2() {
    let board = board![
        [128, 4, 2, 4],
        [4, 32, 4, 2],
        [8, 16, 2, 8],
        [4, 32, 4, 1024]
    ];
    assert_eq!(game_over(&board), true);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_game_not_over_1() {
    let board = board![
        [2, 4, 2, 2],
        [4, 2, 4, 2],
        [2, 4, 2, 4],
        [4, 2, 4, 2]
    ];
    assert_eq!(game_over(&board), false, "can tilt north");
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_game_not_over_2() {
    let board = board![
        [2, 4, 2, 4],
        [4, 2, 4, 2],
        [2, 4, 2, 4],
        [4, 2, 4, 0]
    ];
    assert_eq!(game_over(&board), false, "can tilt south");
}
