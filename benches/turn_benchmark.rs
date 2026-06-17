use filler::{
    strategy,
    types::{Cell, Grid, Piece, Player},
    validator,
};
use std::time::Instant;

fn main() {
    // 1. Construct simulated 100x100 grid with scattered Player 1 and Player 2 cells
    let rows = 100;
    let cols = 100;
    let mut data = vec![vec![Cell::Empty; cols]; rows];

    // Scatter player 1 and player 2 cells
    for r in 0..rows {
        for c in 0..cols {
            if (r + c) % 25 == 0 {
                data[r][c] = Cell::Player1Old;
            } else if (r + c) % 25 == 12 {
                data[r][c] = Cell::Player2Old;
            }
        }
    }

    let grid = Grid { rows, cols, data };

    // 2. Construct 20x20 piece with diagonal block pattern
    let mut blocks = Vec::new();
    for r in 0..20 {
        blocks.push((r, r));
    }
    let piece = Piece {
        rows: 20,
        cols: 20,
        blocks,
    };

    let me = Player::P1;
    let opponent = Player::P2;

    // 3. Measure time for full pipeline cycle
    let start = Instant::now();

    let valid = validator::find_valid_placements(&grid, &piece, me);
    let heatmap = strategy::generate_heatmap(&grid, opponent, me);
    let _chosen = strategy::choose_best_placement(&valid, &heatmap, &piece);

    let duration = start.elapsed();
    println!("Decision time: {:?}", duration);
    assert!(
        duration.as_millis() < 500,
        "Performance regression: decision took longer than 500ms"
    );
}
