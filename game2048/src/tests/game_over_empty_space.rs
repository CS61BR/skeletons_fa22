use crate::game::game_over;

use super::utils::{board, board_from_list};

#[test]
#[rustfmt::skip::macros(board)]
fn test_completely_empty() {
    let board = board![
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    assert_eq!(
        game_over(&board),
        false,
        "Board is empty, has an empty space"
    );
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_empty_top_row() {
    let board = board![
        [0, 0, 0, 0],
        [2, 4, 2, 4],
        [4, 2, 4, 2],
        [2, 4, 2, 4]
    ];
    assert_eq!(game_over(&board), false, "Top row is empty");
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_empty_bottom_row() {
    let board = board![
        [2, 4, 2, 4],
        [4, 2, 4, 2],
        [2, 4, 2, 4],
        [0, 0, 0, 0]
    ];
    assert_eq!(game_over(&board), false, "Bottom row is empty");
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_empty_left_col() {
    let board = board![
        [0, 4, 2, 4],
        [0, 2, 4, 2],
        [0, 4, 2, 4],
        [0, 2, 4, 2]
    ];
    assert_eq!(game_over(&board), false, "Left column is empty");
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_empty_right_col() {
    let board = board![
        [2, 4, 2, 0],
        [4, 2, 4, 0],
        [2, 4, 2, 0],
        [4, 2, 4, 0]
    ];
    assert_eq!(game_over(&board), false, "Right column is empty");
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_almost_full_board() {
    let board = board![
        [2, 4, 2, 4],
        [4, 2, 4, 2],
        [2, 0, 2, 4],
        [4, 2, 4, 2]
    ];
    assert_eq!(game_over(&board), false, "Board has one empty space");
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_full_board() {
    let board = board![
        [2, 2, 2, 2],
        [2, 2, 2, 2],
        [2, 2, 2, 2],
        [2, 2, 2, 2]
    ];
    assert_eq!(
        game_over(&board),
        false,
        "Board has no empty spaces, but does have merges"
    );
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_full_board_no_merge() {
    let board = board![
        [2, 4, 2, 4],
        [4, 2, 4, 2],
        [2, 4, 2, 4],
        [4, 2, 4, 2]
    ];
    assert_eq!(
        game_over(&board),
        true,
        "Board has no empty spaces or merges"
    );
}
