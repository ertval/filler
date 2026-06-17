// Audit Q7: Integration tests — full pipeline parse→validate→choose→format
// Audit Q3: 1-cell overlap rule verified end-to-end (IT-1, IT-2)
// Audit Q9: Placement Validation in pipeline context
// Audit Q10: Boundary Detection wired into pipeline (IT-5)
mod common;

use filler::types::{Player, Point};
use filler::{output, parser, strategy, validator};

#[test]
fn test_it1_p1_single_valid_turn() {
    let input = concat!(
        "$$$ exec p1 : [robots/bender]\n",
        "Anfield 5 5:\n",
        "    01234\n",
        "000 .....\n",
        "001 .@...\n",
        "002 .....\n",
        "003 ...$.\n",
        "004 .....\n",
        "Piece 1 1:\n",
        "O\n"
    );
    let mut reader = common::mock_stdin(input);
    let turn = parser::parse_turn(&mut reader, None).unwrap();
    assert_eq!(turn.me, Player::P1);

    let valid = validator::find_valid_placements(&turn.grid, &turn.piece, turn.me);
    // (1,1) is the only place it can overlap exactly 1 own cell
    assert_eq!(valid, vec![Point { row: 1, col: 1 }]);

    let heatmap = strategy::generate_heatmap(&turn.grid, turn.opponent, turn.me);
    let chosen = strategy::choose_best_placement(&valid, &heatmap, &turn.piece).unwrap();
    assert_eq!(chosen, Point { row: 1, col: 1 });
    assert_eq!(output::format_move(chosen), "1 1\n");
}

#[test]
fn test_it2_p2_single_valid_turn() {
    let input = concat!(
        "$$$ exec p2 : [robots/bender]\n",
        "Anfield 5 5:\n",
        "    01234\n",
        "000 .....\n",
        "001 .@...\n",
        "002 .....\n",
        "003 ...$.\n",
        "004 .....\n",
        "Piece 1 1:\n",
        "O\n"
    );
    let mut reader = common::mock_stdin(input);
    let turn = parser::parse_turn(&mut reader, None).unwrap();
    assert_eq!(turn.me, Player::P2);

    let valid = validator::find_valid_placements(&turn.grid, &turn.piece, turn.me);
    assert_eq!(valid, vec![Point { row: 3, col: 3 }]);

    let heatmap = strategy::generate_heatmap(&turn.grid, turn.opponent, turn.me);
    let chosen = strategy::choose_best_placement(&valid, &heatmap, &turn.piece).unwrap();
    assert_eq!(chosen, Point { row: 3, col: 3 });
    assert_eq!(output::format_move(chosen), "3 3\n");
}

#[test]
fn test_it3_no_valid_placement_fallback() {
    // No own territory cell on the grid
    let input = concat!(
        "$$$ exec p1 : [robots/bender]\n",
        "Anfield 5 5:\n",
        "    01234\n",
        "000 .....\n",
        "001 .....\n",
        "002 .....\n",
        "003 ...$.\n",
        "004 .....\n",
        "Piece 1 1:\n",
        "O\n"
    );
    let mut reader = common::mock_stdin(input);
    let turn = parser::parse_turn(&mut reader, None).unwrap();
    let valid = validator::find_valid_placements(&turn.grid, &turn.piece, turn.me);
    assert!(valid.is_empty());
    assert_eq!(output::format_no_move(), "0 0\n");
}

#[test]
fn test_it4_multiple_valid_placements_closest() {
    // Own cells: (1,1) and (1,2)
    // Opponent cell: (3,3)
    // Piece 1x1
    let input = concat!(
        "$$$ exec p1 : [robots/bender]\n",
        "Anfield 5 5:\n",
        "    01234\n",
        "000 .....\n",
        "001 .@@..\n",
        "002 .....\n",
        "003 ...$.\n",
        "004 .....\n",
        "Piece 1 1:\n",
        "O\n"
    );
    let mut reader = common::mock_stdin(input);
    let turn = parser::parse_turn(&mut reader, None).unwrap();
    let valid = validator::find_valid_placements(&turn.grid, &turn.piece, turn.me);
    // Should have valid placements at (1,1) and (1,2)
    assert!(valid.contains(&Point { row: 1, col: 1 }));
    assert!(valid.contains(&Point { row: 1, col: 2 }));

    let heatmap = strategy::generate_heatmap(&turn.grid, turn.opponent, turn.me);
    let chosen = strategy::choose_best_placement(&valid, &heatmap, &turn.piece).unwrap();
    // (1,2) is closer to (3,3) than (1,1)
    assert_eq!(chosen, Point { row: 1, col: 2 });
}

#[test]
fn test_it5_boundary_rejection() {
    // Piece 2x2. Placed at bottom-right corner (4,4) would extend out of bounds.
    let input = concat!(
        "$$$ exec p1 : [robots/bender]\n",
        "Anfield 5 5:\n",
        "    01234\n",
        "000 .....\n",
        "001 .....\n",
        "002 .....\n",
        "003 .....\n",
        "004 ....@\n",
        "Piece 2 2:\n",
        "OO\n",
        "OO\n"
    );
    let mut reader = common::mock_stdin(input);
    let turn = parser::parse_turn(&mut reader, None).unwrap();
    let valid = validator::find_valid_placements(&turn.grid, &turn.piece, turn.me);
    // Cannot place 2x2 at (4,4) legally since it overflows bottom-right.
    assert!(!valid.contains(&Point { row: 4, col: 4 }));
}

#[test]
fn test_it6_two_consecutive_turns() {
    let input = concat!(
        "$$$ exec p1 : [robots/bender]\n",
        "Anfield 5 5:\n",
        "    01234\n",
        "000 .....\n",
        "001 .@...\n",
        "002 .....\n",
        "003 ...$.\n",
        "004 .....\n",
        "Piece 1 1:\n",
        "O\n",
        "Anfield 5 5:\n",
        "    01234\n",
        "000 .....\n",
        "001 .@...\n",
        "002 .....\n",
        "003 ...$.\n",
        "004 .....\n",
        "Piece 1 1:\n",
        "O\n"
    );
    let mut reader = common::mock_stdin(input);
    let turn1 = parser::parse_turn(&mut reader, None).unwrap();
    assert_eq!(turn1.me, Player::P1);

    let turn2 = parser::parse_turn(&mut reader, Some(turn1)).unwrap();
    assert_eq!(turn2.me, Player::P1);
}
