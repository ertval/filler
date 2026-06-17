# Filler Implementation Plan: TDD Approach

This document outlines a highly detailed, Test-Driven Development (TDD) plan for the `filler` project. It is structured so a junior software engineer can implement it sequentially in Rust.

---

## 1. Core Architecture & Loop
The program must run continuously, reading from `stdin` and writing to `stdout`.
**State Data Structures:**
- `Player`: Stores our character (`@` or `$`) and the opponent's character.
- `Grid`: 2D Vector (`Vec<Vec<char>>`) representing the Anfield dimensions and cells.
- `Piece`: 2D Vector (`Vec<Vec<char>>`) representing the current game piece.
- `Heatmap`: 2D Vector of integers (`Vec<Vec<i32>>`) matching the Grid, used for strategy.

---

## 2. TDD Phases & Module Breakdown

### Module A: Parsing Engine
**Goal:** Parse incoming data from the Game Engine via `stdin`.

**Step 1: Player Identity Parser**
- **Unit Test:** Feed `$$$ exec p1 : [robots/bender]\n`. Assert that our player is assigned `p1` (chars `@` and `a`) and the opponent is `p2` (chars `$` and `s`).
- **Implementation:** Read the very first line from stdin and extract the player number.

**Step 2: Anfield Parser**
- **Unit Test:** Feed `Anfield 15 20:\n` followed by column headers and 15 rows of grid data. Assert that the resulting `Grid` struct has exactly 15 rows, 20 columns, and the cells correctly identify `.` (empty), `@/a` (Player 1), and `$/s` (Player 2).
- **Implementation:** Skip the column header line. Read exactly `Rows` number of lines. Store characters in a 2D Vector.

**Step 3: Piece Parser**
- **Unit Test:** Feed `Piece 4 1:\n.O.\n`. Assert the `Piece` struct stores the shape coordinates relative to the piece's top-left corner.
- **Implementation:** Parse dimensions, read the shape, and store coordinates of the `O` or `*` characters.

### Module B: Placement Validator
**Goal:** Check if placing a piece at grid coordinate `(X, Y)` is legal.

**Step 1: Boundary Detection**
- **Unit Test:** Attempt to place a piece such that it extends past row `< 0`, row `>= MaxRows`, col `< 0`, or col `>= MaxCols`. Assert the function returns `false`.
- **Implementation:** Iterate over the piece's block coordinates and add them to `(X, Y)`. Return `false` if any sum is out of bounds.

**Step 2: Collision & Overlap Rules**
- **Unit Test:**
  - Case 1: 0 overlaps with own territory -> Assert `false`.
  - Case 2: 2 or more overlaps with own territory -> Assert `false`.
  - Case 3: 1 own overlap AND 1 opponent overlap -> Assert `false`.
  - Case 4: Exactly 1 own overlap AND 0 opponent overlaps -> Assert `true`.
- **Implementation:** For a given `(X, Y)`, loop over piece blocks. Check the grid cell underneath. Count own block matches and opponent block matches. Return `ownCount == 1 && oppCount == 0`.

### Module C: Strategy Algorithm (Heatmap/BFS)
**Goal:** Determine the *best* legal placement. The most aggressive strategy is to rush the opponent and trap them.

**Step 1: Heatmap Generation**
- **Unit Test:** Provide a simple 5x5 grid with the opponent at `(0,0)`. Assert that the heatmap assigns `0` to `(0,0)`, `1` to `(0,1)` and `(1,0)`, `2` to `(0,2)`, `(1,1)`, `(2,0)`, etc.
- **Implementation:** Create a 2D Vector of integers initialized to `-1` or a high maximum. Set all opponent cells to `0`. Push them to a queue (e.g., `VecDeque`). Perform Breadth-First Search (BFS). For each neighbor, set its value to `current_value + 1` and push to the queue.

**Step 2: Placement Scorer**
- **Unit Test:** Given multiple valid placements, assert the scorer selects the one with the lowest combined heatmap score.
- **Implementation:** Loop through every cell in the Grid `(X, Y)`. If `is_valid(X, Y)` is true, sum the heatmap values underneath the piece blocks. Keep track of the `(X, Y)` with the minimum score.

### Module D: Output Generator
**Goal:** Print the chosen coordinate to `stdout`.

- **Unit Test:** Assert that a selected coordinate `(col=5, row=2)` prints exactly `5 2\n`. Assert that if no valid placement exists, it prints `0 0\n`.
- **Implementation:** Use `println!("{} {}", X, Y)`. Note: Game engines often use `X` as Column and `Y` as Row. Confirm this during testing. Ensure to flush stdout if necessary using `std::io::stdout().flush()`.

---

## 3. Integration Tests
**Goal:** Ensure the pipeline works end-to-end for a single turn.
- **Test:** Feed a complete stdin block (Player Info, Grid, Piece).
- **Assert:** The module parses everything, calculates the heatmap, validates placements, picks the lowest score, and prints the exact expected `X Y\n` string to stdout.

---

## 4. E2E & Audit Test Suite (The `audit.md` Checklist)
To satisfy the audit, we will write a custom bash script (`run_audit_suite.sh`) that automates the required tests:

1. **Docker Setup Check:** 
   - Script runs `docker build -t filler .` to verify successful container creation.
2. **Standard Functional Check:**
   - Script manually triggers `./game_engine -f maps/map01 -p1 robots/bender -p2 robots/terminator` to confirm the engine works.
3. **Automated Win-Rate Checks (Core Audit Questions):**
   - The script will run our robot 5 times as `p1` and 5 times as `p2` for each scenario below, counting the wins automatically.
   - **Scenario 1:** `maps/map00` vs `robots/wall_e` (Goal: >= 4 wins).
   - **Scenario 2:** `maps/map01` vs `robots/h2_d2` (Goal: >= 4 wins).
   - **Scenario 3:** `maps/map02` vs `robots/bender` (Goal: >= 4 wins).
4. **Visualizer (Bonus):**
   - Create a basic Rust binary that reads the game engine's `stdout` and prints a colored terminal UI for the grid.

---

## 5. Step-by-Step Execution for Junior Dev
1. **Initialize Project:** `cargo init --name filler` (or `cargo new filler` to create a new folder).
2. **TDD Loop - Parsing:** Write parsing tests in `src/parser.rs` using `#[cfg(test)]` -> Watch `cargo test` fail -> Write parsing logic -> Watch pass.
3. **TDD Loop - Validation:** Write validation tests in `src/validator.rs` -> Watch `cargo test` fail -> Write validation logic -> Watch pass.
4. **TDD Loop - Strategy:** Write heatmap tests in `src/strategy.rs` -> Watch `cargo test` fail -> Write strategy logic -> Watch pass.
5. **Main Integration:** Wire up `src/main.rs` to loop infinitely, reading from `std::io::stdin()` and writing to `std::io::stdout()`.
6. **E2E Automation:** Write `run_audit_suite.sh`. Execute to verify the 4/5 win rates required by `audit.md`.
7. **Dockerize:** Create `Dockerfile` with a Rust base image. Ensure `run_audit_suite.sh` passes inside Docker.
