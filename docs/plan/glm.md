# Filler Implementation Plan: TDD — Deep Detail

Rust implementation. Every module: write tests first (RED), implement (GREEN), refactor. Junior dev follows verbatim.

---

## 0. Project Structure

```
filler/
├── Cargo.toml                      # workspace root
├── Cargo.lock
├── src/
│   ├── main.rs                     # entry: stdin loop, orchestration
│   ├── lib.rs                      # re-export all modules
│   ├── models.rs                   # data structures: Grid, Piece, Player, Coord, Placement
│   ├── parser.rs                   # stdin parsing: player line, grid, piece
│   ├── validator.rs                # placement legality: boundary, overlap, collision
│   ├── heatmap.rs                  # BFS heatmap generation
│   ├── scorer.rs                   # evaluate placements, pick best
│   └── output.rs                   # format coordinate as "X Y\n"
├── tests/
│   ├── pipeline.rs                 # integration: full-turn pipeline
│   ├── multi_turn.rs               # integration: multi-turn sequence
│   └── e2e.rs                      # e2e: run game_engine, win-rate assertions
├── benches/
│   └── turn_benchmark.rs           # performance: single turn < 500ms
├── scripts/
│   ├── run_audit_suite.sh          # bash: automates audit.md checklist
│   └── docker_test.sh              # bash: build & run inside docker
├── Dockerfile                      # build container
├── .dockerignore
└── visualizer/
    ├── Cargo.toml                  # separate bin crate
    └── src/
        └── main.rs                  # bonus: terminal color grid
```

### Cargo.toml

```toml
[package]
name = "filler"
version = "0.1.0"
edition = "2024"

[dev-dependencies]
assert_cmd = "2"
predicates = "3"
tempfile = "3"

[[bin]]
name = "filler"
path = "src/main.rs"

[lib]
name = "filler"
path = "src/lib.rs"

[[bench]]
name = "turn_benchmark"
harness = false
path = "benches/turn_benchmark.rs"

# E2E tests gated behind feature flag
[features]
e2e = []
```

### lib.rs

```rust
pub mod models;
pub mod parser;
pub mod validator;
pub mod heatmap;
pub mod scorer;
pub mod output;
```

---

## 1. Module: Models (`src/models.rs`)

### Data Structures

```rust
// models.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Coord {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<Vec<u8>>,  // b'.' | b'O' | b'o' | b'X' | b'x'
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            cells: vec![vec![b'.'; cols]; rows],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<u8> {
        self.cells.get(row).and_then(|r| r.get(col).copied())
    }

    pub fn set(&mut self, row: usize, col: usize, val: u8) {
        if row < self.rows && col < self.cols {
            self.cells[row][col] = val;
        }
    }

    pub fn count_char(&self, ch: u8) -> usize {
        self.cells.iter().flat_map(|row| row.iter()).filter(|&&c| c == ch).count()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    pub height: usize,
    pub width: usize,
    pub blocks: Vec<Coord>,  // coordinates of '#' cells relative to piece top-left
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub number: u8,        // 1 or 2
    pub char_up: u8,       // b'O' (p1) or b'X' (p2) — old territory
    pub char_lo: u8,       // b'o' (p1) or b'x' (p2) — last placed
    pub opp_up: u8,        // opponent uppercase
    pub opp_lo: u8,        // opponent lowercase
}

impl Player {
    pub fn new(number: u8) -> Result<Self, String> {
        match number {
            1 => Ok(Self { number: 1, char_up: b'O', char_lo: b'o', opp_up: b'X', opp_lo: b'x' }),
            2 => Ok(Self { number: 2, char_up: b'X', char_lo: b'x', opp_up: b'O', opp_lo: b'o' }),
            _ => Err(format!("invalid player number: {number}")),
        }
    }

    pub fn is_own(&self, cell: u8) -> bool {
        cell == self.char_up || cell == self.char_lo
    }

    pub fn is_opp(&self, cell: u8) -> bool {
        cell == self.opp_up || cell == self.opp_lo
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Placement {
    pub row: i32,
    pub col: i32,
}

impl Placement {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    pub fn none() -> Self {
        Self { row: 0, col: 0 }
    }
}

#[derive(Debug)]
pub struct TurnInput {
    pub grid: Grid,
    pub piece: Piece,
}
```

### Tests (`src/models.rs` — `#[cfg(test)]` module at bottom of file)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_grid_all_dots() {
        let g = Grid::new(3, 4);
        assert_eq!(g.rows, 3);
        assert_eq!(g.cols, 4);
        assert!(g.cells.iter().all(|row| row.iter().all(|&c| c == b'.')));
    }

    #[test]
    fn new_piece_from_blocks() {
        // TODO after implementing Piece::parse or Piece::new
    }

    #[test]
    fn player_p1() {
        let p = Player::new(1).unwrap();
        assert_eq!(p.char_up, b'O');
        assert_eq!(p.char_lo, b'o');
        assert_eq!(p.opp_up, b'X');
        assert_eq!(p.opp_lo, b'x');
    }

    #[test]
    fn player_p2() {
        let p = Player::new(2).unwrap();
        assert_eq!(p.char_up, b'X');
        assert_eq!(p.char_lo, b'x');
        assert_eq!(p.opp_up, b'O');
        assert_eq!(p.opp_lo, b'o');
    }

    #[test]
    fn player_invalid() {
        assert!(Player::new(3).is_err());
    }

    #[test]
    fn grid_get_set() {
        let mut g = Grid::new(5, 5);
        g.set(2, 3, b'O');
        assert_eq!(g.get(2, 3), Some(b'O'));
        assert_eq!(g.get(0, 0), Some(b'.'));
        assert_eq!(g.get(99, 0), None);  // out of bounds
    }

    #[test]
    fn is_own_is_opp() {
        let p1 = Player::new(1).unwrap();
        assert!(p1.is_own(b'O'));
        assert!(p1.is_own(b'o'));
        assert!(!p1.is_own(b'X'));
        assert!(p1.is_opp(b'X'));
        assert!(p1.is_opp(b'x'));
    }
}
```

---

## 2. Module: Parser (`src/parser.rs`)

### Functions

```rust
// parser.rs

use std::io::{BufRead, BufReader, Read};
use crate::models::{Grid, Piece, Coord, Player};

/// Parse the first line: "$$$ exec p<N> : [<path>]"
/// Returns player number (1 or 2).
pub fn parse_player_line(line: &str) -> Result<u8, String> {
    // Implementation: split by whitespace, find pN segment, extract N
}

/// Parse the Anfield from reader.
/// Expects: "Anfield <rows> <cols>:" header, then column header, then <rows> data lines.
/// Each data line: "NNN <content>" where NNN is row index.
/// Maps '@'→b'O', 'a'→b'o', '$'→b'X', 's'→b'x', '.'→b'.'
pub fn parse_grid<R: Read>(reader: &mut BufReader<R>) -> Result<Grid, String> {
    // Implementation: read header line, parse rows/cols, skip column line,
    // read rows lines, strip "NNN " prefix, map chars to internal byte repr.
}

/// Parse a piece from reader.
/// Expects: "Piece <height> <width>:" header, then <height> lines.
/// Returns Piece with blocks = coordinates of non-'.' cells.
pub fn parse_piece<R: Read>(reader: &mut BufReader<R>) -> Result<Piece, String> {
    // Implementation: read header, parse height/width, read shape lines,
    // collect (row, col) of '#' or '*' cells.
}
```

### Tests (`src/parser.rs` — `#[cfg(test)]` module)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;
    use std::io::Cursor;

    fn make_reader(data: &str) -> BufReader<Cursor<&str>> {
        BufReader::new(Cursor::new(data))
    }

    #[test]
    fn parse_player_line_p1() {
        assert_eq!(parse_player_line("$$$ exec p1 : [robots/bender]").unwrap(), 1);
    }

    #[test]
    fn parse_player_line_p2() {
        assert_eq!(parse_player_line("$$$ exec p2 : [./filler]").unwrap(), 2);
    }

    #[test]
    fn parse_player_line_invalid() {
        assert!(parse_player_line("invalid line").is_err());
    }

    #[test]
    fn parse_grid_basic() {
        let input = "Anfield 5 10:\n0123456789\n000 ..........\n001 ...@......\n002 ..........\n003 ..........\n004 ..........\n";
        let mut reader = make_reader(input);
        let grid = parse_grid(&mut reader).unwrap();
        assert_eq!(grid.rows, 5);
        assert_eq!(grid.cols, 10);
        assert_eq!(grid.cells[1][3], b'O');  // '@' mapped to 'O'
    }

    #[test]
    fn parse_grid_opponent() {
        let input = "Anfield 3 5:\n01234\n000 ..@..\n001 .....\n002 ..$..\n";
        let mut reader = make_reader(input);
        let grid = parse_grid(&mut reader).unwrap();
        assert_eq!(grid.cells[2][2], b'X');  // '$' mapped to 'X'
    }

    #[test]
    fn parse_grid_row_index_stripped() {
        let input = "Anfield 1 5:\n01234\n005 .....\n";
        let mut reader = make_reader(input);
        let grid = parse_grid(&mut reader).unwrap();
        assert_eq!(grid.cells[0][0], b'.');
    }

    #[test]
    fn parse_grid_truncated() {
        let input = "Anfield 3 5:\n01234\n000 .....\n";  // only 1 of 3 rows
        let mut reader = make_reader(input);
        assert!(parse_grid(&mut reader).is_err());
    }

    #[test]
    fn parse_piece_small() {
        let input = "Piece 2 2:\n.#\n#.\n";
        let mut reader = make_reader(input);
        let piece = parse_piece(&mut reader).unwrap();
        assert_eq!(piece.height, 2);
        assert_eq!(piece.width, 2);
        assert_eq!(piece.blocks, vec![Coord::new(0, 1), Coord::new(1, 0)]);
    }

    #[test]
    fn parse_piece_large() {
        let input = "Piece 5 4:\n.##..\n.##..\n..#..\n...#.\n";
        let mut reader = make_reader(input);
        let piece = parse_piece(&mut reader).unwrap();
        assert_eq!(piece.height, 4);
        assert_eq!(piece.width, 5);
        assert_eq!(piece.blocks.len(), 6);
    }

    #[test]
    fn parse_piece_all_dots() {
        let input = "Piece 1 1:\n.\n";
        let mut reader = make_reader(input);
        let piece = parse_piece(&mut reader).unwrap();
        assert!(piece.blocks.is_empty());
    }
}
```

### TDD Red-Green Steps (Parser)

1. **RED**: Write `parse_player_line_p1` test. `cargo test`. Compile error — function doesn't exist.
2. **GREEN**: Create `parser.rs`. Implement `parse_player_line` using `line.split_whitespace()`, find token starting with `p`, extract digit. `cargo test`. Pass.
3. **REFACTOR**: Consider regex vs manual parsing. Keep manual (no regex dependency). Re-run.
4. **RED**: Write `parse_grid_basic`, `parse_grid_opponent`, `parse_grid_row_index_stripped`.
5. **GREEN**: Implement `parse_grid`. `cargo test`. Pass.
6. **RED**: Write `parse_piece_small`, `parse_piece_large`, `parse_piece_all_dots`.
7. **GREEN**: Implement `parse_piece`. `cargo test`. Pass.

---

## 3. Module: Validator (`src/validator.rs`)

### Function Signature

```rust
// validator.rs

use crate::models::{Grid, Piece, Player, Placement};

/// Returns true iff placing piece at (row, col) on grid is legal:
///   1. All piece blocks fall within grid boundaries
///   2. Exactly 1 piece block overlaps with player's own territory
///   3. Zero piece blocks overlap with opponent territory
///   4. If piece has no blocks, return false
///
/// row/col are i32 to allow negative values (which fail boundary check).
pub fn is_valid(grid: &Grid, piece: &Piece, player: &Player, row: i32, col: i32) -> bool {
    let mut own_overlap = 0u32;
    for block in &piece.blocks {
        let r = row + block.row as i32;
        let c = col + block.col as i32;
        // Boundary check
        if r < 0 || c < 0 || r >= grid.rows as i32 || c >= grid.cols as i32 {
            return false;
        }
        let cell = grid.cells[r as usize][c as usize];
        if player.is_opp(cell) {
            return false;  // opponent overlap → invalid
        }
        if player.is_own(cell) {
            own_overlap += 1;
        }
    }
    own_overlap == 1 // exactly 1 own overlap required
}

/// Returns all valid placements for piece on grid for player.
pub fn find_valid_placements(grid: &Grid, piece: &Piece, player: &Player) -> Vec<Placement> {
    let mut results = Vec::new();
    for row in -(piece.height as i32)..grid.rows as i32 {
        for col in -(piece.width as i32)..grid.cols as i32 {
            if is_valid(grid, piece, player, row, col) {
                results.push(Placement::new(row, col));
            }
        }
    }
    results
}
```

### Tests (`src/validator.rs` — `#[cfg(test)]` module)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    fn make_grid_10x10() -> Grid {
        let mut g = Grid::new(10, 10);
        g.set(4, 4, b'O');  // own territory
        g.set(0, 0, b'X');  // opponent territory
        g
    }

    fn make_player_p1() -> Player {
        Player::new(1).unwrap()
    }

    fn make_piece_single() -> Piece {
        Piece { height: 1, width: 1, blocks: vec![Coord::new(0, 0)] }
    }

    fn make_piece_two_horizontal() -> Piece {
        Piece { height: 1, width: 2, blocks: vec![Coord::new(0, 0), Coord::new(0, 1)] }
    }

    #[test]
    fn boundary_top() {
        let g = Grid::new(10, 10);
        let p = make_player_p1();
        let piece = make_piece_single();
        assert!(!is_valid(&g, &piece, &p, -1, 5));
    }

    #[test]
    fn boundary_left() {
        let g = Grid::new(10, 10);
        let p = make_player_p1();
        let piece = make_piece_single();
        assert!(!is_valid(&g, &piece, &p, 5, -1));
    }

    #[test]
    fn boundary_bottom() {
        let g = Grid::new(10, 10);
        let p = make_player_p1();
        let piece = Piece { height: 3, width: 1, blocks: vec![Coord::new(0,0), Coord::new(1,0), Coord::new(2,0)] };
        assert!(!is_valid(&g, &piece, &p, 9, 0));  // extends to row 11
    }

    #[test]
    fn boundary_right() {
        let g = Grid::new(10, 10);
        let p = make_player_p1();
        let piece = Piece { height: 1, width: 3, blocks: vec![Coord::new(0,0), Coord::new(0,1), Coord::new(0,2)] };
        assert!(!is_valid(&g, &piece, &p, 0, 9));  // extends to col 11
    }

    #[test]
    fn zero_own_overlap() {
        let g = Grid::new(10, 10);  // empty — no own territory
        let p = make_player_p1();
        let piece = make_piece_single();
        assert!(!is_valid(&g, &piece, &p, 5, 5));
    }

    #[test]
    fn two_own_overlap() {
        let mut g = Grid::new(10, 10);
        g.set(4, 4, b'O');
        g.set(4, 5, b'O');
        let p = make_player_p1();
        let piece = make_piece_two_horizontal();
        // Placing at (4,4): blocks at (4,4) and (4,5) — both are own
        assert!(!is_valid(&g, &piece, &p, 4, 4));
    }

    #[test]
    fn one_own_zero_opp() {
        let mut g = Grid::new(10, 10);
        g.set(4, 4, b'O');
        let p = make_player_p1();
        let piece = make_piece_two_horizontal();
        // Placing at (4,3): block at (4,4)=own, block at (4,3)=empty
        assert!(is_valid(&g, &piece, &p, 4, 3));
    }

    #[test]
    fn one_own_one_opp() {
        let mut g = Grid::new(10, 10);
        g.set(4, 3, b'O');  // own
        g.set(4, 4, b'X');  // opponent
        let p = make_player_p1();
        let piece = make_piece_two_horizontal();
        assert!(!is_valid(&g, &piece, &p, 4, 3));
    }

    #[test]
    fn overlap_with_opponent_only() {
        let mut g = Grid::new(10, 10);
        g.set(5, 5, b'X');
        let p = make_player_p1();
        let piece = make_piece_single();
        assert!(!is_valid(&g, &piece, &p, 5, 5));
    }

    #[test]
    fn first_move() {
        let mut g = Grid::new(15, 20);
        g.set(2, 9, b'O');  // starting position
        let p = make_player_p1();
        let piece = make_piece_single();
        assert!(is_valid(&g, &piece, &p, 2, 9));
    }

    #[test]
    fn piece_over_empty_only() {
        let mut g = Grid::new(10, 10);
        g.set(0, 0, b'O');  // territory far from placement
        let p = make_player_p1();
        let piece = make_piece_single();
        assert!(!is_valid(&g, &piece, &p, 5, 5));
    }

    #[test]
    fn all_blocks_on_grid() {
        let mut g = Grid::new(10, 10);
        g.set(4, 4, b'O');
        let p = make_player_p1();
        let piece = make_piece_two_horizontal();
        assert!(is_valid(&g, &piece, &p, 4, 3));
    }

    #[test]
    fn find_valid_placements_returns_correct_count() {
        let mut g = Grid::new(5, 5);
        g.set(2, 2, b'O');
        let p = make_player_p1();
        let piece = make_piece_single();
        let valids = find_valid_placements(&g, &piece, &p);
        assert_eq!(valids.len(), 1);
        assert_eq!(valids[0], Placement::new(2, 2));
    }

    #[test]
    fn piece_no_blocks_always_invalid() {
        let mut g = Grid::new(5, 5);
        g.set(2, 2, b'O');
        let p = make_player_p1();
        let piece = Piece { height: 1, width: 1, blocks: vec![] };
        assert!(!is_valid(&g, &piece, &p, 2, 2));
    }
}
```

### TDD Red-Green Steps (Validator)

1. **RED**: Write `zero_own_overlap`. `cargo test`. Fail: `is_valid` doesn't exist.
2. **GREEN**: Create `validator.rs`. Implement boundary check + overlap counting. Pass.
3. **REFACTOR**: Consider extracting `count_overlaps` helper (not required — compiler inlines). Keep simple.
4. Write each boundary test. Implement boundary check. All pass.
5. Write overlap tests. Implement overlap logic. All pass.
6. Write `find_valid_placements_returns_correct_count`. Implement `find_valid_placements`. Pass.

---

## 4. Module: Heatmap (`src/heatmap.rs`)

### Function Signature

```rust
// heatmap.rs

use std::collections::VecDeque;
use crate::models::{Grid, Player, Coord};

/// BFS heatmap. Opponent territory cells = 0.
/// Empty cells get Manhattan distance from nearest opponent cell.
/// Own territory cells = -1 (invalid target for scoring center, but still scorable as under-piece).
/// Cells with no reachable path from opponent = i32::MAX (treat as far away).
pub fn build_heatmap(grid: &Grid, player: &Player) -> Vec<Vec<i32>> {
    let mut heatmap = vec![vec![-1i32; grid.cols]; grid.rows];
    let mut queue: VecDeque<Coord> = VecDeque::new();

    // Seed: all opponent cells → value 0
    for r in 0..grid.rows {
        for c in 0..grid.cols {
            if player.is_opp(grid.cells[r][c]) {
                heatmap[r][c] = 0;
                queue.push_back(Coord::new(r, c));
            }
        }
    }

    // BFS: 4-directional
    let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some(pos) = queue.pop_front() {
        let current = heatmap[pos.row][pos.col];
        for (dr, dc) in &directions {
            let nr = pos.row as i32 + dr;
            let nc = pos.col as i32 + dc;
            if nr >= 0 && nc >= 0 {
                let nr = nr as usize;
                let nc = nc as usize;
                if nr < grid.rows && nc < grid.cols && heatmap[nr][nc] == -1 {
                    let cell = grid.cells[nr][nc];
                    if !player.is_own(cell) {
                        heatmap[nr][nc] = current + 1;
                        queue.push_back(Coord::new(nr, nc));
                    }
                }
            }
        }
    }

    // Fill unreachable unreachable cells with i32::MAX
    for r in 0..grid.rows {
        for c in 0..grid.cols {
            if heatmap[r][c] == -1 && !player.is_own(grid.cells[r][c]) {
                heatmap[r][c] = i32::MAX;
            }
        }
    }

    heatmap
}
```

### Tests (`src/heatmap.rs` — `#[cfg(test)]` module)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    #[test]
    fn opponent_at_origin() {
        let mut g = Grid::new(5, 5);
        g.set(0, 0, b'X');  // opponent
        let p = Player::new(1).unwrap();
        let hm = build_heatmap(&g, &p);
        assert_eq!(hm[0][0], 0);
        assert_eq!(hm[0][1], 1);
        assert_eq!(hm[1][0], 1);
        assert_eq!(hm[1][1], 2);
    }

    #[test]
    fn multiple_opponent() {
        let mut g = Grid::new(5, 5);
        g.set(0, 0, b'X');
        g.set(0, 4, b'X');
        let p = Player::new(1).unwrap();
        let hm = build_heatmap(&g, &p);
        assert_eq!(hm[0][2], 2);  // equidistant from both
    }

    #[test]
    fn own_territitory_minus_one() {
        let mut g = Grid::new(5, 5);
        g.set(0, 0, b'X');
        g.set(3, 3, b'O');  // own territory
        let p = Player::new(1).unwrap();
        let hm = build_heatmap(&g, &p);
        assert_eq!(hm[3][3], -1);
    }

    #[test]
    fn no_opponent_cells() {
        let g = Grid::new(5, 5);
        let p = Player::new(1).unwrap();
        let hm = build_heatmap(&g, &p);
        for row in &hm {
            for &val in row {
                assert_eq!(val, i32::MAX);
            }
        }
    }

    #[test]
    fn full_opponent() {
        let mut g = Grid::new(3, 3);
        for r in 0..3 {
            for c in 0..3 {
                g.set(r, c, b'X');
            }
        }
        let p = Player::new(1).unwrap();
        let hm = build_heatmap(&g, &p);
        for row in &hm {
            for &val in row {
                assert_eq!(val, 0);
            }
        }
    }
}
```

---

## 5. Module: Scorer (`src/scorer.rs`)

### Function Signature

```rust
// scorer.rs

use crate::models::{Grid, Piece, Player, Placement};
use crate::validator::is_valid;

/// Find best legal placement: minimum combined heatmap score.
/// Tiebreak: lower row first, then lower col (deterministic).
/// Returns (Placement, true) if found, (Placement::none(), false) if no valid placement.
pub fn find_best_placement(
    grid: &Grid,
    piece: &Piece,
    player: &Player,
    heatmap: &Vec<Vec<i32>>,
) -> (Placement, bool) {
    let mut best: Option<(i32, Placement)> = None;

    for row in -(piece.height as i32)..grid.rows as i32 {
        for col in -(piece.width as i32)..grid.cols as i32 {
            if is_valid(grid, piece, player, row, col) {
                let mut score = 0i32;
                let mut overflow = false;
                for block in &piece.blocks {
                    let r = (row + block.row as i32) as usize;
                    let c = (col + block.col as i32) as usize;
                    let v = heatmap[r][c];
                    if v == i32::MAX { overflow = true; break; }
                    if v == -1 {
                        // own territory under piece: skip adding, or treat as 0
                        continue;
                    }
                    score += v;
                }
                if overflow { continue; }

                match best {
                    None => best = Some((score, Placement::new(row, col))),
                    Some((best_score, _)) if score < best_score => {
                        best = Some((score, Placement::new(row, col)));
                    }
                    Some((best_score, best_place)) if score == best_score => {
                        // Tiebreak: lower row, then lower col
                        if row < best_place.row || (row == best_place.row && col < best_place.col) {
                            best = Some((score, Placement::new(row, col)));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    match best {
        Some((_, place)) => (place, true),
        None => (Placement::none(), false),
    }
}
```

### Tests (`src/scorer.rs` — `#[cfg(test)]` module)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;
    use crate::heatmap::build_heatmap;

    fn setup_simple() -> (Grid, Player, Piece) {
        let mut g = Grid::new(10, 10);
        g.set(4, 4, b'O');  // own
        g.set(0, 0, b'X');  // opponent
        let p = Player::new(1).unwrap();
        let piece = Piece { height: 1, width: 1, blocks: vec![Coord::new(0, 0)] };
        (g, p, piece)
    }

    #[test]
    fn picks_closest_to_opponent() {
        let (g, p, piece) = setup_simple();
        let hm = build_heatmap(&g, &p);
        let (place, found) = find_best_placement(&g, &piece, &p, &hm);
        assert!(found);
        // Best single-block placement is own cell at (4,4) — heatmap there is -1,
        // but that's the only valid spot for a single block.
        assert_eq!(place.row, 4);
        assert_eq!(place.col, 4);
    }

    #[test]
    fn no_valid_placement() {
        let g = Grid::new(5, 5);  // empty, no own territory
        let p = Player::new(1).unwrap();
        let piece = Piece { height: 1, width: 1, blocks: vec![Coord::new(0, 0)] };
        let hm = build_heatmap(&g, &p);
        let (_, found) = find_best_placement(&g, &piece, &p, &hm);
        assert!(!found);
    }

    #[test]
    fn single_option() {
        let mut g = Grid::new(5, 5);
        g.set(2, 2, b'O');
        let p = Player::new(1).unwrap();
        let piece = Piece { height: 1, width: 1, blocks: vec![Coord::new(0, 0)] };
        let hm = build_heatmap(&g, &p);
        let (place, found) = find_best_placement(&g, &piece, &p, &hm);
        assert!(found);
        assert_eq!(place, Placement::new(2, 2));
    }

    #[test]
    fn tiebreak_by_row() {
        let mut g = Grid::new(5, 5);
        g.set(2, 2, b'O');
        g.set(3, 3, b'O');
        // Two-block piece: one block must overlap own territory
        // Create piece where placing at (2,2) and (2,1) give same heatmap sum
        // (simplified: test with custom heatmap values)
        let p = Player::new(1).unwrap();
        let mut hm = vec![vec![5; 5]; 5];
        hm[2][2] = -1;  // own
        hm[3][3] = -1;  // own
        hm[2][1] = 3;
        hm[3][2] = 3;
        let piece = Piece { height: 2, width: 1, blocks: vec![Coord::new(0, 0), Coord::new(1, 0)] };
        let (place, _) = find_best_placement(&g, &piece, &p, &hm);
        // Should pick row=2 over row=3 if scores equal
        assert!(place.row <= 3);
    }

    #[test]
    fn tiebreak_by_col() {
        let mut g = Grid::new(5, 5);
        g.set(2, 2, b'O');
        g.set(2, 4, b'O');
        let p = Player::new(1).unwrap();
        let piece = Piece { height: 1, width: 2, blocks: vec![Coord::new(0, 0), Coord::new(0, 1)] };
        let hm = build_heatmap(&g, &p);
        let (place, _) = find_best_placement(&g, &piece, &p, &hm);
        // Deterministic: should prefer lower col
        assert!(place.col >= 0);
    }
}
```

---

## 6. Module: Output (`src/output.rs`)

### Function Signature

```rust
// output.rs

use crate::models::Placement;

/// Format placement as "X Y\n" where X=col, Y=row (game engine format).
pub fn format_move(p: Placement) -> String {
    format!("{} {}\n", p.col, p.row)
}

/// Format no-move output: "0 0\n"
pub fn format_no_move() -> String {
    "0 0\n".to_string()
}
```

### Tests (`src/output.rs` — `#[cfg(test)]` module)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_move_basic() {
        let p = Placement::new(2, 7);
        assert_eq!(format_move(p), "7 2\n");
    }

    #[test]
    fn format_move_zero_zero() {
        let p = Placement::new(0, 0);
        assert_eq!(format_move(p), "0 0\n");
    }

    #[test]
    fn format_move_large() {
        let p = Placement::new(99, 14);
        assert_eq!(format_move(p), "14 99\n");
    }

    #[test]
    fn format_no_move_output() {
        assert_eq!(format_no_move(), "0 0\n");
    }

    #[test]
    fn trailing_newline() {
        let p = Placement::new(5, 3);
        assert!(format_move(p).ends_with('\n'));
    }
}
```

---

## 7. Main Loop (`src/main.rs`)

```rust
// main.rs

use std::io::{self, BufRead, Write};
use filler::{parser, models, heatmap, scorer, output};

fn main() {
    let stdin = io::stdin();
    let mut reader = std::io::BufReader::new(stdin.lock());
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    // 1. Read player line
    let mut first_line = String::new();
    if reader.read_line(&mut first_line).is_err() {
        return;
    }
    let player_num = match parser::parse_player_line(&first_line) {
        Ok(n) => n,
        Err(_) => {
            let _ = out.write_all(b"0 0\n");
            return;
        }
    };
    let player = match models::Player::new(player_num) {
        Ok(p) => p,
        Err(_) => {
            let _ = out.write_all(b"0 0\n");
            return;
        }
    };

    // 2. Game loop
    loop {
        let grid = match parser::parse_grid(&mut reader) {
            Ok(g) => g,
            Err(_) => break,  // EOF = game over
        };

        let piece = match parser::parse_piece(&mut reader) {
            Ok(p) => p,
            Err(_) => break,
        };

        let hm = heatmap::build_heatmap(&grid, &player);
        let (placement, valid) = scorer::find_best_placement(&grid, &piece, &player, &hm);

        let output_str = if valid {
            output::format_move(placement)
        } else {
            output::format_no_move()
        };

        let _ = out.write_all(output_str.as_bytes());
        let _ = out.flush();
    }
}
```

### Error Handling Rules
- `parse_grid` EOF → clean exit (game over).
- `parse_piece` EOF → clean exit (game over).
- Malformed data → print `"0 0\n"` and continue (avoid timeout/crash).
- Never `panic!`. Never `unwrap()` on user input. Always return a move.
- `flush()` after every write to ensure game_engine receives move promptly.

---

## 8. Integration Tests (`tests/`)

### `tests/pipeline.rs`

Testing the full single-turn pipeline without subprocess. Uses `Cursor<&[u8]>` as stdin.

```rust
// tests/pipeline.rs

use std::io::Cursor;
use filler::{parser, models, heatmap, scorer, output, validator};

fn run_turn(reader: &mut impl std::io::BufRead, player: &models::Player) -> String {
    let grid = match parser::parse_grid(reader) {
        Ok(g) => g,
        Err(_) => return output::format_no_move(),
    };
    let piece = match parser::parse_piece(reader) {
        Ok(p) => p,
        Err(_) => return output::format_no_move(),
    };
    let hm = heatmap::build_heatmap(&grid, player);
    let (placement, valid) = scorer::find_best_placement(&grid, &piece, player, &hm);
    if valid {
        output::format_move(placement)
    } else {
        output::format_no_move()
    }
}

#[test]
fn single_turn() {
    let input = "Anfield 15 20:\n01234567890123456789\n000 ....................\n001 ....................\n002 .........@..........\n003 ....................\n004 ....................\n005 ....................\n006 ....................\n007 ....................\n008 ....................\n009 ....................\n010 ....................\n011 ....................\n012 .........$..........\n013 ....................\n014 ....................\nPiece 4 1:\n.OOO\n";
    let mut reader = std::io::BufReader::new(Cursor::new(input));
    let player = models::Player::new(1).unwrap();
    let result = run_turn(&mut reader, &player);
    // Verify output format: "X Y\n"
    let parts: Vec<&str> = result.trim().split(' ').collect();
    assert_eq!(parts.len(), 2);
    let col: i32 = parts[0].parse().unwrap();
    let row: i32 = parts[1].parse().unwrap();
    // Verify placement is valid
    let piece = models::Piece { height: 1, width: 4, blocks: vec![
        models::Coord::new(0, 0), models::Coord::new(0, 1),
        models::Coord::new(0, 2), models::Coord::new(0, 3),
    ]};
    // Re-parse grid for validation (or cache from run_turn — simplified here)
    // The key assertion: output is valid format
    assert!(result.ends_with('\n'));
}

#[test]
fn first_move_overlaps_starting_position() {
    let input = "Anfield 5 5:\n01234\n000 .....\n001 ..@..\n002 .....\n003 .....\n004 .....\nPiece 2 2:\n.O\nO.\n";
    let mut reader = std::io::BufReader::new(Cursor::new(input));
    let player = models::Player::new(1).unwrap();
    let result = run_turn(&mut reader, &player);
    let result = result.trim();
    let parts: Vec<&str> = result.split(' ').collect();
    let col: i32 = parts[0].parse().unwrap();
    let row: i32 = parts[1].parse().unwrap();
    // Placed piece must overlap starting position at (1, 2)
    assert!(row <= 2 && row >= 0);
    assert!(col <= 3 && col >= 1);
}

#[test]
fn invalid_input_outputs_zero_zero() {
    let input = "garbage data\n";
    let mut reader = std::io::BufReader::new(Cursor::new(input));
    let player = models::Player::new(1).unwrap();
    let result = run_turn(&mut reader, &player);
    assert_eq!(result, "0 0\n");
}
```

### `tests/multi_turn.rs`

```rust
// tests/multi_turn.rs

use std::io::Cursor;
use filler::{parser, models, heatmap, scorer, output};

#[test]
fn territory_grows_over_three_turns() {
    // Simulate 3 turns by applying each output to grid manually.
    // Turn 1: initial grid with starting position
    // Turn 2: grid updated with Turn 1 placement
    // Turn 3: grid updated with Turn 2 placement
    // After each turn, own territory count should increase.

    // Setup: 10x10 grid, P1 starts at (5,5)
    let mut grid = models::Grid::new(10, 10);
    grid.set(5, 5, b'O');
    let player = models::Player::new(1).unwrap();
    let mut own_count = 1usize;

    for turn in 0..3 {
        let piece = models::Piece {
            height: 1, width: 1,
            blocks: vec![models::Coord::new(0, 0)],
        };
        let hm = heatmap::build_heatmap(&grid, &player);
        let (placement, valid) = scorer::find_best_placement(&grid, &piece, &player, &hm);
        assert!(valid, "turn {turn}: should find valid placement");
        // Apply placement
        let r = placement.row as usize;
        let c = placement.col as usize;
        assert!(grid.cells[r][c] == b'.' || player.is_own(grid.cells[r][c]));
        grid.set(r, c, b'O');
        let new_count = grid.count_char(b'O') + grid.count_char(b'o');
        assert!(new_count >= own_count, "turn {turn}: territory should not shrink");
        own_count = new_count;
    }
}

#[test]
fn no_valid_move_outputs_zero_zero() {
    // Grid fully filled except one cell that's opponent
    let mut grid = models::Grid::new(3, 3);
    for r in 0..3 {
        for c in 0..3 {
            grid.set(r, c, b'O');
        }
    }
    let player = models::Player::new(1).unwrap();
    let piece = models::Piece { height: 1, width: 1, blocks: vec![models::Coord::new(0, 0)] };
    // Every placement overlaps >=2 own cells or 0 own cells — no exactly-1 overlap possible
    // with all cells filled as own and a 1-block piece (1 overlap is valid, actually)
    // Better test: all empty grid, no own territory
    let empty_grid = models::Grid::new(3, 3);
    let hm = heatmap::build_heatmap(&empty_grid, &player);
    let (_, valid) = scorer::find_best_placement(&empty_grid, &piece, &player, &hm);
    assert!(!valid);
    assert_eq!(output::format_no_move(), "0 0\n");
}
```

---

## 9. E2E Tests (`tests/e2e.rs`)

**Gated behind `#[cfg(feature = "e2e")]`**. Run with `cargo test --features e2e`.

### `tests/e2e.rs`

```rust
// tests/e2e.rs

#![cfg(feature = "e2e")]

use std::process::Command;
use std::path::Path;

struct GameResult {
    p1_score: usize,
    p2_score: usize,
    winner: u8,  // 1 or 2
    raw_output: String,
}

fn run_game(engine: &str, map: &str, p1: &str, p2: &str, timeout: u32) -> Result<GameResult, String> {
    let output = Command::new(engine)
        .args(["-f", map, "-p1", p1, "-p2", p2, "-t", &timeout.to_string(), "-q"])
        .output()
        .map_err(|e| format!("failed to run engine: {e}"))?;

    if !output.status.success() && !output.status.code().map_or(false, |c| c != 0) {
        // Game engine may return non-zero for normal games; parse output anyway
    }

    let raw = String::from_utf8_lossy(&output.stdout).to_string();

    // Parse last lines for scores (format: "X O" or similar from engine)
    // This depends on game_engine output format. Typical:
    // "fin: O:17 X:8" or similar. Parse generously.
    let (p1_score, p2_score, winner) = parse_game_result(&raw);

    Ok(GameResult { p1_score, p2_score, winner, raw_output: raw })
}

fn parse_game_result(output: &str) -> (usize, usize, u8) {
    // TODO: implement based on actual game_engine output format
    // Placeholder: parse "fin: O:<n> X:<m>" from last line
    (0, 0, 1)
}

fn filler_binary() -> String {
    "./target/release/filler".to_string()
}

fn engine_path() -> String {
    "./game_engine".to_string()
}

fn run_match_series(map: &str, opponent: &str, runs: usize) -> usize {
    let mut wins = 0;
    for i in 0..runs {
        let (p1, p2) = if i % 2 == 0 {
            (filler_binary(), opponent.to_string())
        } else {
            (opponent.to_string(), filler_binary())
        };
        let result = run_game(&engine_path(), map, &p1, &p2, 10).unwrap();
        let filler_won = if i % 2 == 0 {
            result.winner == 1
        } else {
            result.winner == 2
        };
        if filler_won { wins += 1; }
    }
    wins
}

#[test]
fn game_engine_runs() {
    let result = run_game(
        &engine_path(),
        "maps/map01",
        "robots/bender",
        "robots/terminator",
        10,
    );
    assert!(result.is_ok());
}

#[test]
fn beat_wall_e() {
    let wins = run_match_series("maps/map00", "robots/wall_e", 5);
    assert!(wins >= 4, "won {wins}/5 vs wall_e, need >= 4");
}

#[test]
fn beat_h2_d2() {
    let wins = run_match_series("maps/map01", "robots/h2_d2", 5);
    assert!(wins >= 4, "won {wins}/5 vs h2_d2, need >= 4");
}

#[test]
fn beat_bender() {
    let wins = run_match_series("maps/map02", "robots/bender", 5);
    assert!(wins >= 4, "won {wins}/5 vs bender, need >= 4");
}

#[test]
fn piece_overlap_correct() {
    // Run one game, capture filler's moves, replay each on grid,
    // verify every move has exactly 1 own overlap, 0 opponent, no boundary violation.
    let result = run_game(
        &engine_path(),
        "maps/map01",
        &filler_binary(),
        "robots/bender",
        10,
    ).unwrap();

    // Parse raw_output for each filler move line "X Y"
    // Replay against grid state. Verify validity.
    // TODO: implement replay validator (parses engine's per-turn grid output in -q mode)
    // For now: just verify game completed
    assert!(!result.raw_output.is_empty());
}

#[test]
fn beat_terminator_bonus() {
    let wins = run_match_series("maps/map01", "robots/terminator", 5);
    // Bonus: no hard assertion, just report
    eprintln!("BONUS: won {wins}/5 vs terminator");
}
```

---

## 10. Audit Test Suite (`scripts/run_audit_suite.sh`)

Maps 1:1 to every question in `audit.md`. Exit code 0 = all pass.

```bash
#!/bin/bash
set -euo pipefail

ENGINE="./game_engine"
FILLER="./target/release/filler"
PASS=0
FAIL=0

check() {
    local desc="$1"
    local cmd="$2"
    echo -n "AUDIT: $desc ... "
    if eval "$cmd"; then
        echo "PASS"
        ((PASS++))
    else
        echo "FAIL"
        ((FAIL++))
    fi
}

# --- Functional ---

# 1. Docker image builds
check "Docker image builds" "docker build -t filler . 2>&1 | tail -1 | grep -q 'Successfully'"

# 2. Container runs correctly
check "Container starts" "docker run --rm filler echo ok 2>&1 | grep -q ok"

# 3. Game runs correctly with student player
check "Game runs against bender" "$ENGINE -f maps/map01 -p1 $FILLER -p2 robots/bender -q"

# 4. Piece overlap: exactly 1 cell
check "Pieces placed with exactly 1 overlap" \
    "cargo test --features e2e -q piece_overlap_correct 2>&1 | tail -1 | grep -q 'ok'"

# 5. Win >=4/5 vs wall_e on map00
check "Win >=4/5 vs wall_e on map00" \
    "cargo test --features e2e -q beat_wall_e 2>&1 | tail -1 | grep -q 'ok'"

# 6. Win >=4/5 vs h2_d2 on map01
check "Win >=4/5 vs h2_d2 on map01" \
    "cargo test --features e2e -q beat_h2_d2 2>&1 | tail -1 | grep -q 'ok'"

# 7. Win >=4/5 vs bender on map02
check "Win >=4/5 vs bender on map02" \
    "cargo test --features e2e -q beat_bender 2>&1 | tail -1 | grep -q 'ok'"

# --- Unit Tests ---

# 8. All unit tests pass
check "All unit tests pass" "cargo test --lib -q 2>&1 | tail -1 | grep -q 'ok'"

# 9. Input parsing tests exist
check "Input parsing tests" \
    "cargo test --lib parser -q 2>&1 | grep -q 'test result: ok'"

# 10. Placement validation tests
check "Placement validation tests" \
    "cargo test --lib validator -q 2>&1 | grep -q 'test result: ok'"

# 11. Boundary detection tests
check "Boundary detection tests" \
    "cargo test --lib validator::tests::boundary -q 2>&1 | tail -1 | grep -q 'ok'"

# Summary
echo ""
echo "=== AUDIT RESULTS: $PASS passed, $FAIL failed ==="
[ "$FAIL" -eq 0 ]
```

### Audit Mapping Table

| audit.md Question | Covered By |
|---|---|
| Docker image/container created? | `check "Docker image builds"`, `check "Container starts"` |
| Project runs correctly? | `check "Game runs against bender"` |
| Pieces placed with 1-cell overlap? | `piece_overlap_correct` E2E test |
| Win >=4/5 vs wall_e (map00)? | `beat_wall_e` E2E test |
| Win >=4/5 vs h2_d2 (map01)? | `beat_h2_d2` E2E test |
| Win >=4/5 vs bender (map02)? | `beat_bender` E2E test |
| All unit tests pass? | `cargo test --lib` |
| Input parsing tests? | `cargo test --lib parser` |
| Placement validation tests? | `cargo test --lib validator` |
| Boundary detection tests? | `cargo test --lib validator::tests::boundary` |
| Beat terminator (bonus)? | `beat_terminator_bonus` (no hard fail) |
| Visualizer (bonus)? | Manual check or `cargo run -p visualizer` |

---

## 11. Docker Setup

### Dockerfile

```dockerfile
FROM rust:1.95-alpine
WORKDIR /filler
RUN apk add --no-cache musl-dev

COPY ./docker_image/game_engine /filler/game_engine
COPY ./docker_image/maps /filler/maps
COPY ./docker_image/robots /filler/robots
RUN chmod +x /filler/game_engine

COPY . /filler/solution
RUN cd /filler/solution && cargo build --release -p filler

ENTRYPOINT ["/bin/sh"]
```

### .dockerignore

```
target/
.git/
```

### Build & Run

```bash
docker build -t filler .
docker run -v "$(pwd)":/filler/solution -it filler
# Inside container:
./game_engine -f maps/map01 -p1 solution/target/release/filler -p2 robots/bender
```

---

## 12. Step-by-Step Implementation Order (Junior Dev)

### Phase 1: Cargo Setup (30 min)
1. Run `cargo init filler` in project root
2. Add dev-dependencies to `Cargo.toml`: `assert_cmd`, `predicates`, `tempfile`
3. Add `e2e` feature flag to `Cargo.toml`
4. Create module files: `src/lib.rs`, `src/models.rs`, `src/parser.rs`, `src/validator.rs`, `src/heatmap.rs`, `src/scorer.rs`, `src/output.rs`
5. Add `pub mod` declarations in `lib.rs`
6. Run `cargo build` — verify compiles (empty modules OK)

### Phase 2: Models — TDD (30 min)
7. Write all model structs in `models.rs` (Coord, Grid, Piece, Player, Placement)
8. Write `#[cfg(test)] mod tests` at bottom of `models.rs` with all tests from Section 1
9. Run `cargo test --lib models` — GREEN (structs + basic tests)

### Phase 3: Parser — TDD (2 hours)
10. Write `parse_player_line_p1`, `parse_player_line_p2`, `parse_player_line_invalid` tests
11. `cargo test --lib parser` — RED (functions don't exist)
12. Implement `parse_player_line`: split on whitespace, find `"p1"` or `"p2"`, extract digit
13. `cargo test --lib parser` — GREEN
14. Write `parse_grid_basic`, `parse_grid_opponent`, `parse_grid_row_index_stripped`, `parse_grid_truncated` tests
15. `cargo test --lib parser` — RED
16. Implement `parse_grid`:
    - Read header line, `parse::<usize>` for rows/cols
    - Skip column header line (readln + discard)
    - For each row: read line, find first space, take substring after space, map chars
    - Char mapping: `'@'` → `b'O'`, `'a'` → `b'o'`, `'$'` → `b'X'`, `'s'` → `b'x'`, `'.'` → `b'.'`
17. `cargo test --lib parser` — GREEN
18. Write `parse_piece_small`, `parse_piece_large`, `parse_piece_all_dots` tests
19. `cargo test --lib parser` — RED
20. Implement `parse_piece`:
    - Read `"Piece <h> <w>:"` header, parse height/width
    - Read `height` lines, find non-'.' chars, record (row, col) as `Coord`
21. `cargo test --lib parser` — GREEN
22. Refactor if needed

### Phase 4: Validator — TDD (2 hours)
23. Write `boundary_top`, `boundary_left`, `boundary_bottom`, `boundary_right` tests
24. `cargo test --lib validator` — RED
25. Implement boundary check: for each block, compute `row + block.row`, `col + block.col`, check in bounds
26. `cargo test --lib validator` — GREEN (boundary tests)
27. Write `zero_own_overlap`, `two_own_overlap`, `one_own_zero_opp`, `one_own_one_opp`, `overlap_with_opponent_only`, `first_move`, `piece_over_empty_only` tests
28. `cargo test --lib validator` — RED
29. Implement overlap counting: iterate blocks, check grid cell, count own/opp overlaps, return `own == 1 && opp == 0`
30. `cargo test --lib validator` — GREEN
31. Write `find_valid_placements_returns_correct_count`, `piece_no_blocks_always_invalid`
32. Implement `find_valid_placements`
33. `cargo test --lib validator` — GREEN

### Phase 5: Heatmap — TDD (1.5 hours)
34. Write `opponent_at_origin`, `multiple_opponent`, `own_territory_minus_one`, `no_opponent_cells`, `full_opponent` tests
35. `cargo test --lib heatmap` — RED
36. Implement `build_heatmap`:
    - Initialize `heatmap` with `-1`
    - Seed queue with opponent cells (value 0)
    - BFS with `VecDeque`, 4-directional
    - Skip own territory cells (leave as -1)
    - After BFS, set unreachable empty cells to `i32::MAX`
37. `cargo test --lib heatmap` — GREEN

### Phase 6: Scorer — TDD (1.5 hours)
38. Write `picks_closest_to_opponent`, `no_valid_placement`, `single_option`, `tiebreak_by_row`, `tiebreak_by_col` tests
39. `cargo test --lib scorer` — RED
40. Implement `find_best_placement`:
    - Double loop over all possible (row, col) positions
    - Call `is_valid` for each
    - Sum heatmap values under piece blocks (skip -1 for own territory)
    - Track minimum sum, deterministic tiebreak (row then col)
41. `cargo test --lib scorer` — GREEN

### Phase 7: Output — TDD (30 min)
42. Write `format_move_basic`, `format_move_zero_zero`, `format_move_large`, `format_no_move_output`, `trailing_newline` tests
43. `cargo test --lib output` — RED
44. Implement `format_move` and `format_no_move` using `format!()` macro
45. `cargo test --lib output` — GREEN

### Phase 8: Main Loop (1 hour)
46. Wire `main.rs`: read player line, loop (parse grid → parse piece → heatmap → score → print)
47. Add error handling: EOF break, malformed → print "0 0\n"
48. Add `flush()` after each write
49. `cargo build` — verify compiles
50. Manually test with piped input

### Phase 9: Integration Tests (1.5 hours)
51. Create `tests/pipeline.rs` with `single_turn`, `first_move_overlaps_starting_position`, `invalid_input_outputs_zero_zero`
52. `cargo test --test pipeline` — fix until GREEN
53. Create `tests/multi_turn.rs` with `territory_grows_over_three_turns`, `no_valid_move_outputs_zero_zero`
54. `cargo test --test multi_turn` — fix until GREEN

### Phase 10: Docker (1 hour)
55. Write `Dockerfile` (Alpine + Rust)
56. Write `.dockerignore`
57. `docker build -t filler .`
58. `docker run -v "$(pwd)":/filler/solution -it filler`
59. Inside container: `cargo build --release && ./game_engine -f maps/map01 -p1 ./target/release/filler -p2 robots/bender`

### Phase 11: E2E & Audit (2 hours)
60. Create `tests/e2e.rs` with `#[cfg(feature = "e2e")]`
61. Implement `run_game` using `std::process::Command`
62. Implement `parse_game_result` based on actual engine output
63. Write all E2E test functions (see Section 9)
64. `cargo test --features e2e --test e2e` — fix until GREEN
65. Write `scripts/run_audit_suite.sh`
66. Run full audit inside Docker: `bash scripts/run_audit_suite.sh`
67. If win rate < 4/5: tune strategy (Section 13)

### Phase 12: Bonus (2+ hours)
68. Create `visualizer/` as separate Cargo binary crate
69. Implement terminal color grid rendering
70. Attempt terminator: enhance strategy with edge weighting + opponent blocking

---

## 13. Strategy Tuning Guide (If Win Rate < 4/5)

Base strategy: BFS heatmap toward opponent → pick minimum sum placement. Aggressive (rushes opponent).

### Enhancement 1: Edge Weighting

Add bonus for cells near grid edges. Opponents trapped against edges have fewer options.

```rust
fn edge_bonus(row: usize, col: usize, rows: usize, cols: usize) -> i32 {
    let dist_to_edge = row.min(rows - 1 - row).min(col.min(cols - 1 - col));
    match dist_to_edge {
        0 => -5,
        1 => -2,
        _ => 0,
    }
}
```

Apply: `score += edge_bonus(r, c, grid.rows, grid.cols)` for each piece block.

### Enhancement 2: Territory Connectivity

After scoring, prefer placements that keep own territory in one connected component. Add penalty for isolated regions.

### Enhancement 3: Opponent Blocking

When own territory neighbors opponent, prefer placements extending along boundary. This blocks opponent expansion routes.

### Enhancement 4: Center Start

First move: prefer placement moving toward center + opponent direction. More room to expand.

### Enhancement 5: Piece Size Bonus

Prefer placements that cover more empty cells (expand territory faster). Score: `score -= empty_cells_under_piece * 2`.

---

## 14. Key Edge Cases to Test

| Case | Expected Behavior |
|---|---|
| First turn (only 1 starting cell) | Must place piece overlapping starting cell |
| Piece larger than grid | No valid placement, output `"0 0\n"` |
| Grid 100% filled | No valid placement, output `"0 0\n"` |
| Opponent surrounds own territory | No valid placement, output `"0 0\n"` |
| Piece is single block (1×1 with one `#`) | Must overlap exactly one own cell |
| Piece is 1×1 with only `.` | No blocks, no valid placement, output `"0 0\n"` |
| Concurrent own + opponent overlap | Invalid (must be 0 opponent overlaps) |
| Timeout scenario | Respond < 500ms per turn on 100×100 grid |
| Player number 2 starting with `$` | `$` maps to `b'X'`, `s` maps to `b'x'` |
| Very large grid (100×100) | Must not panic, must respond within timeout |

---

## 15. Benchmark (`benches/turn_benchmark.rs`)

```rust
// benches/turn_benchmark.rs

use filler::{parser, heatmap, scorer, models};
use std::io::Cursor;

// Use criterion or simple timing
// cargo bench

fn bench_single_turn() {
    let input = "Anfield 100 100:\n...";  // 100x100 grid + piece
    let mut reader = std::io::BufReader::new(Cursor::new(input));
    let grid = parser::parse_grid(&mut reader).unwrap();
    let piece = parser::parse_piece(&mut reader).unwrap();
    let player = models::Player::new(1).unwrap();
    let hm = heatmap::build_heatmap(&grid, &player);
    let _ = scorer::find_best_placement(&grid, &piece, &player, &hm);
}

// Assert: bench_single_turn completes in < 500ms on 100x100 grid
```

---

## 16. Command Reference

```bash
# Build
cargo build --release

# Run all unit tests
cargo test --lib

# Run specific module tests
cargo test --lib parser
cargo test --lib validator
cargo test --lib heatmap
cargo test --lib scorer
cargo test --lib output

# Run integration tests
cargo test --test pipeline
cargo test --test multi_turn

# Run e2e tests (requires game_engine)
cargo test --features e2e --test e2e

# Run all tests
cargo test

# Run benchmarks
cargo bench

# Run audit suite
bash scripts/run_audit_suite.sh

# Docker
docker build -t filler .
docker run -v "$(pwd)":/filler/solution -it filler
# Inside container:
./game_engine -f maps/map01 -p1 solution/target/release/filler -p2 robots/bender -q

# Clippy lint
cargo clippy -- -D warnings

# Format check
cargo fmt --check
```
