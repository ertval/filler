# Audit Guide — Filler

Step-by-step for each question in [requirements/audit.md](../requirements/audit.md). Covers manual test procedure, expected outcome, and codebase references.

---

## Prerequisites

```bash
# Build the binary — release mode for optimised (LTO) binary; debug too slow for timeouts
cargo build --release

# Verify game_engine and robots exist — engine is the referee binary, robots are opponent AIs
ls -l engine-maps-robots/linux_game_engine engine-maps-robots/linux_robots/  # from project root
```

---

## Functional

### Q1 — Docker image

**Run:**
```bash
make docker-build                          # invokes docker build -t filler .
# or
docker build -t filler .                   # multi-stage: stage 1 compiles, stage 2 is slim runtime
```

**Check:**
```bash
docker image inspect filler > /dev/null 2>&1 && echo "exists" || echo "missing"
# ^ suppress stdout/stderr, only care about exit code; "exists" means image is present
```

**References:**
- `Dockerfile` — multi-stage Debian Bookworm Slim build
- `e2e/run_audit_suite.sh:21-25` — automated check

---

### Q2 — Project runs correctly

**Run:**
```bash
engine-maps-robots/linux_game_engine -f engine-maps-robots/maps/map01 -p1 ./target/release/filler -p2 engine-maps-robots/linux_robots/bender
#   -f <map>   path to Anfield file
#   -p1 <path> path to P1 binary (our filler)
#   -p2 <path> path to P2 binary (reference robot bender)
```

**Check:** No crash, no segfault, no panic. Game completes with a winner.

**References:**
- `src/main.rs:10-11` — panic catch-all, EOF exits cleanly
- `src/main.rs:23` — `catch_unwind` wraps every turn
- `src/main.rs:53-55` — EOF triggers clean break, not error
- `src/main.rs:60-64` — panic recovery outputs `0 0` fallback
- `src/parser.rs:5` — parser errors caught in main loop, no panics
- `e2e/run_audit_suite.sh:38-43` — smoke test vs bender

---

### Q3 — 1-cell overlap rule

**Run:**
```bash
# Manual: place piece exactly 1 cell on own territory, 0 on opponent
engine-maps-robots/linux_game_engine -f engine-maps-robots/maps/map00 -p1 ./target/release/filler -p2 engine-maps-robots/linux_robots/wall_e
#   map00 has symmetric start positions; wall_e is defensive, gives many turns to observe placement
```

**Check:** Bot only places pieces with exactly 1 own-cell overlap and 0 opponent overlap. Invalid moves cause game engine to reject (engine prints error and skips that player's turn).

**References:**
- `src/validator.rs:20-46` — `is_valid_placement()` enforces `own_count == 1 && opp_count == 0`
- `src/validator.rs:38-42` — counts own vs opponent overlap per cell
- `tests/validator_tests.rs:40-55` — `test_is_valid_placement_basic`
- `tests/validator_tests.rs:103-117` — `test_e8_opponent_chars_rejected`
- `tests/validator_tests.rs:120-135` — `test_e9_own_chars_overlap`
- `tests/integration_tests.rs:11-36` — IT-1: P1 single valid turn, only (1,1) passes
- `tests/integration_tests.rs:38-63` — IT-2: P2 single valid turn, only (3,3) passes
- `tests/e2e.rs` — live game replay asserts no invalid moves

---

### Q4 — Win-rate vs wall_e on map00 (4/5)

**Run:**
```bash
ENGINE=engine-maps-robots/linux_game_engine
MAP=engine-maps-robots/maps/map00
ROBOT=engine-maps-robots/linux_robots/wall_e
PLAYER=./target/release/filler

# As p1 (5 games) — -q is quiet mode, engine outputs only win/loss line
for i in 1 2 3 4 5; do
  $ENGINE -f $MAP -p1 $PLAYER -p2 $ROBOT -q 2>&1 | grep -c "Player1 won"
done

# As p2 (5 games) — swap -p1 and -p2 to test both starting positions
for i in 1 2 3 4 5; do
  $ENGINE -f $MAP -p1 $ROBOT -p2 $PLAYER -q 2>&1 | grep -c "Player2 won"
done
```

> Note: Engine outputs `Player1 won` / `Player2 won` (no space). Use `"Player[12] won"` pattern.

**Check:** Student wins ≥8/10 total (≥4/5 of relevant games). P1 + P2 combined to prevent map-side bias.

**References:**
- `e2e/run_audit_suite.sh:93` — `check_winrate map00 student wall_e 8`
- `e2e/run_audit_suite.sh:59-91` — `check_winrate` implementation: 5 as p1 + 5 as p2
- `src/strategy.rs:8-90` — BFS heatmap drives aggressive expansion

---

### Q5 — Win-rate vs h2_d2 on map01 (4/5)

**Run:** Same as Q4, substituting `engine-maps-robots/maps/map01` and `engine-maps-robots/linux_robots/h2_d2`.
<!-- h2_d2 is medium-strength; map01 layout has asymmetric start positions -->

**Reference:** `e2e/run_audit_suite.sh:94`

---

### Q6 — Win-rate vs bender on map02 (4/5)

**Run:** Same as Q4, substituting `engine-maps-robots/maps/map02` and `engine-maps-robots/linux_robots/bender`.
<!-- bender is strongest of the three required robots; map02 is larger with more distance -->

**Reference:** `e2e/run_audit_suite.sh:95`, `src/strategy.rs:5`

---

## Unit Tests

### Q7 — All tests pass

**Run:**
```bash
cargo test --lib          # library (non-e2e) tests only; fast, no game_engine dependency
```

**Check:** 14+ tests green.

**References:**
- `src/parser.rs:4` — input parsing tests
- `src/validator.rs:3-5` — validation tests
- `src/strategy.rs:142-181` — heatmap + tiebreak tests
- `src/output.rs:3` — formatting tests
- `src/visualizer.rs:5` — visualizer tests
- `tests/parser_tests.rs` — 10 tests: player line, anfield, piece, blank lines, EOF
- `tests/validator_tests.rs` — 8 tests: bounds, placement, zero-block, filled grid, opponent chars
- `tests/strategy_tests.rs` — deterministic tiebreaker tests
- `tests/multi_turn.rs` — territory growth over 3 turns
- `tests/integration_tests.rs` — 6 integration tests (IT-1 through IT-6)
- `tests/e2e.rs` — live game replay validation (requires `--features e2e`)

---

### Q8 — Input Parsing tests

**Run:**
```bash
cargo test --lib parser::tests   # --lib filters to unit tests only; "parser::tests" is the #[cfg(test)] module name
```

**Check:** Tests verify Anfield dimensions and piece shape parsed from stdin.

**References:**
- `src/parser.rs:23-32` — `parse_player_line()`: regex-free extraction of P1/P2
- `src/parser.rs:53-86` — `parse_anfield()`: reads header, column header, row data
- `src/parser.rs:104-122` — `parse_piece()`: reads header, block coordinates
- `tests/parser_tests.rs:6-15` — valid player line parsing
- `tests/parser_tests.rs:32-47` — valid anfield parsing with row/col mapping
- `tests/parser_tests.rs:88-95` — valid piece parsing

---

### Q9 — Placement Validation tests

**Run:**
```bash
cargo test --lib validator::tests   # runs only the tests module inside src/validator.rs
```

**Check:** Tests verify:
- Move rejected if it overlaps opponent cells
- Move rejected if it overlaps 2+ own cells
- Move accepted if exactly 1 own, 0 opponent
<!-- The core game rule: is_valid_placement() enforces own_count == 1 && opp_count == 0 -->

**References:**
- `src/validator.rs:20-46` — `is_valid_placement()`: own_count == 1, opp_count == 0
- `tests/validator_tests.rs:103-117` — `test_e8`: placement on opponent `$` or `s` rejected
- `tests/validator_tests.rs:120-135` — `test_e9`: placement on own `@` or `a` accepted
- `tests/validator_tests.rs:40-55` — basic 1-own, 0-opponent acceptance

---

### Q10 — Boundary Detection tests

**Run:**
```bash
cargo test --lib -- is_in_bounds   # `--` separates cargo args from test binary args; filters test names containing "is_in_bounds"
```

**Check:** Pieces never placed partially outside grid.

**References:**
- `src/validator.rs:8-17` — `is_in_bounds()`: checks every block row/col against grid dimensions
- `src/validator.rs:49-63` — `find_valid_placements()`: scan range includes negative offsets
- `tests/validator_tests.rs:23-37` — `test_is_in_bounds`: corner cases (row 4, col 4 out; -1 row out)
- `tests/validator_tests.rs:138-158` — `test_negative_offset_placements`: piece anchors off-grid but block lands in-bounds
- `tests/integration_tests.rs:118-138` — IT-5: 2×2 piece at (4,4) rejected as out-of-bounds

---

## Basic

### Q11 — Good practices

Code follows Rust standard practices: `cargo fmt`, `cargo clippy`, meaningful names, no unsafe, documented modules.
<!-- Audit expects idiomatic Rust: formatting, lint-clean, no `unsafe` blocks, module-level docs -->

### Q12 — Test file exists

All tests in `tests/` directory with `#[test]` annotations.
<!-- Verifies separate test files exist (not just inline #[cfg(test)] modules); tests/ directory has 5+ files -->

### Q13 — Tests check each case

Tests cover: valid parse, malformed input, EOF, bounds, placement rules, opponent chars, own chars, zero-block pieces, filled grids, piece > grid, edge weighting, opponent blocking, tiebreak, multi-turn growth, full pipeline integration, live game replay.
<!-- Broad coverage = examiner checks that edge cases aren't ignored: malformed input, oversized pieces, negative offsets, empty grids -->

---

## Bonus

### Visualizer

**Run:**
```bash
cargo test --lib visualizer::    # runs all tests in the visualizer module (file parsing, rendering, frame playback)
```

**Reference:** `src/visualizer.rs` — 212 lines, ANSI colored terminal replay, 8+ tests.
<!-- Bonus: parses game engine log output into frames, replays with ANSI color per player, uses \x1b[2J clear -->

### Win-rate vs terminator

**Run:** Same as Q4 format, any map, `engine-maps-robots/linux_robots/terminator`.
<!-- terminator is the hardest bonus robot; requires aggressive heatmap + edge-weighting strategy -->

**Reference:** `e2e/run_audit_suite.sh:137-138`

---

## Quick Reference: All Test Commands

```bash
make build              # Release binary — single codegen unit, LTO enabled
make test-lib           # Unit tests — lib only (parser, validator, strategy, output, visualizer)
make test               # All tests — includes integration_tests.rs and multi_turn.rs
make test-e2e           # E2E replay — requires --features e2e flag + game_engine binary present
make bench              # Performance benchmark — asserts <500ms on 100×100 grid
make audit              # Full automated audit suite — Docker, crash-free, win-rates, unit tests
make docker-build       # Docker image — multi-stage: builder → slim runtime
make play MAP=map01 P1=filler P2=bender VIS=0   # build + run one game with optional visualizer
```
