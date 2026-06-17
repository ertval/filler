// Audit Q7: Territory growth test — verifies own count increases monotonically over 3 turns
use filler::{
    strategy,
    types::{Cell, Grid, Piece, Player},
    validator,
};

#[test]
fn test_territory_monotonically_increases() {
    // 1. Initialize 5x5 grid with 1 player cell at (0,0), opponent at (4,4)
    let mut data = vec![vec![Cell::Empty; 5]; 5];
    data[0][0] = Cell::Player1Old;
    data[4][4] = Cell::Player2Old;
    let mut grid = Grid {
        rows: 5,
        cols: 5,
        data,
    };

    let me = Player::P1;
    let opponent = Player::P2;

    // Piece 1x2 horizontal: [OO]
    let piece = Piece {
        rows: 1,
        cols: 2,
        blocks: vec![(0, 0), (0, 1)],
    };

    // Helper to count player cells
    let count_me = |g: &Grid| -> usize {
        g.data
            .iter()
            .flatten()
            .filter(|&&c| c.belongs_to(me))
            .count()
    };

    assert_eq!(count_me(&grid), 1);

    // Turn 1
    let valid1 = validator::find_valid_placements(&grid, &piece, me);
    let heatmap1 = strategy::generate_heatmap(&grid, opponent, me);
    let chosen1 = strategy::choose_best_placement(&valid1, &heatmap1, &piece).unwrap();

    // Apply chosen1 to grid
    for &(dr, dc) in &piece.blocks {
        let r = (chosen1.row + dr as i32) as usize;
        let c = (chosen1.col + dc as i32) as usize;
        grid.data[r][c] = Cell::Player1Old;
    }
    assert_eq!(count_me(&grid), 2);

    // Turn 2
    let valid2 = validator::find_valid_placements(&grid, &piece, me);
    let heatmap2 = strategy::generate_heatmap(&grid, opponent, me);
    let chosen2 = strategy::choose_best_placement(&valid2, &heatmap2, &piece).unwrap();

    // Apply chosen2 to grid
    for &(dr, dc) in &piece.blocks {
        let r = (chosen2.row + dr as i32) as usize;
        let c = (chosen2.col + dc as i32) as usize;
        grid.data[r][c] = Cell::Player1Old;
    }
    assert_eq!(count_me(&grid), 3);

    // Turn 3
    let valid3 = validator::find_valid_placements(&grid, &piece, me);
    let heatmap3 = strategy::generate_heatmap(&grid, opponent, me);
    let chosen3 = strategy::choose_best_placement(&valid3, &heatmap3, &piece).unwrap();

    // Apply chosen3 to grid
    for &(dr, dc) in &piece.blocks {
        let r = (chosen3.row + dr as i32) as usize;
        let c = (chosen3.col + dc as i32) as usize;
        grid.data[r][c] = Cell::Player1Old;
    }
    assert_eq!(count_me(&grid), 4);
}
