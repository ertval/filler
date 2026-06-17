# DeepPro Methodology & Best Practices

Methodology and coding standards for implementing the `filler` Rust project based on `docs/plan/deeppro.md`.

## 1. Test-Driven Development (TDD)
- **Cycle**: Always follow the classic TDD workflow: Write a failing unit/integration/E2E test (RED) → Implement the minimum code to make it pass (GREEN) → Refactor (REFACTOR) → Repeat.
- **Test Locations**:
  - Unit tests in `tests/parser_tests.rs`, `tests/validator_tests.rs`, `tests/strategy_tests.rs`, `tests/output_tests.rs`.
  - Integration tests in `tests/integration_tests.rs` and `tests/multi_turn.rs`.
  - E2E replay checks in `tests/e2e.rs` (gated by `#[cfg(feature = "e2e")]`).
  - Performance benchmarks in `benches/turn_benchmark.rs`.

## 2. Robust Error Handling (Never Panic Guardrail)
- **Never Panic**: Under no circumstances should the program panic, crash, or use `unwrap()` / `expect()` on external input (stdin or game engine input).
- **Graceful Fallback**: If input is malformed, log the error to `stderr`, output the fallback move `0 0\n`, and continue processing subsequent turns.
- **EOF Exit**: Exit cleanly with `Ok(())` when the input stream reaches EOF.

## 3. Coordinate System & Offsets
- **Formatting**: The game engine uses `X Y\n` output format, where `X` is the column and `Y` is the row.
- **Internal Point**: Use `Point { row, col }` internally. Convert to `X Y` on serialization in `src/output.rs`.
- **Negative Placements**: Placements must support negative coordinate offsets (using signed `i32` or `isize`) because the game engine allows placing a piece at negative coordinates if the filled blocks remain within grid boundaries (due to empty top/left piece padding).

## 4. Input/Output (IO) Protocol
- **Blocking Read**: `BufRead::read_line` blocks to wait for the game engine.
- **Flushing**: Flush `stdout` immediately after every turn's response using `writer.flush()`. The game engine reads line-by-line and will hang if stdout is not flushed.
- **Buffering**: Use `BufWriter` around `stdout.lock()` to optimize intermediate writes.
- **Streams**: Send errors/debug logs to `stderr` and only valid/fallback moves to `stdout`.

## 5. Decision Performance Target
- **Timeout Protection**: The game engine enforces strict turn timeout limits.
- **Goal**: Full turn decision (parse → heatmap → score → choose) must take **less than 500ms** on a standard 100x100 grid with a 20x20 piece. Verify via `cargo bench` and `benches/turn_benchmark.rs`.

## 6. Strategy & Deterministic Selection
- **Manhattan Distance Heatmap**: Generate a distance BFS heatmap starting from the opponent's cells (0 distance). Set own cells to `-1` to prevent scoring penalties.
- **Minimum Distance Scoring**: Select the placement that minimizes the sum of heatmap cell values under the piece blocks (aggressive strategy targeting opponent cutoff).
- **Deterministic Tie-Breaking**: If multiple placements share the same minimal score, always choose the one with the **lowest row (top-most)**, and if rows are equal, the **lowest column (left-most)**.
