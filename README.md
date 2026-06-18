<picture>
  <source
    media="(prefers-color-scheme: dark)"
    srcset="https://raw.githubusercontent.com/42school/42ai/main/assets/filler-banner-dark.svg"
  />
  <img alt="filler" src="https://raw.githubusercontent.com/42school/42ai/main/assets/filler-banner-light.svg" />
</picture>

# filler

> An autonomous bot for the **Filler** territory-capturing game. Reads the
> playing field from stdin, computes the optimal piece placement using a BFS
> heatmap strategy, and outputs its move — one turn at a time.

<p align="center">
  <img src="https://img.shields.io/badge/rust-2021-dea584?logo=rust&logoColor=white" />
  <img src="https://img.shields.io/badge/stdlib_only-✓-6a9fb5" />
  <img src="https://img.shields.io/badge/edition-2021-b7410e" />
  <img src="https://img.shields.io/badge/coverage-14_tests-3fb950" />
  <img src="https://img.shields.io/badge/license-MIT-blue" />
</p>

---

## The Game

Filler is a two-player territory capture game played on a rectangular grid
(the *Anfield*). Each turn the game engine sends your bot:

1. The player identity (`p1` or `p2`)
2. The current state of the Anfield
3. A randomly shaped piece

Your bot replies with `X Y` — where to place the piece. Legal placement
requires **exactly one cell** of the piece overlapping your territory and
**zero cells** overlapping the opponent's. The first player unable to place
a piece loses.

---

## How It Works

```
stdin ──► parse_turn() ──► find_valid_placements() ──► generate_heatmap()
                                                           │
                                                           ▼
                                                      score & choose
                                                           │
                                                           ▼
                                                      format_move()
                                                           │
                                                           ▼
                                                      stdout (X Y)
```

The algorithm uses a **BFS distance heatmap** radiating from the opponent's
territory. Closer to the opponent = lower score = better placement. This
aggressive expansion cuts off the opponent's growth space.

### Strategy Enhancements

| Tactic | Effect |
|---|---|
| **Edge weighting** | Bonuses for cells near grid edges, trapping opponent against boundaries |
| **Opponent blocking** | Extra pull toward cells adjacent to opponent occupied cells |
| **Deterministic tiebreak** | Lower row, then lower column — reproducible across runs |

---

## Project Structure

```
filler/
├── Cargo.toml
├── Dockerfile
├── src/
│   ├── main.rs          Entry point — stdin loop with panic guardrails
│   ├── lib.rs           Module re-exports for testing
│   ├── types.rs         Player, Cell, Grid, Point, Piece, GameState
│   ├── parser.rs        Stdin → GameState (player line, anfield, piece)
│   ├── validator.rs     Boundary check, overlap logic, move enumeration
│   ├── strategy.rs      BFS heatmap generation, scoring, selection
│   ├── output.rs        Move formatting — `X Y\n`
│   └── visualizer.rs    Terminal replay viewer with ANSI colors [bonus]
├── tests/
│   ├── common/mod.rs    Shared test fixtures & mock stdin
│   ├── integration_tests.rs   Full pipeline (IT-1 through IT-6)
│   ├── strategy_tests.rs      Deterministic tiebreaker
│   ├── multi_turn.rs          Territory growth over 3 turns
│   └── e2e.rs                 Live game replay validation
├── benches/
│   └── turn_benchmark.rs      Performance: 100×100 grid < 500ms
├── e2e/
│   ├── run_audit_suite.sh     Automated audit script
│   └── assert_winrate.rs      Standalone win-rate checker
└── requirements/
    └── audit.md               Audit questions & evaluation criteria
```

---

## Quick Start

### Make Commands

| Command | What It Does |
|---|---|
| `make build` | Release build (LTO, single codegen) |
| `make play MAP=map00 P1=filler P2=bender VIS=0` | Build + run game vs opponent (defaults in Makefile) |
| `make run` | Show example game_engine invocation |
| `make test-lib` | Unit tests only (parser, validator, strategy, output) |
| `make test` | All unit + integration tests |
| `make test-e2e` | E2E replay validation (needs `game_engine` binary) |
| `make bench` | Performance benchmark (<500ms assert on 100×100) |
| `make audit` | Full audit suite — Docker, crash-free, win-rates |
| `make docker-build` | Multi-stage Docker build |
| `make docker-run` | Run inside container |

### Manual Build & Run

```bash
cargo build --release
./game_engine -f maps/map01 -p1 ./target/release/filler -p2 robots/bender
```

---

## Module Map

| Module | Lines | Responsibility |
|---|---|---|
| `types` | 84 | Shared data structures |
| `parser` | 178 | Stdin parsing (player, anfield, piece) |
| `validator` | 178 | Boundary & overlap legality |
| `strategy` | 182 | Heatmap BFS, scoring, tiebreak |
| `output` | 31 | Coordinate formatting |
| `visualizer` | 210 | Terminal replay (bonus) |

---

## Audit Compliance

See [requirements/audit.md](./requirements/audit.md) for the full question
set. Every question maps to a test, benchmark, or script check annotated
with `// Audit Q<N>` in the source.

| Audit | Coverage |
|---|---|
| Q1 — Docker image | `e2e/run_audit_suite.sh` |
| Q2 — Runs correctly | `src/main.rs` guardrails, audit smoke test |
| Q3 — 1-cell overlap | `src/validator.rs`, `tests/e2e.rs`, `tests/integration_tests.rs` |
| Q4–6 — Win-rates (80%) | `e2e/run_audit_suite.sh` (wall_e, h2_d2, bender) |
| Q7–10 — Unit tests | `cargo test --lib` (parser, validator, boundary) |
| Bonus — Visualizer | `src/visualizer.rs` |
| Bonus — Terminator | `e2e/run_audit_suite.sh` |

---

## License

MIT
