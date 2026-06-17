# Cross-Plan Analysis: Enrichments for Deeppro from Gem & GLM Plans

Comparison of `docs/plan/deeppro.md` against `docs/plan/gem.md` and `docs/plan/glm.md`. Items below are what deeppro is missing or could improve.

---

## High-Impact Enrichments

### 1. Negative-Offset Search in `find_valid_placements` (Correctness)

**Source:** glm.md — `validator.rs`  
**Problem:** Deeppro iterates `0..grid.rows` / `0..grid.cols` only. Game engine allows piece anchor at negative offsets if piece blocks still land in-bounds (e.g. piece with empty top-left rows).  
**Fix:** Start iteration from `-(piece.height as i32)` and `-(piece.width as i32)`:

```rust
for row in -(piece.height as i32)..grid.rows as i32 {
    for col in -(piece.width as i32)..grid.cols as i32 {
        if is_valid(grid, piece, player, row, col) {
            results.push(Placement::new(row, col));
        }
    }
}
```

**Severity:** Critical — missing valid placements = lost games.

---

### 2. Strategy Tuning Guide (Win-Rate Insurance)

**Source:** glm.md — Section 13  
**Problem:** Deeppro mentions dual-heatmap bonus but gives no actionable roadmap if base strategy underperforms (< 80% win rate).  
**Fix:** Add concrete enhancement catalog:

| Enhancement | Description | Implementation Hint |
|---|---|---|
| Edge Weighting | Bonus for cells near grid edges; traps opponent | `edge_bonus(row, col, rows, cols)` returns -5 / -2 / 0 by dist-to-edge |
| Territory Connectivity | Prefer placements keeping own territory in one connected component | Penalty for creating isolated regions |
| Opponent Blocking | Extend along boundary when neighboring opponent | Score bonus for placements adjacent to opponent cells |
| Center Start | First move bias toward center + opponent direction | Extra weight on center-proximity for turn 1 |
| Piece Size Bonus | Prefer placements covering more empty cells | `score -= empty_cells_under_piece * 2` |

---

### 3. Benchmark Harness (Perf Regression Gate)

**Source:** glm.md — `benches/turn_benchmark.rs`  
**Problem:** Deeppro states "algorithm terminates < 1s" for 100x100 grids but never enforces it.  
**Fix:** Add `benches/turn_benchmark.rs`:

```rust
fn bench_single_turn() {
    // 100x100 grid + piece
    // Assert: completes in < 500ms
}
```

Gate in CI: `cargo bench` must pass under threshold.

---

### 4. E2E Feature Flag + Replay Validation Test

**Source:** glm.md — `tests/e2e.rs` with `#[cfg(feature = "e2e")]`  
**Problem:** Deeppro's E2E is bash-only. CI without `game_engine` binary can't run any E2E. No test validates that real-game moves have exactly 1 own overlap.  
**Fix:**
- Add `e2e` feature flag to `Cargo.toml`:

```toml
[features]
e2e = []
```

- Gate `tests/e2e.rs` behind `#[cfg(feature = "e2e)]`
- Add `piece_overlap_correct` test: runs actual game, captures filler's moves, replays each on grid, validates every move has exactly 1 own overlap + 0 opponent overlaps + no boundary violation
- Run via `cargo test --features e2e --test e2e`

---

### 5. Deterministic Tiebreak Specification + Tests

**Source:** glm.md — `scorer.rs`  
**Problem:** Deeppro's `choose_best_placement` says "first found" but doesn't specify iteration order. Non-deterministic across runs.  
**Fix:** Document and implement: **lower row first, then lower col** when scores are equal. Add test:

```rust
#[test]
fn tiebreak_by_row_then_col() {
    // Two placements with same score
    // Assert: picks lower row; if same row, lower col
}
```

---

## Medium-Impact Enrichments

### 6. Separate `output.rs` Module

**Source:** glm.md — `src/output.rs`  
**Deeppro gap:** Output formatting baked into `main.rs`, no isolated tests.  
**Fix:** Extract `format_move(placement) -> String` and `format_no_move() -> String` into `src/output.rs` with dedicated unit tests for format, trailing newline, and edge values.

---

### 7. Own-Territory Heatmap Value = -1

**Source:** glm.md — `heatmap.rs`  
**Deeppro gap:** Uses `i32::MAX` for all unreachable cells. Can't distinguish own territory from truly unreachable.  
**Fix:** Set own-territory cells to `-1` (unreachable for BFS, distinguishable from `i32::MAX`). Scorer skips `-1` cells instead of penalizing `MAX` cells. Cleaner scoring logic.

---

### 8. Multi-Turn Territory Growth Integration Test

**Source:** glm.md — `tests/multi_turn.rs`  
**Deeppro gap:** IT-6 checks state carry-over only, doesn't verify territory expands.  
**Fix:** Add test: simulate 3 turns, apply placements to grid, assert `own_count` monotonically increases.

---

### 9. `BufWriter` Around Stdout

**Source:** glm.md — `main.rs`  
**Deeppro gap:** Uses raw `stdout.lock()` + flush per turn.  
**Fix:** Wrap in `BufWriter::new(stdout.lock())`. Still flush per turn (game_engine reads one line), but buffered writes are more efficient for potential multi-line debug output.

---

### 10. `dev-dependencies` for Integration Tests

**Source:** glm.md — `Cargo.toml`  
**Deeppro gap:** No test helper crates.  
**Fix:** Add to `Cargo.toml`:

```toml
[dev-dependencies]
assert_cmd = "2"
predicates = "3"
tempfile = "3"
```

Enables subprocess-style integration tests with better assertions.

---

### 11. Error Handling Rule: Never Panic on User Input

**Source:** glm.md — Section 7  
**Deeppro gap:** Uses `?` operator but doesn't state the design constraint.  
**Fix:** Add explicit comment in `main.rs`:

```rust
// Never panic! or unwrap() on user input. Always return a move.
```

Junior dev guardrail.

---

## Low-Impact Enrichments

### 12. `.dockerignore`

**Source:** glm.md — Section 11  
**Deeppro gap:** No `.dockerignore`. Docker context includes `target/` and `.git/`.  
**Fix:** Add `.dockerignore`:

```
target/
.git/
```

Faster Docker builds, smaller context.

---

### 13. Alpine-Based Docker (Smaller Image)

**Source:** glm.md — Dockerfile  
**Deeppro gap:** Uses `debian:bookworm-slim` (~150MB).  
**Trade-off:** Alpine + musl = ~50MB but requires `musl-dev` and may have compatibility issues with game_engine binary (likely linked against glibc).  
**Recommendation:** Keep `debian:bookworm-slim` unless game_engine is confirmed musl-compatible. Worth noting as an option.

---

### 14. Split `strategy.rs` into `heatmap.rs` + `scorer.rs`

**Source:** glm.md — project structure  
**Deeppro gap:** Single `strategy.rs` with both heatmap generation and placement scoring.  
**Trade-off:** Smaller modules are easier to test/replace independently, but deeppro's `strategy.rs` is already small (~80 lines).  
**Recommendation:** Only split if strategy grows complex (e.g. adding tuning enhancements from item 2).

---

### 15. `Piece` Block Character Inconsistency

**Source:** glm.md uses `#` and `*`, deeppro uses `O` and `*`, gem uses `O` and `*`.  
**Note:** The actual game engine uses `O` and `*`. Deeppro is correct. No action needed — flagging for awareness only.

---

## Summary

| Priority | Item | Source |
|----------|------|--------|
| Critical | Negative-offset search | glm |
| High | Strategy tuning guide | glm |
| High | Benchmark harness | glm |
| High | E2E feature flag + replay validation | glm |
| High | Deterministic tiebreak | glm |
| Medium | Separate output module | glm |
| Medium | Own-territory = -1 in heatmap | glm |
| Medium | Multi-turn territory growth test | glm |
| Medium | BufWriter around stdout | glm |
| Medium | dev-dependencies | glm |
| Medium | Never-panic rule | glm |
| Low | .dockerignore | glm |
| Low | Alpine Docker option | glm |
| Low | Strategy module split | glm |
| Info | Block char consistency | n/a |

Gem.md offered no significant enrichments beyond what deeppro already covers.
