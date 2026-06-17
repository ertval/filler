use filler::strategy;
use filler::types::{Piece, Point};

#[test]
fn test_tiebreak_by_row_then_col() {
    let placements = vec![
        Point { row: 3, col: 2 },
        Point { row: 2, col: 5 },
        Point { row: 2, col: 1 },
    ];
    // Flat heatmap: all cells have equal value
    let heatmap = vec![vec![10; 10]; 10];
    let piece = Piece {
        rows: 1,
        cols: 1,
        blocks: vec![(0, 0)],
    };
    // Expected: picks lower row (2), then lower col (1) -> Point { row: 2, col: 1 }
    let best = strategy::choose_best_placement(&placements, &heatmap, &piece).unwrap();
    assert_eq!(best, Point { row: 2, col: 1 });
}
