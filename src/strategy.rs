use crate::types::{Grid, Piece, Point, Player, Cell};
use std::collections::VecDeque;

/// Generate heatmap (BFS from opponent territory)
pub fn generate_heatmap(grid: &Grid, opponent: Player, me: Player) -> Vec<Vec<i32>> {
    let rows = grid.rows;
    let cols = grid.cols;
    let mut heatmap = vec![vec![i32::MAX; cols]; rows];
    let mut queue = VecDeque::new();

    // Initialize BFS queue with all opponent cells
    for r in 0..rows {
        for c in 0..cols {
            if grid.data[r][c].belongs_to(opponent) {
                heatmap[r][c] = 0;
                queue.push_back((r, c));
            } else if grid.data[r][c].belongs_to(me) {
                heatmap[r][c] = i32::MIN; // own territory identifier
            }
        }
    }

    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((r, c)) = queue.pop_front() {
        let dist = heatmap[r][c] + 1;
        for &(dr, dc) in &directions {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                let nr = nr as usize;
                let nc = nc as usize;
                // Only traverse into cells that are unvisited/empty (i32::MAX)
                if heatmap[nr][nc] == i32::MAX {
                    heatmap[nr][nc] = dist;
                    queue.push_back((nr, nc));
                }
            }
        }
    }

    // Apply Strategy Tuning Enhancements to heatmap values directly
    for r in 0..rows {
        for c in 0..cols {
            let val = heatmap[r][c];
            if val > 0 && val != i32::MAX {
                let mut bonus = 0;

                // 1. Edge Weighting: Bonus for cells near grid edges
                let r_dist = std::cmp::min(r, rows - 1 - r);
                let c_dist = std::cmp::min(c, cols - 1 - c);
                let min_edge_dist = std::cmp::min(r_dist, c_dist);
                if min_edge_dist == 0 {
                    bonus += 5; // Strong preference for edges
                } else if min_edge_dist == 1 {
                    bonus += 2; // Moderate preference
                }

                // 2. Opponent Proximity Blocking:
                // Check if any of the 4 neighbors contains an opponent cell
                let mut near_opponent = false;
                for &(dr, dc) in &directions {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                        if grid.data[nr as usize][nc as usize].belongs_to(opponent) {
                            near_opponent = true;
                            break;
                        }
                    }
                }
                if near_opponent {
                    bonus += 5;
                }

                // Apply bonus to lower the score (lower is better)
                heatmap[r][c] = std::cmp::max(0, val - bonus);
            }
        }
    }

    heatmap
}

/// Score a placement (lower score is better/closer to opponent)
pub fn score_placement(
    heatmap: &[Vec<i32>],
    piece: &Piece,
    target: Point,
) -> i32 {
    let mut score = 0i32;
    for &(dr, dc) in &piece.blocks {
        let r = (target.row + dr as i32) as usize;
        let c = (target.col + dc as i32) as usize;
        let h = heatmap[r][c];
        if h == i32::MIN {
            // Skip scoring own territory overlap cell
            continue;
        } else if h == i32::MAX {
            score = score.saturating_add(1000); // penalty for unreachable
        } else {
            score = score.saturating_add(h);
        }
    }
    score
}

/// Choose best placement with deterministic tie-breaking
pub fn choose_best_placement(
    placements: &[Point],
    heatmap: &[Vec<i32>],
    piece: &Piece,
) -> Option<Point> {
    if placements.is_empty() {
        return None;
    }

    let mut best = placements[0];
    let mut best_score = score_placement(heatmap, piece, best);

    for &p in &placements[1..] {
        let score = score_placement(heatmap, piece, p);
        if score < best_score {
            best_score = score;
            best = p;
        } else if score == best_score {
            // Deterministic Tiebreak: Lower row first, then lower col
            if p.row < best.row || (p.row == best.row && p.col < best.col) {
                best = p;
            }
        }
    }

    Some(best)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heatmap_generation() {
        // 3x3 grid with P1 at center (1,1) (me) and P2 at (0,0) (opponent)
        let mut data = vec![vec![Cell::Empty; 3]; 3];
        data[0][0] = Cell::Player2Old; // opponent
        data[1][1] = Cell::Player1Old; // me
        let grid = Grid {
            rows: 3,
            cols: 3,
            data,
        };

        let heatmap = generate_heatmap(&grid, Player::P2, Player::P1);
        assert_eq!(heatmap[0][0], 0);
        assert_eq!(heatmap[1][1], i32::MIN);
        // (0,1) is neighbor to (0,0) opponent and on edge.
        // base dist = 1. edge bonus = 5. opponent blocking bonus = 5.
        // val - bonus = 1 - 10 = -9 capped at 0.
        assert_eq!(heatmap[0][1], 0);
    }

    #[test]
    fn test_tiebreak_by_row_then_col() {
        let placements = vec![
            Point { row: 2, col: 2 },
            Point { row: 1, col: 2 },
            Point { row: 1, col: 1 },
        ];
        // Heatmap with flat values so scores are equal
        let heatmap = vec![vec![5; 5]; 5];
        let piece = Piece {
            rows: 1,
            cols: 1,
            blocks: vec![(0, 0)],
        };
        let best = choose_best_placement(&placements, &heatmap, &piece).unwrap();
        // Should choose row 1, col 1
        assert_eq!(best, Point { row: 1, col: 1 });
    }
}
