use crate::types::{Cell, Grid, Piece, Player, Point};

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
        let r = (target.row + dr as i32) as usize;
        let c = (target.col + dc as i32) as usize;
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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_is_valid_placement() {
        let grid = build_test_grid();

        // Scenario 1: Zero own overlap (far away)
        let piece_1x1 = Piece {
            rows: 1,
            cols: 1,
            blocks: vec![(0, 0)],
        };
        assert!(!is_valid_placement(
            &grid,
            &piece_1x1,
            Point { row: 2, col: 2 },
            Player::P1
        ));

        // Scenario 2: Two own overlaps
        let mut grid_double = build_test_grid();
        grid_double.data[0][1] = Cell::Player1Old; // "@ @ . . ."
        let piece_1x2 = Piece {
            rows: 1,
            cols: 2,
            blocks: vec![(0, 0), (0, 1)],
        };
        assert!(!is_valid_placement(
            &grid_double,
            &piece_1x2,
            Point { row: 0, col: 0 },
            Player::P1
        ));

        // Scenario 3: One own + one opponent overlap
        // Piece 1x2 placed at (3,2) on P2's grid covers (3,2) and (3,3) which is P2
        let mut grid_p1_p2 = build_test_grid();
        grid_p1_p2.data[3][2] = Cell::Player1Old; // P1 at (3,2), P2 at (3,3)
        assert!(!is_valid_placement(
            &grid_p1_p2,
            &piece_1x2,
            Point { row: 3, col: 2 },
            Player::P1
        ));

        // Scenario 4: Exactly 1 own, 0 opponent
        assert!(is_valid_placement(
            &grid,
            &piece_1x2,
            Point { row: 0, col: 0 },
            Player::P1
        ));

        // Scenario 9: Zero own + zero opponent (all empty)
        assert!(!is_valid_placement(
            &grid,
            &piece_1x1,
            Point { row: 2, col: 2 },
            Player::P1
        ));
    }

    #[test]
    fn test_negative_offset_placement() {
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
}
