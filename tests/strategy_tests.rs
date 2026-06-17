use filler::strategy;
use filler::types::{Piece, Point};

#[test]
fn test_tiebreak_by_col_then_row() {
    let placements = vec![
        Point { row: 2, col: 3 },
        Point { row: 3, col: 2 },
    ];
    // Flat heatmap: all cells have equal value
    let heatmap = vec![vec![10; 10]; 10];
    let piece = Piece {
        rows: 1,
        cols: 1,
        blocks: vec![(0, 0)],
    };
    let best = strategy::choose_best_placement(&placements, &heatmap, &piece).unwrap();
    // Col-first: chooses row 3, col 2
    assert_eq!(best, Point { row: 3, col: 2 });
}
