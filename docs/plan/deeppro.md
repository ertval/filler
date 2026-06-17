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
│
├── src/
│   ├── main.rs               # Entry point: stdin → parse → decide → stdout loop
│   ├── lib.rs                # Re-export all modules for testing
│   ├── types.rs              # Shared data structures & constants
│   ├── parser.rs             # Parse stdin into GameState
│   ├── validator.rs          # Placement legality checks
│   ├── strategy.rs           # Heatmap + placement scoring
│   └── visualizer.rs         # Bonus: terminal visualizer
│
├── tests/
│   ├── common/mod.rs         # Shared test helpers & fixtures
│   ├── parser_tests.rs       # Unit tests for parser
│   ├── validator_tests.rs    # Unit tests for validator
│   ├── strategy_tests.rs     # Unit tests for strategy
│   ├── integration_tests.rs  # Full pipeline integration tests
│   └── visualizer_tests.rs   # Bonus
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

/// A point in the grid (row, col) — row is Y, col is X in output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub row: usize,
    pub col: usize,
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

Internally store `Point { row, col }`. Convert on output:
```rust
println!("{} {}", point.col, point.row);
```

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
| Negative check via unsigned: target=(0,0) piece with block at (0,1) on 5x5 grid | 5x5 | 1x2 horizontal | (0,4) | `false` |
| Fits exactly at boundary | 10x10 | 2x2 | (8,8) | `true` |
| 1x1 piece anywhere in bounds | 10x10 | 1x1 | (5,5) | `true` |

**Implementation sketch:**
```rust
pub fn is_in_bounds(grid: &Grid, piece: &Piece, target: Point) -> bool {
    for &(dr, dc) in &piece.blocks {
        let r = target.row + dr;
        let c = target.col + dc;
        if r >= grid.rows || c >= grid.cols {
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
| 2 | Two own overlaps (piece 1x2 `[OO]` on two own cells) | 1x2 horiz `[OO]` | (0,0) | `false` — but need two own cells! Grid has only one own cell at (0,0). Let's make grid: `@ @ . . .` then target=(0,0) with 1x2 horiz → covers (0,0) own + (0,1) own = false |
| 3 | One own + one opponent overlap | 1x2 horiz `[OO]` | Place so one cell hits own, one hits opponent | `false` |
| 4 | Exactly 1 own, 0 opponent — VALID | 1x2 horiz `[OO]` at (0,0) on grid `@ . . . .` | (0,0) | `true` — covers (0,0) own + (0,1) empty |
| 5 | Exactly 1 own (via `a` recent char) | Same grid but own cell is `a` | 1x2 `[OO]` at (0,0) | `true` |
| 6 | Opponent cell is `$` (old) | Opponent at (3,3) is `$`; piece covers it | 1x1 `[O]` at (3,3) | `false` — opponent overlap |
| 7 | Opponent cell is `s` (recent) | Same, opponent char is `s` | 1x1 `[O]` at (3,3) | `false` |
| 8 | Piece covers both own AND empty cells (multiple new cells) | 2x2 `[OO/OO]` at (0,0) | (0,0) | `true` — 1 own overlap + 3 empty |
| 9 | Zero own + zero opponent (all empty) | 1x1 `[O]` at (2,2) all empty grid | (2,2) | `false` — must overlap exactly 1 own cell |
| 10 | Piece larger than grid — should be caught by `is_in_bounds` first | 6x6 piece on 5x5 | (0,0) | `false` |

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
        let r = target.row + dr;
        let c = target.col + dc;
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
| No valid placement exists | Returns empty `Vec` |
| Piece covers entire grid | Returns empty `Vec` (or 1 if exactly 1 own cell and grid fits) |

**Implementation sketch:**
```rust
pub fn find_valid_placements(
    grid: &Grid,
    piece: &Piece,
    player: Player,
) -> Vec<Point> {
    let mut results = Vec::new();
    for row in 0..grid.rows {
        for col in 0..grid.cols {
            let target = Point { row, col };
            if is_valid_placement(grid, piece, target, player) {
                results.push(target);
            }
        }
    }
    results
}
```

---

## 4. Module C: Strategy Algorithm — `src/strategy.rs`

### Goal
Select the best placement from all valid placements.

### Strategy: Aggressive Heatmap (Distance BFS)

Create a distance heatmap from the opponent's territory. Choose placements closest to the opponent to cut off their expansion space.

#### C1: Generate Heatmap (BFS from opponent territory)

**Function signature:**
```rust
pub fn generate_heatmap(grid: &Grid, opponent: Player) -> Vec<Vec<i32>>
```

**Test cases (`tests/strategy_tests.rs`):**

| Scenario | Expected |
|----------|----------|
| 5x5 grid, opponent at (0,0), rest empty | heatmap[0][0]=0, neighbors=1, diagonal=2... |
| 5x5 grid, opponent at (2,2) AND (2,3) | Both at 0, BFS from both simultaneously |
| Grid with walls/edges | BFS respects grid boundaries (doesn't wrap) |
| All cells are territory (no empty) | Sentinels stay as `i32::MAX` for non-opponent cells that are our territory |
| 3x3 grid, opponent at (1,1) | Center=0, edges=1, corners=2 |

**Implementation sketch:**
```rust
use std::collections::VecDeque;

pub fn generate_heatmap(grid: &Grid, opponent: Player) -> Vec<Vec<i32>> {
    let rows = grid.rows;
    let cols = grid.cols;
    let mut heatmap = vec![vec![i32::MAX; cols]; rows];
    let mut queue = VecDeque::new();

    for r in 0..rows {
        for c in 0..cols {
            if grid.data[r][c].belongs_to(opponent) {
                heatmap[r][c] = 0;
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
| Placement with 2x2 piece covering cells with distances 2,2,3,3 | Score = 10 |
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
        let r = target.row + dr;
        let c = target.col + dc;
        let h = heatmap[r][c];
        if h == i32::MAX {
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
| Equal scores | Chooses first found (deterministic) |
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
        }
    }

    Some(best)
}
```

#### C4 (Bonus): Advanced Territory Control

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

## 5. Main Loop — `src/main.rs`

```rust
use std::io::{self, BufRead, Write};
use filler::parser;
use filler::validator;
use filler::strategy;
use filler::types::GameState;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());
    let stdout = io::stdout();
    let mut writer = stdout.lock();

    let mut state: Option<GameState> = None;

    loop {
        let turn = match parser::parse_turn(&mut reader, state.clone()) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Parse error: {e}");
                break;
            }
        };

        let valid = validator::find_valid_placements(
            &turn.grid,
            &turn.piece,
            turn.me,
        );

        if valid.is_empty() {
            writeln!(writer, "0 0")?;
            writer.flush()?;
            state = Some(turn);
            continue;
        }

        let heatmap = strategy::generate_heatmap(&turn.grid, turn.opponent);
        let chosen = strategy::choose_best_placement(&valid, &heatmap, &turn.piece);

        match chosen {
            Some(p) => writeln!(writer, "{} {}", p.col, p.row)?,
            None     => writeln!(writer, "0 0")?,
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
pub mod visualizer; // bonus
```

---

## 7. `Cargo.toml`

```toml
[package]
name = "filler"
version = "0.1.0"
edition = "2021"

[dependencies]
# Minimal dependencies. Stdlib-only if possible.
# Add `colored` crate only if building visualizer (bonus).

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

**Expected:** Output is `1 1\n` (col=1, row=1 — the only cell with P1 territory, 1x1 piece must overlap it exactly).

#### IT-2: P2 single valid turn

Same grid, P2 input. Expected: output is `3 3\n`.

#### IT-3: No valid placement → fallback `0 0`

**Input:**
```
$$$ exec p1 : [robots/bender]
Anfield 3 3:
    012
000 ...
001 .@.
002 ...
Piece 3 3:
OOO
OOO
OOO
```

**Expected:** Output is `0 0\n` (piece too large, covers opponent area or itself with too many overlaps).

#### IT-4: Multiple valid placements → picks closest to opponent

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

**Expected:** P1 at (1,1), P2 at (3,3). Valid 1x1 placements: (1,1) only (must overlap own territory exactly once). Output `1 1\n`.

Wait — with a 1x1 piece on P1 at (1,1), the only valid placement is exactly on top of (1,1) because you need exactly 1 own overlap and 0 opponent. That's correct per the rules — but it means you're just placing on top of your own cell without expanding. Is that useful? No. So a 1x1 piece on existing territory is technically legal but strategically useless.

Better test: use a 1x2 piece so there are multiple valid placements.

**Revised IT-4 input:**
```
$$$ exec p1 : [robots/bender]
Anfield 5 5:
    01234
000 .....
001 .@...
002 .....
003 ...$.
004 .....
Piece 1 2:
O
O
```

Piece is vertical: 2 rows, 1 column.
P1 at (1,1). Valid placements:
- target=(0,1): covers (0,1) empty + (1,1) own → valid → output `1 0\n`
- target=(1,1): covers (1,1) own + (2,1) empty → valid → output `1 1\n`
Both are 1 distance from P1 territory. The one closer to opponent at (3,3): (2,1) vs (1,1). Manhattan from (3,3): (2,1) → |2-3|+|1-3|=3, (1,1) → |3-1|+|3-1|=4. Not a huge difference but heatmap will rank them. Expected: chooses (2,1) → output `1 1\n`... wait output is `col row\n` = `X Y\n`. (row=2, col=1) → `1 2\n`. (row=1, col=1) → `1 1\n`. The one closer to opponent is `1 2\n`.

Actually let's just verify the strategy picks one deterministically. The test should verify that the output is one of the valid placements.

#### IT-5: Boundary rejection wired in pipeline

Piece placed partially outside grid. Verifies that `is_in_bounds` is called and rejects.

#### IT-6: Two consecutive turns (no player line re-sent)

First turn includes player line. Second turn jumps straight to anfield header. Verifies state carries over.

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

## 11. Dockerfile

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

# Run all tests (unit + integration)
cargo test

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

### Day 1: Project Setup + Types + Parser

1. `cargo init filler`
2. Create `src/types.rs` with all data structures — no logic, just types
3. Create `src/lib.rs` with module declarations
4. Write `tests/parser_tests.rs` — all parser test cases (RED)
5. Write `tests/common/mod.rs` — shared test fixtures
6. Implement `src/parser.rs` (GREEN)
7. Create `testdata/` fixture files

**Checkpoint:** `cargo test --lib parser` — all green

### Day 2: Validator + Strategy

1. Write `tests/validator_tests.rs` — boundary + overlap + find_all tests (RED)
2. Implement `src/validator.rs` (GREEN)
3. Write `tests/strategy_tests.rs` — heatmap + scoring + choose tests (RED)
4. Implement `src/strategy.rs` (GREEN)

**Checkpoint:** `cargo test --lib` — all green

### Day 3: Main Loop + Integration + Docker

1. Write `tests/integration_tests.rs` — full pipeline tests (RED)
2. Implement `src/main.rs` (GREEN)
3. Write `Dockerfile` — multi-stage build
4. Build release binary: `cargo build --release`
5. Test inside Docker container

**Checkpoint:** Integration tests pass. Binary works inside Docker.

### Day 4: E2E Script + Strategy Tuning

1. Write `e2e/run_audit_suite.sh`
2. Write `e2e/assert_winrate.rs`
3. Run E2E against provided robots
4. Tune heatmap if win rates below 80%
5. Add advanced dual-heatmap strategy if needed

**Checkpoint:** E2E script passes all audit requirements.

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
| Player line malformed | `parser_tests.rs` | `Err` returned, main exits gracefully |
| Anfield dimensions = 0 | `parser_tests.rs` | `Err` |
| Piece with zero blocks (all dots) | `validator_tests.rs` | No valid placements, output `0 0` |
| Grid entirely filled | `validator_tests.rs` | No valid placements |
| Piece larger than grid in both dimensions | `validator_tests.rs` | All placements rejected (bounds) |
| stdin EOF mid-turn | `main.rs` | Exit gracefully with `Ok(())` |
| Opponent chars: both `s` and `$` | `validator_tests.rs` | Both treated as opponent |
| Our chars: both `a` and `@` | `validator_tests.rs` | Both count as own overlap |
| Placement at (0,0) with piece that fits | `validator_tests.rs` | Valid |
| Placement at (MaxRow-1, MaxCol-1) | `validator_tests.rs` | Valid if piece doesn't overflow |
| Very large grid (100x100) | `strategy_tests.rs` | Algorithm terminates < 1s |
| Player line includes newline / trailing space | `parser_tests.rs` | Trimmed correctly |
| Piece header uses `*` instead of `O` | `parser_tests.rs` | Both parsed as filled blocks |
| Game engine sends empty line between sections | `parser_tests.rs` | `read_line` handles blank lines gracefully |

---

## 15. Audit Question → Test Mapping

| Audit Question | Covered By |
|---------------|------------|
| Image and container creation | `e2e/run_audit_suite.sh` Q1 |
| Project runs correctly (no crash/timeout) | `e2e/run_audit_suite.sh` Q2 |
| Pieces placed with 1-cell overlap | `validator_tests.rs` — `is_valid_placement` tests all overlap combos |
| vs wall_e on map00, win 4/5 | `e2e/run_audit_suite.sh` — `check_winrate map00 wall_e` |
| vs h2_d2 on map01, win 4/5 | `e2e/run_audit_suite.sh` — `check_winrate map01 h2_d2` |
| vs bender on map02, win 4/5 | `e2e/run_audit_suite.sh` — `check_winrate map02 bender` |
| All unit tests pass | `cargo test --lib` in audit suite |
| Input Parsing tests exist | `parser_tests.rs` |
| Placement Validation tests exist | `validator_tests.rs` |
| Boundary Detection tests exist | `validator_tests.rs` — `is_in_bounds` tests |
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
- **5 Rust source modules**: types, parser, validator, strategy, visualizer
- **30+ unit test cases** across parser, validator, strategy
- **6 integration test cases** for full pipeline
- **E2E audit script** mapping every audit question to an automated check
- **14 edge cases** with expected behavior and test location
- **Day-by-day execution order** for a junior engineer
- **Zero external crate dependencies** (stdlib only for core; `colored` optional for visualizer)

Follow TDD cycle: write test (RED) → minimal implementation (GREEN) → refactor → repeat.
Every audit question has a corresponding test or script check.
