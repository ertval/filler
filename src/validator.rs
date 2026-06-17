use crate::types::{Cell, Grid, Piece, Player, Point};

// Audit Q10: Boundary Detection — pieces never placed partially outside grid
// Audit Q3: 1-cell overlap rule
// Audit Q9: Placement Validation — 0 overlap, 1 overlap, 2+ overlap, opponent overlap

/// Verify piece fits within grid boundary
pub fn is_in_bounds(grid: &Grid, piece: &Piece, target: Point) -> bool {
    for &(dr, dc) in &piece.blocks {
        let r = target.row + dr as i32;
        let c = target.col + dc as i32;
        if r < 0 || r >= grid.rows as i32 || c < 0 || c >= grid.cols as i32 {
            return false;
        }
    }
    true
}

/// Assert 1 own cell and 0 opponent cells overlap
pub fn is_valid_placement(grid: &Grid, piece: &Piece, target: Point, player: Player) -> bool {
    if !is_in_bounds(grid, piece, target) {
        return false;
    }

    let mut own_count = 0;
    let mut opp_count = 0;

    for &(dr, dc) in &piece.blocks {
        let r_i32 = target.row + dr as i32;
        let c_i32 = target.col + dc as i32;
        if r_i32 < 0 || r_i32 >= grid.rows as i32 || c_i32 < 0 || c_i32 >= grid.cols as i32 {
            return false;
        }
        let r = r_i32 as usize;
        let c = c_i32 as usize;
        let cell = grid.data[r][c];

        if cell.belongs_to(player) {
            own_count += 1;
        } else if cell != Cell::Empty {
            opp_count += 1;
        }
    }

    own_count == 1 && opp_count == 0
}

/// Scan grid (including negative padding)
pub fn find_valid_placements(grid: &Grid, piece: &Piece, player: Player) -> Vec<Point> {
    let mut results = Vec::new();
    // Start search from negative offsets so pieces with top-left padding can anchor off-grid
    let min_row = -(piece.rows as i32);
    let min_col = -(piece.cols as i32);

    for row in min_row..grid.rows as i32 {
        for col in min_col..grid.cols as i32 {
            let target = Point { row, col };
            if is_valid_placement(grid, piece, target, player) {
                results.push(target);
            }
        }
    }
    results
}
