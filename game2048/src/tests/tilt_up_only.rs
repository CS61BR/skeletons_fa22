use super::utils::{board, board_from_list, test_tilt};
use crate::game::{Direction, MovingTile};

#[test]
#[rustfmt::skip::macros(board)]
fn test_up_no_merge() {
    let before = board![
        [0, 0, 4, 0],
        [0, 0, 0, 2],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 4, 2],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(2, 0, 2, 0, 4),
        MovingTile::new(3, 1, 3, 0, 2),
    ];
    test_tilt(&before, Direction::North, &after, moves, 0);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_up_basic_merge() {
    let before = board![
        [0, 0, 0, 0],
        [0, 0, 2, 0],
        [0, 0, 2, 0],
        [0, 0, 0, 0]
    ];
    let after = board![
        [0, 0, 4, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(2, 1, 2, 0, 2),
        MovingTile::new(2, 2, 2, 0, 2),
    ];
    test_tilt(&before, Direction::North, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_up_triple_merge() {
    let before = board![
        [0, 0, 2, 0],
        [0, 0, 0, 0],
        [0, 0, 2, 0],
        [0, 0, 2, 0]
    ];
    let after = board![
        [0, 0, 4, 0],
        [0, 0, 2, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(2, 0, 2, 0, 2),
        MovingTile::new(2, 2, 2, 0, 2),
        MovingTile::new(2, 3, 2, 1, 2),
    ];
    test_tilt(&before, Direction::North, &after, moves, 4);
}

#[test]
#[rustfmt::skip::macros(board)]
fn test_up_tricky_merge() {
    let before = board![
        [0, 0, 4, 0],
        [0, 0, 0, 0],
        [0, 0, 4, 0],
        [0, 0, 8, 0]
    ];
    let after = board![
        [0, 0, 8, 0],
        [0, 0, 8, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    let moves = vec![
        MovingTile::new(2, 0, 2, 0, 4),
        MovingTile::new(2, 2, 2, 0, 4),
        MovingTile::new(2, 3, 2, 1, 8),
    ];
    test_tilt(&before, Direction::North, &after, moves, 8);
}
