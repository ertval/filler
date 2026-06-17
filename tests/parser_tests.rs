use filler::parser::{parse_anfield, parse_piece, parse_player_line, parse_turn};
use filler::types::{Cell, Player};
use std::io::Cursor;

#[test]
fn test_parse_player_line_valid() {
    assert_eq!(
        parse_player_line("$$$ exec p1 : [robots/bender]").unwrap(),
        Player::P1
    );
    assert_eq!(
        parse_player_line("$$$ exec p2 : [robots/terminator]").unwrap(),
        Player::P2
    );
}

#[test]
fn test_parse_player_line_malformed() {
    // Missing Exec prefix
    assert!(parse_player_line("$$$ p1 : [robots/bender]").is_err());
    // Missing colon/bracket spaces
    assert!(parse_player_line("$$$ exec p1:[robots/bender]").is_err());
    // Missing closing bracket
    assert!(parse_player_line("$$$ exec p1 : [robots/bender").is_err());
    // Invalid player ID
    assert!(parse_player_line("$$$ exec p3 : [robots/bender]").is_err());
    // Empty line
    assert!(parse_player_line("").is_err());
}

#[test]
fn test_parse_anfield_valid() {
    // Dynamic row prefix testing (e.g. "000 ", "1000 ")
    let input = "Anfield 3 2:\n    012\n000 .@.\n001 .$.\n";
    let mut reader = Cursor::new(input);
    let grid = parse_anfield(&mut reader).unwrap();
    assert_eq!(grid.cols, 3);
    assert_eq!(grid.rows, 2);
    assert_eq!(
        grid.data[0],
        vec![Cell::Empty, Cell::Player1Old, Cell::Empty]
    );
    assert_eq!(
        grid.data[1],
        vec![Cell::Empty, Cell::Player2Old, Cell::Empty]
    );
}

#[test]
fn test_parse_anfield_large_row_prefix() {
    // Row indices with dynamic spaces (e.g., "1000 ")
    let input = "Anfield 3 2:\n     012\n1000 .@.\n1001 .$.\n";
    let mut reader = Cursor::new(input);
    let grid = parse_anfield(&mut reader).unwrap();
    assert_eq!(grid.cols, 3);
    assert_eq!(grid.rows, 2);
}

#[test]
fn test_parse_anfield_invalid_dimensions() {
    // Anfield 0 0:
    let input = "Anfield 0 0:\n    \n";
    let mut reader = Cursor::new(input);
    assert!(parse_anfield(&mut reader).is_err());

    // Anfield negative/non-integer
    let input = "Anfield -5 5:\n";
    let mut reader = Cursor::new(input);
    assert!(parse_anfield(&mut reader).is_err());
}

#[test]
fn test_parse_anfield_invalid_char() {
    // Anfield contains 'X' which is not valid
    let input = "Anfield 3 1:\n    012\n000 .X.\n";
    let mut reader = Cursor::new(input);
    assert!(parse_anfield(&mut reader).is_err());
}

#[test]
fn test_parse_anfield_header_missing_cols() {
    let col_header_short = "Anfield 5 2:\n 01\n000 .....\n001 .....\n";
    let mut reader = Cursor::new(col_header_short);
    assert!(parse_anfield(&mut reader).is_err());
}

#[test]
fn test_parse_piece_valid() {
    let input = "Piece 2 3:\n.#\n#.\n..\n";
    let mut reader = Cursor::new(input);
    let piece = parse_piece(&mut reader).unwrap();
    assert_eq!(piece.cols, 2);
    assert_eq!(piece.rows, 3);
    assert_eq!(piece.blocks, vec![(0, 1), (1, 0)]);
}

#[test]
fn test_parse_piece_zero_dimensions() {
    let input = "Piece 0 2:\n";
    let mut reader = Cursor::new(input);
    assert!(parse_piece(&mut reader).is_err());
}

#[test]
fn test_blank_line_tolerance() {
    // Blank lines intermixed before and within sections
    let input = "\n\n$$$ exec p1 : [robots/bender]\n\nAnfield 3 2:\n\n    012\n000 .@.\n001 .$.\n\nPiece 1 1:\n\nO\n";
    let mut reader = Cursor::new(input);
    let state = parse_turn(&mut reader, None).unwrap();
    assert_eq!(state.me, Player::P1);
    assert_eq!(state.grid.cols, 3);
    assert_eq!(state.piece.cols, 1);
}

#[test]
fn test_parse_eof_mid_turn() {
    let input = "$$$ exec p1 : [robots/bender]\nAnfield 3 2:\n    012\n000 .@.\n";
    let mut reader = Cursor::new(input);
    let res = parse_turn(&mut reader, None);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("EOF"));
}
