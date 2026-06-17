# Proposed Enrichments for `deeppro.md`

Based on a comparison of `docs/plan/deeppro.md` with `docs/plan/glm.md` and `docs/plan/gem.md`, the following enrichments are proposed to enhance the robustness, testability, and audit preparedness of the DeepPro implementation plan.

---

## 1. Negative Coordinate Placement Support (From `glm.md`)

* **Context**: The game engine allows a player to place a piece at negative coordinates `(X, Y)` relative to the grid top-left, provided that the actual *filled cells* of the piece remain inside the grid boundaries. This is common when a piece has empty padding (`.`) along its top or left edges.
* **Problem**: `deeppro.md` specifies `Point { row: usize, col: usize }`, which prevents negative coordinates and will result in either an underflow panic or a failure to find valid edge placements.
* **Proposed Change**: 
  * Change `Point` structure coordinates to `isize` or `i32` when representing placements.
  * Adjust validation search space to check from `-(piece.rows as isize)` to `grid.rows as isize`.

---

## 2. Explicit Audit Targets in E2E Suite (From `gem.md`)

* **Context**: The audit guidelines require demonstrating successful execution and a winning rate against specific reference bots.
* **Problem**: `deeppro.md` lists `run_audit_suite.sh` but leaves the scenarios and win-rate thresholds unspecified.
* **Proposed Change**: Add explicit testing scenarios and win-rate assertions to the E2E verification section:
  * **Scenario 1**: `maps/map00` vs `robots/wall_e` (Must win $\ge$ 4/5 matches)
  * **Scenario 2**: `maps/map01` vs `robots/h2_d2` (Must win $\ge$ 4/5 matches)
  * **Scenario 3**: `maps/map02` vs `robots/bender` (Must win $\ge$ 4/5 matches)

---

## 3. Turn Execution Performance Benchmarks (From `glm.md`)

* **Context**: The game engine terminates bots that exceed the turn execution timeout (usually 1 or 10 seconds depending on settings).
* **Problem**: A naive BFS heatmap implementation might suffer from performance issues on larger grid sizes.
* **Proposed Change**:
  * Introduce `benches/turn_benchmark.rs`.
  * Set a target of `< 500ms` per turn for parsing and computing the strategy on a standard 100x100 grid.

---

## 4. CLI Testing Utilities (From `glm.md`)

* **Context**: Verification of integration behavior requires feeding mocked inputs into the main binary's standard input.
* **Problem**: Testing the binary execution directly using standard library tools can be verbose.
* **Proposed Change**: Add CLI assertion helpers to `Cargo.toml` dev-dependencies:
  ```toml
  [dev-dependencies]
  assert_cmd = "2"
  predicates = "3"
  ```
  This enables direct execution testing of the compiled `filler` binary inside unit/integration tests.

---

## 5. Deterministic Tie-Breaker Logic (From `glm.md`)

* **Context**: When evaluating placements, multiple positions can yield the same optimal score.
* **Problem**: Relying implicitly on validation loop ordering makes testing and behavior fragile across different refactorings.
* **Proposed Change**: Define an explicit tie-breaking rule:
  * If scores are identical, prefer the placement with the lowest row (top-most), followed by the lowest column (left-most).
