use filler::types::{Cell, Grid, Piece, Player, Point};
use filler::validator::{find_valid_placements, is_in_bounds, is_valid_placement};

// Helper to build a simple grid
fn build_test_grid() -> Grid {
    // 5x5 grid:
    // @ . . . .
    // . . . . .
    // . . . . .
    // . . . $ .
    // . . . . .
    let mut data = vec![vec![Cell::Empty; 5]; 5];
    data[0][0] = Cell::Player1Old;
    data[3][3] = Cell::Player2Old;
    Grid {
        rows: 5,
        cols: 5,
        data,
    }
}

#[test]
fn test_is_in_bounds() {
    let grid = build_test_grid();
    // 2x2 piece with all blocks filled
    let piece = Piece {
        rows: 2,
        cols: 2,
        blocks: vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    };

    assert!(is_in_bounds(&grid, &piece, Point { row: 0, col: 0 }));
    assert!(is_in_bounds(&grid, &piece, Point { row: 3, col: 3 }));
    assert!(!is_in_bounds(&grid, &piece, Point { row: 4, col: 3 }));
    assert!(!is_in_bounds(&grid, &piece, Point { row: 3, col: 4 }));
    assert!(!is_in_bounds(&grid, &piece, Point { row: -1, col: 0 }));
}

#[test]
fn test_is_valid_placement_basic() {
    let grid = build_test_grid();

    // Scenario: Exactly 1 own, 0 opponent
    let piece_1x2 = Piece {
        rows: 1,
        cols: 2,
        blocks: vec![(0, 0), (0, 1)],
    };
    assert!(is_valid_placement(
        &grid,
        &piece_1x2,
        Point { row: 0, col: 0 },
        Player::P1
    ));
}

#[test]
fn test_e4_zero_blocks_piece() {
    // Piece with zero blocks (all dots) has no valid placement
    let grid = build_test_grid();
    let piece = Piece {
        rows: 2,
        cols: 2,
        blocks: vec![],
    };
    let placements = find_valid_placements(&grid, &piece, Player::P1);
    assert!(placements.is_empty());
}

#[test]
fn test_e5_grid_entirely_filled() {
    // A grid filled entirely with opponent cells leaves no placement possible
    let piece = Piece {
        rows: 1,
        cols: 1,
        blocks: vec![(0, 0)],
    };
    let data_opp = vec![vec![Cell::Player2Old; 5]; 5];
    let grid_opp = Grid {
        rows: 5,
        cols: 5,
        data: data_opp,
    };
    let placements_opp = find_valid_placements(&grid_opp, &piece, Player::P1);
    assert!(placements_opp.is_empty());
}

#[test]
fn test_piece_larger_than_grid() {
    let grid = build_test_grid();
    // A piece with blocks at (0,0) and (9,9) cannot possibly fit in a 5x5 grid
    let piece = Piece {
        rows: 10,
        cols: 10,
        blocks: vec![(0, 0), (9, 9)],
    };
    let placements = find_valid_placements(&grid, &piece, Player::P1);
    assert!(placements.is_empty());
}

#[test]
fn test_e8_opponent_chars_rejected() {
    // Both 's' and '$' must be treated as opponent territory
    let mut grid = build_test_grid();
    grid.data[1][1] = Cell::Player2Recent; // 's'
    grid.data[1][2] = Cell::Player2Old; // '$'

    let piece_1x1 = Piece {
        rows: 1,
        cols: 1,
        blocks: vec![(0, 0)],
    };

    // P1 attempting to place on 's' or '$' must be rejected
    assert!(!is_valid_placement(
        &grid,
        &piece_1x1,
        Point { row: 1, col: 1 },
        Player::P1
    ));
    assert!(!is_valid_placement(
        &grid,
        &piece_1x1,
        Point { row: 1, col: 2 },
        Player::P1
    ));
}

#[test]
fn test_e9_own_chars_overlap() {
    // Both 'a' and '@' must count as own territory
    let mut grid = build_test_grid();
    grid.data[1][1] = Cell::Player1Recent; // 'a'
    grid.data[1][2] = Cell::Player1Old; // '@'

    let piece_1x1 = Piece {
        rows: 1,
        cols: 1,
        blocks: vec![(0, 0)],
    };

    // P1 placing on 'a' or '@' must be valid (overlap exactly 1 owned cell)
    assert!(is_valid_placement(
        &grid,
        &piece_1x1,
        Point { row: 1, col: 1 },
        Player::P1
    ));
    assert!(is_valid_placement(
        &grid,
        &piece_1x1,
        Point { row: 1, col: 2 },
        Player::P1
    ));
}

#[test]
fn test_negative_offset_placements() {
    let grid = build_test_grid();
    // Piece 2x2 with blocks at (1,1)
    let piece = Piece {
        rows: 2,
        cols: 2,
        blocks: vec![(1, 1)],
    };
    // Target at (-1, -1) offset.
    // Block (1,1) lands at (-1+1, -1+1) = (0,0) which is P1.
    // This is a valid placement since it overlaps P1 exactly once and has no opponent overlap.
    assert!(is_valid_placement(
        &grid,
        &piece,
        Point { row: -1, col: -1 },
        Player::P1
    ));

    let placements = find_valid_placements(&grid, &piece, Player::P1);
    assert!(placements.contains(&Point { row: -1, col: -1 }));
}
