# Final Cross-Plan Analysis: Enrichments for Deeppro

Merged and deduplicated from:
- `docs/report/cross-plan-enrichments.md`
- `docs/report/gem_report.md`
- Manual comparison of `docs/plan/deeppro.md` vs `docs/plan/glm.md` vs `docs/plan/gem.md`

All 15 items from `cross-plan-enrichments.md` confirmed. One additional item (#16) surfaced during independent review. `gem_report.md` contained no unique items not already covered.

---

## Critical

### 1. Negative-Offset Placement Search

**Source:** glm.md (`validator.rs`), gem_report.md (item 1)  
**Problem:** Deeppro iterates `0..grid.rows` / `0..grid.cols` only. Game engine allows piece anchor at negative offsets if filled blocks still land in-bounds (common when piece has empty top/left padding).  
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

Also switch placement coordinates to `i32` (deeppro uses `usize` Point — can't represent negatives). Missing valid placements = lost games.

---

## High-Impact

### 2. Strategy Tuning Guide (Win-Rate Insurance)

**Source:** glm.md (Section 13)  
**Problem:** Deeppro mentions dual-heatmap as a bonus stub but gives no actionable roadmap if base strategy falls below 80% win rate.  
**Fix:** Add concrete enhancement catalog:

| Enhancement | Description | Implementation |
|---|---|---|
| Edge Weighting | Bonus for cells near grid edges; traps opponent | `edge_bonus(row, col, rows, cols)` → -5 / -2 / 0 by dist-to-edge |
| Territory Connectivity | Prefer placements keeping own territory connected | Penalty for creating isolated regions |
| Opponent Blocking | Extend along boundary when adjacent to opponent | Score bonus for cells neighboring opponent |
| Center Start Bias | First move favors center + opponent direction | Extra weight on center-proximity for turn 1 |
| Piece Size Bonus | Prefer placements covering more empty cells | `score -= empty_cells_under_piece * 2` |

---

### 3. Benchmark Harness (Perf Regression Gate)

**Source:** glm.md (`benches/turn_benchmark.rs`), gem_report.md (item 3)  
**Problem:** Deeppro says "algorithm terminates < 1s" for 100×100 grids but never enforces it.  
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

**Source:** glm.md (`tests/e2e.rs`)  
**Problem:** Deeppro's E2E is bash-only. CI without `game_engine` binary can't run any E2E. No test validates that real-game moves have exactly 1 own overlap.  
**Fix:**
- Add `e2e` feature flag to `Cargo.toml`:

```toml
[features]
e2e = []
```

- Gate `tests/e2e.rs` behind `#[cfg(feature = "e2e")]`
- Add `piece_overlap_correct` test: runs actual game, captures filler's moves, replays each on grid, validates every move has exactly 1 own overlap + 0 opponent overlaps + no boundary violation
- Run: `cargo test --features e2e --test e2e`

---

### 5. Deterministic Tiebreak Specification + Tests

**Source:** glm.md (`scorer.rs`), gem_report.md (item 5)  
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

## Medium-Impact

### 6. Separate `output.rs` Module

**Source:** glm.md  
**Gap:** Output formatting baked into `main.rs` in deeppro. No isolated tests.  
**Fix:** Extract `format_move(Placement) -> String` and `format_no_move() -> String` into `src/output.rs` with dedicated unit tests for format, trailing newline, and edge values (`"0 0\n"`, `"99 14\n"`).

---

### 7. Own-Territory Heatmap Value = -1

**Source:** glm.md (`heatmap.rs`)  
**Gap:** Deeppro uses `i32::MAX` for all unreachable cells. Can't distinguish own territory from truly unreachable.  
**Fix:** Set own-territory cells to `-1` (unvisited by BFS, distinguishable from `i32::MAX`). Scorer skips `-1` cells instead of penalizing `MAX`. Cleaner scoring with fewer edge cases.

---

### 8. Multi-Turn Territory Growth Integration Test

**Source:** glm.md (`tests/multi_turn.rs`)  
**Gap:** Deeppro's IT-6 checks state carry-over only, doesn't verify territory expands.  
**Fix:** Add test: simulate 3 turns, apply placements to grid, assert `own_count` monotonically increases. Catches regression in scoring that fails to extend territory.

---

### 9. `BufWriter` Around Stdout

**Source:** glm.md (`main.rs`)  
**Gap:** Deeppro uses raw `stdout.lock()` + flush per turn.  
**Fix:** Wrap in `BufWriter::new(stdout.lock())`. Still flush per turn (game_engine reads one line), but buffered intermediate writes are more efficient for potential multi-line debug/log output.

---

### 10. `dev-dependencies` for Integration Tests

**Source:** glm.md (`Cargo.toml`), gem_report.md (item 4)  
**Gap:** Deeppro lists zero dev-dependencies.  
**Fix:** Add:

```toml
[dev-dependencies]
assert_cmd = "2"
predicates = "3"
tempfile = "3"
```

Enables subprocess-style integration tests (`Command::cargo_bin("filler")`) with richer assertions.

---

### 11. Error Handling Rule: Never Panic on User Input

**Source:** glm.md (Section 7)  
**Gap:** Deeppro uses `?` operator but never states the design constraint.  
**Fix:** Add explicit guardrail:

```rust
// Never panic! or unwrap() on user input. Always return a move.
```

Also codify centralized rules:
- `parse_grid` EOF → clean exit
- `parse_piece` EOF → clean exit
- Malformed data → print `"0 0\n"` and continue
- `parse_turn` must never crash on garbage input

---

## Low-Impact

### 12. `.dockerignore`

**Source:** glm.md (Section 11)  
**Gap:** Deeppro omits `.dockerignore`. Docker build context includes `target/` and `.git/`.  
**Fix:**

```
target/
.git/
```

Smaller context, faster builds.

---

### 13. Alpine-Based Docker (Smaller Image)

**Source:** glm.md (Dockerfile)  
**Gap:** Deeppro uses `debian:bookworm-slim` (~150MB).  
**Trade-off:** Alpine + musl ≈ 50MB but requires `musl-dev`. The `game_engine` binary is likely glibc-linked — Alpine will break unless engine is recompiled or confirmed musl-compatible.  
**Recommendation:** Keep `debian:bookworm-slim`. Note Alpine as an option if image size becomes a requirement.

---

### 14. Split `strategy.rs` into `heatmap.rs` + `scorer.rs`

**Source:** glm.md (project structure)  
**Gap:** Deeppro has single `strategy.rs` with both heatmap generation and placement scoring.  
**Trade-off:** Smaller modules are easier to test/replace independently, but deeppro's `strategy.rs` is already compact (~80 lines).  
**Recommendation:** Only split if strategy grows beyond ~150 lines (e.g. when adding tuning enhancements from item 2).

---

## Informational

### 15. Piece Block Character Consistency

**Source:** cross-check across all plans  
**Note:** glm.md uses `#` and `*` as filled chars; deeppro and gem use `O` and `*`. Actual game engine uses `O` and `*`. Deeppro is correct. Flagged for awareness if copy-pasting from glm parser examples.

---

### 16. Byte-Level Grid Storage (Architectural Variant)

**Source:** glm.md (`models.rs`) — surfaced during independent review  
**Note:** glm.md stores grid cells as `u8` bytes (`b'O'`, `b'X'`, `b'.'`, etc.) instead of deeppro's `Cell` enum. Advantages: single-byte storage (cache-friendly), branchless cell comparisons (`cell == char_up || cell == char_lo`), zero allocation per cell. Trade-off: less type-safe, implicit char-to-meaning mapping.  
**Recommendation:** Mention as performance optimization variant in strategy tuning phase. Not needed for base implementation.

---

## Summary Table

| # | Priority | Item | Source(s) |
|---|----------|------|-----------|
| 1 | Critical | Negative-offset placement search | glm, gem_report |
| 2 | High | Strategy tuning guide | glm |
| 3 | High | Benchmark harness | glm, gem_report |
| 4 | High | E2E feature flag + replay validation | glm |
| 5 | High | Deterministic tiebreak | glm, gem_report |
| 6 | Medium | Separate output.rs module | glm |
| 7 | Medium | Own-territory = -1 in heatmap | glm |
| 8 | Medium | Multi-turn territory growth test | glm |
| 9 | Medium | BufWriter around stdout | glm |
| 10 | Medium | dev-dependencies | glm, gem_report |
| 11 | Medium | Never-panic error handling rules | glm |
| 12 | Low | .dockerignore | glm |
| 13 | Low | Alpine Docker option | glm |
| 14 | Low | Strategy module split | glm |
| 15 | Info | Block char consistency | all |
| 16 | Info | Byte-level grid storage variant | glm, manual |

`gem_report.md` items mapped: 1→#1, 2→ (already in deeppro), 3→#3, 4→#10, 5→#5. No unique additions.