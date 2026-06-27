use crate::types::{Cell, Grid, Piece, Player, Point};
use std::collections::VecDeque;

fn apply_placement(grid: &Grid, piece: &Piece, target: &Point, me: Player) -> Grid {
    let mut g = grid.clone();
    let recent = match me {
        Player::P1 => Cell::Player1Recent,
        Player::P2 => Cell::Player2Recent,
    };
    for &(dr, dc) in &piece.blocks {
        let r = (target.row + dr as i32) as usize;
        let c = (target.col + dc as i32) as usize;
        g.data[r][c] = recent;
    }
    g
}

fn frontier_size(grid: &Grid, player: Player) -> i32 {
    let dirs: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut count = 0_i32;
    for r in 0..grid.rows {
        for c in 0..grid.cols {
            if grid.data[r][c] != Cell::Empty {
                continue;
            }
            for &(dr, dc) in &dirs {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr >= 0 && nr < grid.rows as isize && nc >= 0 && nc < grid.cols as isize {
                    if grid.data[nr as usize][nc as usize].belongs_to(player) {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }
    count
}

fn count_opponent_adjacent(grid: &Grid, piece: &Piece, target: &Point, opponent: Player) -> i32 {
    let dirs: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut count = 0_i32;
    for &(dr, dc) in &piece.blocks {
        let r = (target.row + dr as i32) as isize;
        let c = (target.col + dc as i32) as isize;
        for &(ddr, ddc) in &dirs {
            let nr = r + ddr;
            let nc = c + ddc;
            if nr >= 0 && nr < grid.rows as isize && nc >= 0 && nc < grid.cols as isize {
                if grid.data[nr as usize][nc as usize].belongs_to(opponent) {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

fn reachable_area(grid: &Grid, player: Player) -> i32 {
    let dirs: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut visited = vec![vec![false; grid.cols]; grid.rows];
    let mut queue = VecDeque::new();
    for r in 0..grid.rows {
        for c in 0..grid.cols {
            if grid.data[r][c].belongs_to(player) {
                visited[r][c] = true;
                queue.push_back((r, c));
            }
        }
    }
    while let Some((r, c)) = queue.pop_front() {
        for &(dr, dc) in &dirs {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr >= 0 && nr < grid.rows as isize && nc >= 0 && nc < grid.cols as isize {
                let nr = nr as usize;
                let nc = nc as usize;
                if !visited[nr][nc] && grid.data[nr][nc] == Cell::Empty {
                    visited[nr][nc] = true;
                    queue.push_back((nr, nc));
                }
            }
        }
    }
    visited.iter().flatten().filter(|&&v| v).count() as i32
}

pub fn generate_heatmap(grid: &Grid, opponent: Player, me: Player) -> Vec<Vec<i32>> {
    let rows = grid.rows;
    let cols = grid.cols;
    let mut heatmap = vec![vec![i32::MAX; cols]; rows];
    let mut queue = VecDeque::new();

    for (r, row) in heatmap.iter_mut().enumerate() {
        for (c, cell) in row.iter_mut().enumerate() {
            if grid.data[r][c].belongs_to(opponent) {
                *cell = 0;
                queue.push_back((r, c));
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
                if heatmap[nr][nc] == i32::MAX {
                    heatmap[nr][nc] = dist;
                    queue.push_back((nr, nc));
                }
            }
        }
    }

    for (r, row) in heatmap.iter_mut().enumerate() {
        for (c, cell) in row.iter_mut().enumerate() {
            let val = *cell;
            if grid.data[r][c].belongs_to(me) {
                if val == i32::MAX {
                    *cell = i32::MIN;
                } else {
                    *cell = -val;
                }
            } else if val > 0 && val != i32::MAX {
                let mut bonus = 0;

                let r_dist = std::cmp::min(r, rows - 1 - r);
                let c_dist = std::cmp::min(c, cols - 1 - c);
                let min_edge_dist = std::cmp::min(r_dist, c_dist);
                if min_edge_dist == 0 {
                    bonus += 5;
                } else if min_edge_dist == 1 {
                    bonus += 2;
                }

                for &(dr, dc) in &directions {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;
                    if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                        let nru = nr as usize;
                        let ncu = nc as usize;
                        if grid.data[nru][ncu].belongs_to(opponent) {
                            bonus += 4;
                        }
                        if grid.data[nru][ncu] == Cell::Empty {
                            bonus += 1;
                        }
                    }
                }

                *cell = std::cmp::max(0, val - bonus);
            }
        }
    }

    heatmap
}

pub fn score_placement(heatmap: &Vec<Vec<i32>>, piece: &Piece, target: Point) -> (i32, i32) {
    let rows = heatmap.len();
    let cols = if rows > 0 { heatmap[0].len() } else { 0 };
    let mut new_dist = 0_i32;
    let mut own_dist = 0_i32;
    for &(dr, dc) in &piece.blocks {
        let r = target.row + dr as i32;
        let c = target.col + dc as i32;
        if r < 0 || c < 0 || r >= rows as i32 || c >= cols as i32 {
            continue;
        }
        let r = r as usize;
        let c = c as usize;
        let h = heatmap[r][c];
        if h < 0 {
            if h != i32::MIN {
                own_dist = own_dist.saturating_add(-h);
            }
        } else if h == i32::MAX {
            new_dist = new_dist.saturating_add(1000);
        } else {
            new_dist = new_dist.saturating_add(h);
        }
    }
    (new_dist, own_dist)
}

pub fn choose_best_placement(
    placements: &[Point],
    heatmap: &Vec<Vec<i32>>,
    piece: &Piece,
    grid: &Grid,
    me: Player,
    opponent: Player,
) -> Option<Point> {
    if placements.is_empty() {
        return None;
    }

    let mut best = placements[0];
    let mut best_score = score_placement_full(heatmap, piece, best, grid, me, opponent);

    for &p in &placements[1..] {
        let score = score_placement_full(heatmap, piece, p, grid, me, opponent);
        if score < best_score {
            best_score = score;
            best = p;
        } else if score == best_score {
            if p.col < best.col || (p.col == best.col && p.row < best.row) {
                best = p;
            }
        }
    }

    Some(best)
}

fn score_placement_full(
    heatmap: &Vec<Vec<i32>>,
    piece: &Piece,
    target: Point,
    grid: &Grid,
    me: Player,
    opponent: Player,
) -> i64 {
    let (new_dist, own_dist) = score_placement(heatmap, piece, target);

    let after = apply_placement(grid, piece, &target, me);
    let opp_front = frontier_size(&after, opponent);
    let my_front = frontier_size(&after, me);
    let opp_adj = count_opponent_adjacent(grid, piece, &target, opponent);
    let opp_reach = reachable_area(&after, opponent);
    let my_reach = reachable_area(&after, me);

    (new_dist as i64) * 1_000_000
        + (own_dist as i64) * 1_000
        + (opp_front as i64) * 25
        - (my_front as i64) * 8
        - (opp_adj as i64) * 200
        + (opp_reach as i64) * 2
        - (my_reach as i64) * 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Cell;

    #[test]
    fn test_heatmap_generation() {
        let mut data = vec![vec![Cell::Empty; 3]; 3];
        data[0][0] = Cell::Player2Old;
        data[1][1] = Cell::Player1Old;
        let grid = Grid {
            rows: 3,
            cols: 3,
            data,
        };

        let heatmap = generate_heatmap(&grid, Player::P2, Player::P1);
        assert_eq!(heatmap[0][0], 0);
        assert_eq!(heatmap[1][1], -2);
    }

    #[test]
    fn test_tiebreak_by_col_then_row() {
        let placements = vec![Point { row: 2, col: 3 }, Point { row: 3, col: 2 }];
        let heatmap = vec![vec![5; 5]; 5];
        let piece = Piece {
            rows: 1,
            cols: 1,
            blocks: vec![(0, 0)],
        };
        let grid = Grid {
            rows: 5,
            cols: 5,
            data: vec![vec![Cell::Empty; 5]; 5],
        };
        let best = choose_best_placement(&placements, &heatmap, &piece, &grid, Player::P1, Player::P2).unwrap();
        assert_eq!(best, Point { row: 3, col: 2 });
    }
}
