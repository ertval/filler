# Filler Implementation Plan — TDD in Rust (DeepPro Edition)

Step-by-step guide for implementing the `filler` project in Rust.
Follows **Test-Driven Development** (write failing test → implement → refactor → repeat).
Every audit question in `requirements/audit.md` maps to a specific test or E2E check.

---

## 0. Project Structure

```
filler/
├── Cargo.toml                # Rust project manifest
├── Dockerfile                # Multi-stage Rust build
├── .dockerignore             # Docker build exclusions
│
├── src/
│   ├── main.rs               # Entry point: stdin → parse → decide → stdout loop
│   ├── lib.rs                # Re-export all modules for testing
│   ├── types.rs              # Shared data structures & constants
│   ├── parser.rs             # Parse stdin into GameState
│   ├── validator.rs          # Placement legality checks
│   ├── strategy.rs           # Heatmap + placement scoring
│   ├── output.rs             # Output formatting
│   └── visualizer.rs         # Bonus: terminal visualizer
│
├── tests/
│   ├── common/mod.rs         # Shared test helpers & fixtures
│   ├── parser_tests.rs       # Unit tests for parser
│   ├── validator_tests.rs    # Unit tests for validator
│   ├── strategy_tests.rs     # Unit tests for strategy
│   ├── output_tests.rs       # Unit tests for output formatting
│   ├── integration_tests.rs  # Full pipeline integration tests
│   ├── multi_turn.rs         # Multi-turn territory growth integration test
│   ├── e2e.rs                # E2E replay validation test (gated by feature flag)
│   └── visualizer_tests.rs   # Bonus
│
├── benches/
│   └── turn_benchmark.rs     # Benchmark harness for 100x100 grid performance
│
├── testdata/                 # Fixture files: sample stdin dumps
│   ├── player1_start.txt
│   ├── player2_start.txt
│   ├── anfield_20x15.txt
│   ├── anfield_30x14.txt
│   ├── piece_2x2.txt
│   ├── piece_5x4.txt
│   ├── piece_6x3.txt
│   ├── turn_full_p1.txt
│   └── turn_full_p2.txt
│
├── e2e/
│   ├── run_audit_suite.sh    # E2E script covering every audit question
│   └── assert_winrate.rs     # Parse game_engine output, count wins
│
└── scripts/
    └── run_all_tests.sh      # Run all: unit → integration → e2e
```

---

## 1. Data Structures & Types — `src/types.rs`

```rust
use std::fmt;

/// Player identity as reported by game engine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    P1,
    P2,
}

impl Player {
    /// Returns the two characters representing this player on the grid.
    /// P1 → ('@', 'a')   P2 → ('$', 's')
    pub fn chars(self) -> (char, char) {
        match self {
            Player::P1 => ('@', 'a'),
            Player::P2 => ('$', 's'),
        }
    }
}

/// A single cell on the Anfield
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Player1Recent,  // 'a'
    Player1Old,     // '@'
    Player2Recent,  // 's'
    Player2Old,     // '$'
}

impl Cell {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Cell::Empty,
            '@' => Cell::Player1Old,
            'a' => Cell::Player1Recent,
            '$' => Cell::Player2Old,
            's' => Cell::Player2Recent,
            _   => Cell::Empty, // defensive
        }
    }

    /// Returns true if this cell belongs to the given player
    pub fn belongs_to(self, player: Player) -> bool {
        match (self, player) {
            (Cell::Player1Old | Cell::Player1Recent, Player::P1) => true,
            (Cell::Player2Old | Cell::Player2Recent, Player::P2) => true,
            _ => false,
        }
    }
}

/// 2D grid representing the Anfield
#[derive(Debug, Clone)]
pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<Cell>>,
}

/// A point in the grid (row, col) — row is Y, col is X in output.
/// Uses signed `i32` to support negative placement offsets (piece padding).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub row: i32,
    pub col: i32,
}

/// A random game piece
#[derive(Debug, Clone)]
pub struct Piece {
    pub rows: usize,
    pub cols: usize,
    /// List of filled cell positions relative to the piece's top-left corner.
    /// Each point is (row_offset, col_offset).
    pub blocks: Vec<(usize, usize)>,
}

/// Full game state for the current turn
#[derive(Debug, Clone)]
pub struct GameState {
    pub me: Player,
    pub opponent: Player,
    pub grid: Grid,
    pub piece: Piece,
}
```

### Coordinate Convention

The game engine uses `X Y\n` format:
- **X = column** (0-indexed, left to right)
- **Y = row** (0-indexed, top to bottom)

Internally store `Point { row, col }`. Format using `src/output.rs`:
```rust
output::format_move(point)
```

---

### Architectural Variant: Byte-Level Grid Storage (Performance Optimization)

Instead of a type-safe `Cell` enum, we can store grid cells as `u8` bytes (`b'O'`, `b'X'`, `b'.'`, etc.).
- **Advantages:** Single-byte storage (cache-friendly), branchless comparisons (e.g. `cell == char_up || cell == char_lo`), zero allocation.
- **Trade-offs:** Less type safety, implicit char-to-meaning mappings. Recommended as a post-MVP performance optimization variant.

---

## 2. Module A: Parser — `src/parser.rs`

### Goal
Read from `BufRead` (stdin in prod, `Cursor<&str>` / `&[u8]` in tests) and produce `GameState`.

### Input Format (exact)

Each turn sends these lines:

1. **Player line** (first turn only):
   ```
   $$$ exec p1 : [robots/bender]
   ```

2. **Anfield header**:
   ```
   Anfield <cols> <rows>:
   ```
   Example: `Anfield 20 15:`

3. **Column header** (skip):
   ```
       01234567890123456789
   ```
   Starts with 4 spaces, then `cols` digits.

4. **Grid rows** (`rows` lines):
   ```
   000 ....................
   001 ....................
   ```
   Each: 3-digit row number + space + `cols` characters.

5. **Piece header**:
   ```
   Piece <cols> <rows>:
   ```
   Example: `Piece 4 1:`

6. **Piece rows** (`rows` lines):
   ```
   .OO.
   ```
   `.` = empty, `O` or `*` = filled.

### Step-by-Step Implementation

#### A1: Parse Player Identity

**Function signature:**
```rust
pub fn parse_player_line(line: &str) -> Result<Player, String>
```

**Test cases (`tests/parser_tests.rs`):**

| Input line | Expected |
|-----------|----------|
| `$$$ exec p1 : [robots/bender]` | `Ok(Player::P1)` |
| `$$$ exec p2 : [robots/bender]` | `Ok(Player::P2)` |
| `$$$ exec p1 : [/path/to/robot]` | `Ok(Player::P1)` |
| `$$$ exec pX : [whatever]` | `Err(...)` |
| Empty string | `Err(...)` |
| Line without `$$$ exec p` prefix | `Err(...)` |

**Implementation sketch:**
```rust
pub fn parse_player_line(line: &str) -> Result<Player, String> {
    if !line.starts_with("$$$ exec p") || line.len() < 11 {
        return Err(format!("invalid player line: {line}"));
    }
    match line.as_bytes()[10] {
        b'1' => Ok(Player::P1),
        b'2' => Ok(Player::P2),
        _    => Err(format!("unknown player in line: {line}")),
    }
}
```

#### A2: Parse Anfield Grid

**Function signature:**
```rust
pub fn parse_anfield<R: BufRead>(reader: &mut R) -> Result<Grid, String>
```

**Test cases:**

| Input fixture | Expected |
|--------------|----------|
| 3x3 grid, P1 at (1,1), P2 at (2,2) | Grid { 3, 3, cells match } |
| Header `Anfield 6 4:` | Grid { 4, 6, ... } — 4 rows, 6 cols |
| All dots | All cells = `Cell::Empty` |
| Mixed 'a', '@', 's', '$' | Correct `Cell` variants |
| Missing "Anfield" prefix | `Err(...)` |
| Negative / zero dimensions in header | `Err(...)` |

**Implementation sketch:**
```rust
pub fn parse_anfield<R: BufRead>(reader: &mut R) -> Result<Grid, String> {
    let header = read_line(reader)?;
    let (cols, rows) = parse_anfield_header(&header)?;

    // Skip column header line
    read_line(reader)?;

    let mut data = Vec::with_capacity(rows);
    for _ in 0..rows {
        let line = read_line(reader)?;
        // Strip first 4 chars: "000 " for row number prefix
        if line.len() < 4 + cols {
            return Err("row line too short".into());
        }
        let row_chars = &line[4..4 + cols];
        let row: Vec<Cell> = row_chars.chars().map(Cell::from_char).collect();
        data.push(row);
    }

    Ok(Grid { rows, cols, data })
}

fn parse_anfield_header(line: &str) -> Result<(usize, usize), String> {
    // "Anfield 20 15:"
    let line = line.strip_prefix("Anfield ").ok_or("missing Anfield prefix")?;
    let line = line.strip_suffix(':').ok_or("missing colon")?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("expected 'cols rows' in header".into());
    }
    let cols: usize = parts[0].parse().map_err(|_| "invalid cols")?;
    let rows: usize = parts[1].parse().map_err(|_| "invalid rows")?;
    Ok((cols, rows))
}
```

#### A3: Parse Piece

**Function signature:**
```rust
pub fn parse_piece<R: BufRead>(reader: &mut R) -> Result<Piece, String>
```

**Test cases:**

| Input | Expected |
|-------|----------|
| `Piece 2 2:\n.#\n#.\n` | `Piece { 2, 2, blocks: [(0,1), (1,0)] }` |
| `Piece 4 1:\n.OO.\n` | `Piece { 1, 4, blocks: [(0,1), (0,2)] }` |
| `Piece 3 3:\n...\n...\n...\n` | `Piece { 3, 3, blocks: [] }` — empty |
| Piece with `*` fill chars | Same as `O` — both treated as blocks |
| All filled (`OOO\nOOO\n`) | 6 blocks |
| Malformed header | `Err(...)` |

**Implementation sketch:**
```rust
pub fn parse_piece<R: BufRead>(reader: &mut R) -> Result<Piece, String> {
    let header = read_line(reader)?;
    let (cols, rows) = parse_piece_header(&header)?;

    let mut blocks = Vec::new();
    for r in 0..rows {
        let line = read_line(reader)?;
        for (c, ch) in line.chars().enumerate() {
            if c >= cols { break; }
            if ch != '.' {
                blocks.push((r, c));
            }
        }
    }

    Ok(Piece { rows, cols, blocks })
}

fn parse_piece_header(line: &str) -> Result<(usize, usize), String> {
    let line = line.strip_prefix("Piece ").ok_or("missing Piece prefix")?;
    let line = line.strip_suffix(':').ok_or("missing colon")?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("expected 'cols rows' in piece header".into());
    }
    let cols: usize = parts[0].parse().map_err(|_| "invalid cols")?;
    let rows: usize = parts[1].parse().map_err(|_| "invalid rows")?;
    Ok((cols, rows))
}
```

#### A4: Full Turn Parser

**Function signature:**
```rust
/// Parses a complete turn from stdin.
/// `state` is `None` on first call (before player identity known).
/// Returns the game state. On first turn, player identity is parsed.
pub fn parse_turn<R: BufRead>(
    reader: &mut R,
    state: Option<GameState>,
) -> Result<GameState, String>
```

**Test cases (`tests/integration_tests.rs`):**

| Input | Expected |
|-------|----------|
| Full stdin for P1 with 20x15 anfield and 4x1 piece | All fields populated |
| Full stdin for P2 | P2 identity |
| Two consecutive turns (player line only sent first time) | State carries over, second turn has same player |

**Implementation sketch:**
```rust
pub fn parse_turn<R: BufRead>(
    reader: &mut R,
    state: Option<GameState>,
) -> Result<GameState, String> {
    let (me, opponent) = if let Some(ref s) = state {
        (s.me, s.opponent)
    } else {
        let player_line = read_line(reader)?;
        let me = parse_player_line(&player_line)?;
        let opponent = match me {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        };
        (me, opponent)
    };

    let grid = parse_anfield(reader)?;
    let piece = parse_piece(reader)?;

    Ok(GameState { me, opponent, grid, piece })
}
```

#### A5: Helper — Read a line

```rust
fn read_line<R: BufRead>(reader: &mut R) -> Result<String, String> {
    let mut buf = String::new();
    reader.read_line(&mut buf).map_err(|e| e.to_string())?;
    Ok(buf.trim_end_matches('\n').to_string())
}
```

---

## 3. Module B: Placement Validator — `src/validator.rs`

### Goal
Given a `Grid`, `Piece`, target `Point`, and the current player — return whether the placement is legal.

### Rules
1. Exactly **one** cell of the piece must overlap with **our** territory
2. **Zero** cells of the piece may overlap with the **opponent's** territory
3. All piece cells must fit inside grid boundaries

### Step-by-Step Implementation

#### B1: Boundary Check

**Function signature:**
```rust
pub fn is_in_bounds(grid: &Grid, piece: &Piece, target: Point) -> bool
```

**Test cases (`tests/validator_tests.rs`):**

| Scenario | Grid | Piece | Target | Expected |
|----------|------|-------|--------|----------|
| Fits perfectly | 10x10 | 2x2 | (0,0) | `true` |
| Extends past right edge | 10x10 | 2x2 | (0,9) | `false` |
| Extends past bottom edge | 10x10 | 2x2 | (9,0) | `false` |
| Extends past both | 10x10 | 2x2 | (9,9) | `false` |
| Negative check: target=(-1,0) piece with block at (0,0) | 5x5 | 1x2 horizontal | (-1,0) | `false` |
| Fits exactly at boundary | 10x10 | 2x2 | (8,8) | `true` |
| 1x1 piece anywhere in bounds | 10x10 | 1x1 | (5,5) | `true` |

**Implementation sketch:**
```rust
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
```

#### B2: Overlap Logic

**Function signature:**
```rust
pub fn is_valid_placement(
    grid: &Grid,
    piece: &Piece,
    target: Point,
    player: Player,
) -> bool
```

**Test cases (`tests/validator_tests.rs`):**

Use a shared 5x5 fixture (created in `tests/common/mod.rs`):
```
@ . . . .
. . . . .
. . . . .
. . . $ .
. . . . .
```
P1 at (0,0), P2 at (3,3)

| # | Scenario | Piece | Target | Expected |
|---|----------|-------|--------|----------|
| 1 | Zero own overlap (far away) | 1x1 `[O]` | (2,2) | `false` |
| 2 | Two own overlaps | 1x2 horiz `[OO]` on grid `@ @ . . .` | (0,0) | `false` |
| 3 | One own + one opponent overlap | 1x2 horiz `[OO]` | Place so one cell hits own, one hits opponent | `false` |
| 4 | Exactly 1 own, 0 opponent — VALID | 1x2 horiz `[OO]` at (0,0) on grid `@ . . . .` | (0,0) | `true` |
| 5 | Exactly 1 own (via `a` recent char) | Same grid but own cell is `a` | 1x2 `[OO]` at (0,0) | `true` |
| 6 | Opponent cell is `$` (old) | Opponent at (3,3) is `$`; piece covers it | 1x1 `[O]` at (3,3) | `false` |
| 7 | Opponent cell is `s` (recent) | Same, opponent char is `s` | 1x1 `[O]` at (3,3) | `false` |
| 8 | Piece covers both own AND empty cells | 2x2 `[OO/OO]` at (0,0) | (0,0) | `true` |
| 9 | Zero own + zero opponent (all empty) | 1x1 `[O]` at (2,2) all empty grid | (2,2) | `false` |
| 10 | Piece larger than grid — caught by bounds | 6x6 piece on 5x5 | (0,0) | `false` |

**Implementation sketch:**
```rust
pub fn is_valid_placement(
    grid: &Grid,
    piece: &Piece,
    target: Point,
    player: Player,
) -> bool {
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
```

#### B3: Find All Valid Placements

**Function signature:**
```rust
pub fn find_valid_placements(
    grid: &Grid,
    piece: &Piece,
    player: Player,
) -> Vec<Point>
```

**Test cases:**

| Scenario | Expected |
|----------|----------|
| 5x5 grid, P1 at (2,2), piece 1x2 vertical `[O/O]` | Returns 2 valid placements (above and below own cell) |
| Negative-offset placement: piece 2x2 with blocks at (1,1), target (-1,-1) covers (0,0) | Returns valid placement at (-1,-1) |
| No valid placement exists | Returns empty `Vec` |

**Implementation sketch:**
```rust
pub fn find_valid_placements(
    grid: &Grid,
    piece: &Piece,
    player: Player,
) -> Vec<Point> {
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
```

---## 4. Module C: Strategy Algorithm — `src/strategy.rs`

### Goal
Select the best placement from all valid placements.

### Strategy: Aggressive Heatmap (Distance BFS)

Create a distance heatmap from the opponent's territory. Choose placements closest to the opponent to cut off their expansion space.

#### C1: Generate Heatmap (BFS from opponent territory)

**Function signature:**
```rust
pub fn generate_heatmap(grid: &Grid, opponent: Player, me: Player) -> Vec<Vec<i32>>
```

**Test cases (`tests/strategy_tests.rs`):**

| Scenario | Expected |
|----------|----------|
| 5x5 grid, opponent at (0,0), rest empty | heatmap[0][0]=0, neighbors=1, diagonal=2... |
| 5x5 grid, me at (4,4) | heatmap[4][4]=-1 (own territory distinguishable from unreachable) |
| 5x5 grid, opponent at (2,2) AND (2,3) | Both at 0, BFS from both simultaneously |
| 3x3 grid, opponent at (1,1) | Center=0, edges=1, corners=2 |

**Implementation sketch:**
```rust
use std::collections::VecDeque;

pub fn generate_heatmap(grid: &Grid, opponent: Player, me: Player) -> Vec<Vec<i32>> {
    let rows = grid.rows;
    let cols = grid.cols;
    let mut heatmap = vec![vec![i32::MAX; cols]; rows];
    let mut queue = VecDeque::new();

    for r in 0..rows {
        for c in 0..cols {
            if grid.data[r][c].belongs_to(opponent) {
                heatmap[r][c] = 0;
                queue.push_back((r, c));
            } else if grid.data[r][c].belongs_to(me) {
                heatmap[r][c] = -1; // own territory identifier
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

    heatmap
}
```

#### C2: Score a Placement

**Function signature:**
```rust
pub fn score_placement(
    heatmap: &[Vec<i32>],
    piece: &Piece,
    target: Point,
) -> i32
```

**Test cases:**

| Scenario | Expected |
|----------|----------|
| 5x5, opponent at (4,4), placement at (0,0) with 1x1 | Score = 8 |
| Same, placement at (3,3) with 1x1 | Score = 2 |
| Placement covering cell with own territory (-1) | Score skips -1 cell (no penalty) |
| Placement on cell with sentinel (i32::MAX) | Score saturates / penalized |

**Implementation sketch:**
```rust
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
        if h == -1 {
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
```

#### C3: Choose Best Placement

**Function signature:**
```rust
pub fn choose_best_placement(
    placements: &[Point],
    heatmap: &[Vec<i32>],
    piece: &Piece,
) -> Option<Point>
```

**Test cases:**

| Scenario | Expected |
|----------|----------|
| Two placements with scores 5 and 3 | Chooses score-3 placement |
| Equal scores (tiebreak test) | Chooses lower row; if same row, lower col |
| Empty placements slice | Returns `None` |
| Single placement | Returns that placement |

**Implementation sketch:**
```rust
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
```

#### C4: Advanced Territory Control (Bonus)

For beating `terminator`, upgrade to dual-heatmap strategy:

```rust
/// Generate heatmaps from BOTH our territory and opponent territory.
pub fn generate_dual_heatmaps(
    grid: &Grid,
    me: Player,
    opponent: Player,
) -> (Vec<Vec<i32>>, Vec<Vec<i32>>)

/// Score based on territory advantage:
///   advantage = dist_to_me - dist_to_opponent
///   Negative = closer to us (good). Positive = closer to opponent (aggressive).
///   For aggressive: prefer cells slightly toward opponent (small positive).
///   For defensive: prefer cells closer to us (negative).
pub fn score_placement_advanced(
    my_dist: &[Vec<i32>],
    opp_dist: &[Vec<i32>],
    piece: &Piece,
    target: Point,
    aggression: f64,  // 1.0 = full aggressive, 0.0 = defensive
) -> i32
```

---

#### C5: Strategy Tuning Guide (Win-Rate Insurance)

If base strategy win-rate falls below 80% vs specific bots (e.g. `bender`, `terminator`), implement the following enhancement catalog to adjust behavior:

| Enhancement | Description | Implementation Hint |
|---|---|---|
| **Edge Weighting** | Bonus for cells near grid edges; traps opponent. | `edge_bonus(row, col, rows, cols)` returns -5 / -2 / 0 based on distance-to-edge. |
| **Territory Connectivity** | Prefer placements keeping own territory connected. | Add scoring penalty if placement creates isolated/disconnected own sub-regions. |
| **Opponent Blocking** | Extend along boundary when adjacent to opponent. | Score bonus for cells adjacent to opponent cells (`score -= 5`). |
| **Center Start Bias** | First move favors center + opponent direction. | Add extra weight to center proximity on Turn 1 to secure center space early. |
| **Piece Size Bonus** | Prefer placements covering more empty cells. | Add bonus to placements that overlap more empty cells (`score -= empty_cells_under_piece * 2`). |

> [!NOTE]
> **Code Structure Guideline:** If strategy logic expands beyond ~150 lines (e.g., adding edge-weighting, blocking, or dual heatmaps), split `src/strategy.rs` into `src/heatmap.rs` (heatmap generation) and `src/scorer.rs` (all placement scoring functions and tiebreaks) to maintain modular testability.

---

## 4.5. Module D: Output Formatter — `src/output.rs`

### Goal
Format outputs sent to stdout cleanly with trailing newlines. Isolating formatting here ensures it is testable separate from stdin/stdout piping.

### Implementation sketch

```rust
use crate::types::Point;

/// Formats a valid placement point as "X Y\n" for the game engine.
/// Converts internal (row, col) representation to game engine (col, row) coordinates.
pub fn format_move(p: Point) -> String {
    format!("{} {}\n", p.col, p.row)
}

/// Formats a fallback "0 0\n" when no valid moves are possible.
pub fn format_no_move() -> String {
    "0 0\n".to_string()
}
```

---

## 5. Main Loop — `src/main.rs`

### Error Handling & Guardrails
- **Central Rule:** **Never panic! or unwrap() on user/game-engine input. Always return a move.**
- Centralized guardrails:
  - If Anfield or Piece headers are malformed: print error to stderr, output `"0 0\n"`, and continue.
  - If stdin receives EOF during parser execution: exit cleanly with `Ok(())`.
  - Under no circumstances should the program crash.

```rust
use std::io::{self, BufRead, Write, BufWriter};
use filler::parser;
use filler::validator;
use filler::strategy;
use filler::output;
use filler::types::GameState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());
    let stdout = io::stdout();
    // Wrap stdout in BufWriter for efficient buffered writes
    let mut writer = BufWriter::new(stdout.lock());

    let mut state: Option<GameState> = None;

    loop {
        let turn = match parser::parse_turn(&mut reader, state.clone()) {
            Ok(t) => t,
            Err(e) => {
                // Exit cleanly on EOF
                if e.contains("EOF") || e.contains("unexpected end of file") {
                    break;
                }
                eprintln!("Parse error: {e}");
                writer.write_all(output::format_no_move().as_bytes())?;
                writer.flush()?;
                continue;
            }
        };

        let valid = validator::find_valid_placements(
            &turn.grid,
            &turn.piece,
            turn.me,
        );

        if valid.is_empty() {
            writer.write_all(output::format_no_move().as_bytes())?;
            writer.flush()?;
            state = Some(turn);
            continue;
        }

        let heatmap = strategy::generate_heatmap(&turn.grid, turn.opponent, turn.me);
        let chosen = strategy::choose_best_placement(&valid, &heatmap, &turn.piece);

        match chosen {
            Some(p) => writer.write_all(output::format_move(p).as_bytes())?,
            None     => writer.write_all(output::format_no_move().as_bytes())?,
        }
        writer.flush()?;

        state = Some(turn);
    }

    Ok(())
}
```

### Critical IO Behaviour

- `BufRead::read_line` blocks until newline — correct, wait for game_engine
- `writer.flush()` after every output — game_engine reads one line per turn
- Exit gracefully on EOF (reader.read_line returns `Ok(0)` → empty line)
- **No** custom timeouts on stdin — game_engine handles timeouts (default 10s, flag `-t`)
- Error messages go to **stderr**, placements go to **stdout**

---

## 6. `src/lib.rs` — Module Re-exports

```rust
pub mod types;
pub mod parser;
pub mod validator;
pub mod strategy;
pub mod output;
pub mod visualizer; // bonus
```

---

## 7. `Cargo.toml`

```toml
[package]
name = "filler"
version = "0.1.0"
edition = "2021"

[features]
e2e = []  # Opt-in flag for E2E tests containing execution binaries

[dependencies]
# Minimal dependencies. Stdlib-only if possible.
# Add `colored` crate only if building visualizer (bonus).

[dev-dependencies]
assert_cmd = "2"   # CLI process testing
predicates = "3"   # Command outcome assertions
tempfile = "3"     # Temporary directories for E2E file logs

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

---

## 8. Integration Tests — `tests/integration_tests.rs`

### Goal
Test the full pipeline: parse → validate → choose → format output.

### Test Helpers — `tests/common/mod.rs`

```rust
use std::io::Cursor;
use filler::types::Point;

/// Create a simulated stdin from a multiline string
pub fn mock_stdin(input: &str) -> impl std::io::BufRead {
    std::io::BufReader::new(Cursor::new(input.as_bytes().to_vec()))
}

/// Standard 5x5 test anfield as a string block
pub fn fixture_anfield_5x5() -> &'static str {
    "Anfield 5 5:\n    01234\n000 .@...\n001 .....\n\
     002 .....\n003 ...$.\n004 .....\n"
}
```

### Test Cases

#### IT-1: P1 single valid turn — places on own territory
**Input:**
```
$$$ exec p1 : [robots/bender]
Anfield 5 5:
    01234
000 .....
001 .@...
002 .....
003 ...$.
004 .....
Piece 1 1:
O
```
**Expected:** Output is `1 1\n` (the only cell with P1 territory).

#### IT-2: P2 single valid turn
Same grid, P2 input. Expected: output is `3 3\n`.

#### IT-3: No valid placement → fallback `0 0`
**Expected:** Output is `0 0\n` when piece cannot be placed legally.

#### IT-4: Multiple valid placements → picks closest to opponent
**Expected:** Deterministic selection closest to opponent's territory.

#### IT-5: Boundary rejection wired in pipeline
Rejects piece placements extending past bounds.

#### IT-6: Two consecutive turns (no player line re-sent)
Verifies state persists and player identity carries over between turns.

---

### 8.1. Benchmark Harness — `benches/turn_benchmark.rs`

To protect against performance regressions, we compile a release benchmark enforcing that decision processing on maximum dimensions (100x100 Anfield grid + 20x20 Piece) completes in **less than 500ms** (well within the game engine's 10s timeout).

```rust
// benches/turn_benchmark.rs
use std::time::Instant;
use filler::{parser, validator, strategy, types::Point};

fn main() {
    // 1. Construct simulated 100x100 grid with scattered Player 1 and Player 2 cells
    // 2. Construct 20x20 piece
    // 3. Measure time for full pipeline cycle
    let start = Instant::now();
    
    let valid = validator::find_valid_placements(&grid, &piece, me);
    let heatmap = strategy::generate_heatmap(&grid, opponent, me);
    let chosen = strategy::choose_best_placement(&valid, &heatmap, &piece);
    
    let duration = start.elapsed();
    println!("Decision time: {:?}", duration);
    assert!(duration.as_millis() < 500, "Performance regression: decision took longer than 500ms");
}
```

---

### 8.2. E2E Replay Validation Test — `tests/e2e.rs`

This test runs an actual game using the engine, records all generated moves, and replays each move against a validator to ensure 100% rule compliance (exactly 1 own overlap, 0 opponent overlap, in-bounds). Gated behind the `e2e` cargo feature.

```rust
// tests/e2e.rs
#[cfg(feature = "e2e")]
#[test]
fn test_replay_move_correctness() {
    // 1. Run game_engine binary using assert_cmd with student bot vs bender
    // 2. Parse game replay file / stdout
    // 3. For each turn:
    //    - Assert the placement has exactly 1 own cell overlap
    //    - Assert 0 opponent cell overlaps
    //    - Assert is_in_bounds is true
}
```

---

### 8.3. Multi-Turn Territory Growth Test — `tests/multi_turn.rs`

Simulates a mini-game over 3 turns to verify that our scoring logic successfully drives expansion and that the count of own territory cells grows monotonically.

```rust
// tests/multi_turn.rs
#[test]
fn test_territory_monotonically_increases() {
    // 1. Initialize grid with 1 player cell
    // 2. Turn 1: Find best placement, apply placement to grid, assert own_count == 2
    // 3. Turn 2: Find best placement, apply placement to grid, assert own_count == 3
    // 4. Turn 3: Find best placement, assert own_count == 4
}
```

---

### 8.4. Deterministic Tie-Breaker Test — `tests/strategy_tests.rs`

Verifies that if multiple placements yield identical heatmap scores, the scorer tie-breaks deterministically by picking the lower row first, and if the rows are equal, the lower column.

```rust
// tests/strategy_tests.rs
#[test]
fn test_tiebreak_by_row_then_col() {
    let placements = vec![
        Point { row: 3, col: 2 },
        Point { row: 2, col: 5 },
        Point { row: 2, col: 1 },
    ];
    // Assuming all placements have equal score...
    // Expected: picks Point { row: 2, col: 1 } (lowest row, then lowest col)
    let best = strategy::choose_best_placement(&placements, &heatmap, &piece).unwrap();
    assert_eq!(best, Point { row: 2, col: 1 });
}
```

---

## 9. E2E Audit Test Suite — `e2e/run_audit_suite.sh`

Directly answers every question in `requirements/audit.md`.

```bash
#!/bin/bash
set -euo pipefail

STUDENT="./solution/filler"
GAME_ENGINE="./game_engine"
MAPS_DIR="./maps"
ROBOTS_DIR="./robots"
PASS=0
FAIL=0

green() { echo -e "\033[32m[PASS]\033[0m $1"; ((PASS++)); }
red()   { echo -e "\033[31m[FAIL]\033[0m $1"; ((FAIL++)); }
info()  { echo -e "\033[34m[INFO]\033[0m $1"; }

# -------------------------------------------------------------------
# Audit Q1: Docker image and container creation
# -------------------------------------------------------------------
info "=== AUDIT: Functional ==="
info "Q1: Confirm image and container creation"

if docker image inspect filler > /dev/null 2>&1; then
    green "Docker image 'filler' exists"
else
    red "Docker image 'filler' NOT found"
fi

# -------------------------------------------------------------------
# Audit Q2: Project runs correctly
# -------------------------------------------------------------------
info "Q2: Confirm project runs without crash/timeout"

run_game() {
    local map="$1" p1="$2" p2="$3"
    timeout 30 "$GAME_ENGINE" -f "$map" -p1 "$p1" -p2 "$p2" -q 2>&1 || true
}

# Quick smoke test: student vs bender
OUTPUT=$(run_game "$MAPS_DIR/map01" "$STUDENT" "$ROBOTS_DIR/bender")
if echo "$OUTPUT" | grep -qi "error\|timeout\|segfault\|panic"; then
    red "Student crashed or timed out vs bender"
else
    green "Student runs without errors vs bender"
fi

# -------------------------------------------------------------------
# Audit Q3: Pieces placed with correct 1-cell overlap
# -------------------------------------------------------------------
info "Q3: 1-cell overlap rule verified by unit tests"
info "  (validator_tests.rs covers: 0 overlap, 1 overlap, 2+ overlap, opponent overlap)"

# -------------------------------------------------------------------
# Audit Q4-6: Win-rate checks
# -------------------------------------------------------------------

check_winrate() {
    local map="$1"
    local p1="$2"
    local p2="$3"
    local label="$4"
    local needed="$5"

    info "Win-rate: $label (need $needed/10)"
    local wins=0

    # Run 5 as p1, 5 as p2
    for i in $(seq 1 5); do
        local out
        out=$(run_game "$map" "$p1" "$p2")
        if echo "$out" | grep -q "Player 1 won"; then
            ((wins++))
        fi
    done

    for i in $(seq 1 5); do
        local out
        out=$(run_game "$map" "$p2" "$p1")
        if echo "$out" | grep -q "Player 2 won"; then
            ((wins++))
        fi
    done

    if [ "$wins" -ge "$needed" ]; then
        green "$label: $wins/10 wins"
    else
        red "$label: $wins/10 wins (need $needed)"
    fi
}

check_winrate "$MAPS_DIR/map00" "$STUDENT" "$ROBOTS_DIR/wall_e" "vs wall_e on map00" 8
check_winrate "$MAPS_DIR/map01" "$STUDENT" "$ROBOTS_DIR/h2_d2"  "vs h2_d2 on map01"  8
check_winrate "$MAPS_DIR/map02" "$STUDENT" "$ROBOTS_DIR/bender" "vs bender on map02"  8

# -------------------------------------------------------------------
# Audit Q7-10: Unit tests
# -------------------------------------------------------------------
info "=== AUDIT: Unit Tests ==="
info "Running cargo test..."
if cargo test --lib 2>&1; then
    green "All unit tests pass"
else
    red "Unit tests failed"
fi

info "Checking specific test categories..."
if cargo test --lib parser::tests 2>&1 | grep -q "test result: ok"; then
    green "Input Parsing tests exist and pass"
else
    red "Input Parsing tests missing or failing"
fi

if cargo test --lib validator::tests 2>&1 | grep -q "test result: ok"; then
    green "Placement Validation tests exist and pass"
else
    red "Placement Validation tests missing or failing"
fi

if cargo test --lib -- is_in_bounds 2>&1 | grep -q "test result: ok"; then
    green "Boundary Detection tests exist and pass"
else
    red "Boundary Detection tests missing or failing"
fi

# -------------------------------------------------------------------
# Bonus: Visualizer
# -------------------------------------------------------------------
info "=== AUDIT: Bonus ==="
if cargo test --lib visualizer:: 2>&1 | grep -q "test result: ok"; then
    green "Visualizer tests pass"
else
    info "Visualizer not found (bonus, not required)"
fi

# Bonus: Beat terminator
check_winrate "$MAPS_DIR/map01" "$STUDENT" "$ROBOTS_DIR/terminator" "vs terminator" 8

# -------------------------------------------------------------------
# Summary
# -------------------------------------------------------------------
echo ""
echo "========================="
echo "Audit Suite Complete"
echo "Passed: $PASS"
echo "Failed: $FAIL"
echo "========================="
```

---

## 10. Win-Rate Parser — `e2e/assert_winrate.rs`

Standalone binary for parsing game engine output and counting wins.

```rust
// e2e/assert_winrate.rs
// Usage: cargo run --bin assert_winrate -- --map maps/map01 --p1 solution/filler --p2 robots/bender --runs 5

use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // Parse --map, --p1, --p2, --runs flags
    // Run N games, count student wins
    // Print WINS=<n>
}
```

**Winner detection pattern:** parse game_engine stdout for:
```
== Player 1 won! ==
== Player 2 won! ==
```
Adjust based on actual engine output when provided.

---

## 11. Dockerfile & Exclusions

### `.dockerignore`
Exclude build artifacts and repository history from the Docker build context to accelerate builds and reduce image sizes.

```
target/
.git/
```

### Dockerfile (Base: Debian Bookworm Slim)
```dockerfile
# Stage 1: Build
FROM rust:1.78-slim-bookworm AS builder
WORKDIR /filler
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim
WORKDIR /filler
COPY --from=builder /filler/target/release/filler /filler/solution/filler
# game_engine, maps, robots are provided separately in the docker_image folder
```

### Alternative: Alpine-Based Docker (Minimal Image Size Variant)
```dockerfile
# Stage 1: Build (requires musl targets)
FROM rust:1.78-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /filler
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --target x86_64-unknown-linux-musl --release

# Stage 2: Runtime
FROM alpine:3.19
WORKDIR /filler
COPY --from=builder /filler/target/x86_64-unknown-linux-musl/release/filler /filler/solution/filler
```

> [!WARNING]
> **Alpine / musl Compatibility Trade-off:** While Alpine reduces the final image size to ~50MB (vs ~150MB for Debian), it dynamically links compiled binaries against `musl` instead of `glibc`. If the provided `game_engine` binary is statically or dynamically linked to `glibc`, running it under Alpine will fail. Keep Debian as the default unless `game_engine` is confirmed `musl`-compatible.

Build command:
```bash
docker build -t filler .
```

Run:
```bash
docker run -v "$(pwd)/solution":/filler/solution -it filler
```

Inside container:
```bash
cd /filler
./game_engine -f maps/map01 -p1 robots/bender -p2 solution/filler
```

---

## 12. Build & Test Commands

```bash
# Build (debug)
cargo build

# Build (release)
cargo build --release

# Run all unit tests
cargo test --lib

# Run all tests (unit + integration, excluding e2e)
cargo test

# Run E2E replay validation tests (requires game_engine binary and e2e feature flag)
cargo test --features e2e --test e2e

# Run performance benchmarks
cargo bench

# Run specific test module
cargo test --lib parser
cargo test --lib validator

# Run with test output
cargo test -- --nocapture

# Watch mode (requires cargo-watch)
cargo watch -x test

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt -- --check

# Build for E2E
cargo build --release -p filler
cp target/release/filler solution/filler

# Run E2E suite (inside Docker container)
bash e2e/run_audit_suite.sh
```

---

## 13. Full TDD Execution Order (Day-by-Day)

### Day 1: Project Setup + Types + Output + Parser

1. `cargo init filler`
2. Create `src/types.rs` with data structures using `i32` coordinates
3. Create `src/output.rs` and write tests for formatting moves (X Y) and fallbacks
4. Create `src/lib.rs` with module declarations
5. Write `tests/parser_tests.rs` — all parser test cases (RED)
6. Write `tests/common/mod.rs` — shared test fixtures
7. Implement `src/parser.rs` (GREEN)
8. Create `testdata/` fixture files

**Checkpoint:** `cargo test --lib parser` — all green

### Day 2: Validator + Strategy

1. Write `tests/validator_tests.rs` — including negative-offset placements (RED)
2. Implement `src/validator.rs` (GREEN)
3. Write `tests/strategy_tests.rs` — heatmap with own-territory `-1` and deterministic tiebreaks (RED)
4. Implement `src/strategy.rs` (GREEN)

**Checkpoint:** `cargo test --lib` — all green

### Day 3: Main Loop + Integration + Docker + Benchmark

1. Write `tests/integration_tests.rs` — full pipeline tests (RED)
2. Implement `src/main.rs` with centralized "never panic" rules and `BufWriter` (GREEN)
3. Write `benches/turn_benchmark.rs` to assert decision cycle completes in < 500ms
4. Write `Dockerfile` and `.dockerignore`
5. Build release binary: `cargo build --release`
6. Test inside Docker container

**Checkpoint:** Integration tests and benchmarks pass. Binary works inside Docker.

### Day 4: E2E Replay Validation + Multi-Turn Testing + Strategy Tuning

1. Write `tests/e2e.rs` to validate all moves in a live game replay are legal
2. Write `tests/multi_turn.rs` to ensure territory grows monotonically
3. Write `e2e/run_audit_suite.sh`
4. Write `e2e/assert_winrate.rs`
5. Run E2E against provided robots
6. Tune strategy heatmap (edge weighting, blocking, center start) if win rates are under 80%

**Checkpoint:** E2E replay and territory tests pass. Win-rates exceed 80%.

### Day 5: Visualizer (Bonus) + Polish

1. Add `colored` crate to `Cargo.toml` (for terminal colors)
2. Implement `src/visualizer.rs`
3. Write `tests/visualizer_tests.rs`
4. Run `cargo clippy` and `cargo fmt`
5. Final `cargo test` — all green
6. Final `bash e2e/run_audit_suite.sh` — all audits pass

---

## 14. Edge Cases Checklist

| Edge Case | Test Location | Expected Behavior |
|-----------|--------------|-------------------|
| Player line malformed | `parser_tests.rs` | `Err` returned, main outputs `0 0` and continues |
| Anfield dimensions = 0 / negative | `parser_tests.rs` | `Err` |
| Negative-offset placement | `validator_tests.rs` | Pieces with padding successfully placed at negative anchor offsets |
| Piece with zero blocks (all dots) | `validator_tests.rs` | No valid placements, output `0 0` |
| Grid entirely filled | `validator_tests.rs` | No valid placements |
| Piece larger than grid in both dimensions | `validator_tests.rs` | All placements rejected (bounds) |
| stdin EOF mid-turn | `main.rs` | Exit gracefully with `Ok(())` |
| Opponent chars: both `s` and `$` | `validator_tests.rs` | Both treated as opponent |
| Our chars: both `a` and `@` | `validator_tests.rs` | Both count as own overlap |
| Placement at (0,0) with piece that fits | `validator_tests.rs` | Valid |
| Placement at (MaxRow-1, MaxCol-1) | `validator_tests.rs` | Valid if piece doesn't overflow |
| Very large grid (100x100) | `benches/turn_benchmark.rs` | Decision completes in < 500ms |
| Player line includes newline / trailing space | `parser_tests.rs` | Trimmed correctly |
| Piece header uses `*` instead of `O` | `parser_tests.rs` | Both parsed as filled blocks |
| Game engine sends empty line between sections | `parser_tests.rs` | `read_line` handles blank lines gracefully |

---

## 15. Audit Question → Test Mapping

| Audit Question | Covered By |
|---------------|------------|
| Image and container creation | `e2e/run_audit_suite.sh` Q1 |
| Project runs correctly (no crash/timeout) | `e2e/run_audit_suite.sh` Q2 |
| Pieces placed with 1-cell overlap | `validator_tests.rs` — `is_valid_placement` and E2E replay checks |
| vs wall_e on map00, win 4/5 | `e2e/run_audit_suite.sh` — `check_winrate map00 wall_e` |
| vs h2_d2 on map01, win 4/5 | `e2e/run_audit_suite.sh` — `check_winrate map01 h2_d2` |
| vs bender on map02, win 4/5 | `e2e/run_audit_suite.sh` — `check_winrate map02 bender` |
| All unit tests pass | `cargo test --lib` in audit suite |
| Input Parsing tests exist | `parser_tests.rs` |
| Placement Validation tests exist | `validator_tests.rs` |
| Boundary Detection tests exist | `validator_tests.rs` — `is_in_bounds` tests |
| Performance target (under 10s timeout) | `benches/turn_benchmark.rs` (asserts < 500ms) |
| Good practices | `cargo clippy`, `cargo fmt`, module structure, no unsafe code |
| Test file exists | `tests/` directory with per-module test files |
| Tests check each possible case | Table-driven tests in every test module |
| Visualizer (bonus) | `src/visualizer.rs` + `tests/visualizer_tests.rs` |
| Beat terminator 4/5 (bonus) | `e2e/run_audit_suite.sh` — terminator check |

---

## 16. Visualizer Specification (Bonus)

### Goal
Read game replay and render each turn step-by-step with colorized terminal output.

### Implementation sketch

```rust
// src/visualizer.rs

use std::io::{self, BufRead, Write};
use std::time::Duration;
use crate::types::{Grid, Cell};

/// One frame = one game turn's grid state
pub struct Frame {
    pub grid: Grid,
    pub turn: usize,
}

/// Parse a game replay from a reader
pub fn read_replay<R: BufRead>(reader: &mut R) -> Result<Vec<Frame>, String> {
    // Parse consecutive anfield blocks
}

/// Render a single frame with ANSI colors
pub fn render_frame(frame: &Frame, writer: &mut impl Write) -> io::Result<()> {
    // Player 1: red on black
    // Player 2: blue on black
    // Empty: dim white dot
    // Clear screen: \x1b[2J\x1b[H
    // Print grid row by row
}

/// Play all frames with delay
pub fn play(frames: &[Frame], delay: Duration) {
    for frame in frames {
        render_frame(frame, &mut io::stdout()).unwrap();
        std::thread::sleep(delay);
    }
}
```

---

## 17. Summary

Plan covers:
- **6 Rust source modules**: types, parser, validator, strategy, output, visualizer
- **35+ unit test cases** across parser, validator, strategy, output
- **9 integration & E2E test cases** for full pipeline, multi-turn growth, and correctness
- **E2E audit script** mapping every audit question to an automated check
- **Performance benchmark harness** for regression prevention
- **15 edge cases** with expected behavior and test location
- **Day-by-day execution order** for a junior engineer
- **Zero external crate dependencies** (stdlib only for core; `colored` optional for visualizer)

Follow TDD cycle: write test (RED) → minimal implementation (GREEN) → refactor → repeat.
Every audit question has a corresponding test, benchmark, or script check.
